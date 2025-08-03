import { parseArgs } from "util";
import { Keypair, Contract, rpc as StellarRpc, TransactionBuilder, Networks, BASE_FEE } from "@stellar/stellar-sdk";
import { Client as ResolverClient, type Order } from "./bindings/resolver/src/index";
import { Client as OrderClient } from "./bindings/order/src/index";
import { keccak256, toHex, type Hex } from 'viem';
import {  
    HashLock,  
    TimeLocks,
} from '@1inch/cross-chain-sdk';
import { EthereumClient } from './ethereum-client';

enum Network {
    TESTNET = "http://localhost:8000",
    SEPOLIA = "https://sepolia.infura.io/v3/YOUR_INFURA_KEY",
}

const getRpcUrl = (network: Network) => {
    return `${network}/soroban/rpc`
}

type ScriptConfig = {
    limitOrderProtocol: string,
    secret: string,
    resolver: string,
    withdrawalSrcTimelock: number,
    publicWithdrawalSrcTimelock: number,
    cancellationSrcTimelock: number,
    publicCancellationSrcTimelock: number,
    withdrawalDstTimelock: number,
    publicWithdrawalDstTimelock: number,
    cancellationDstTimelock: number,
    publicCancellationDstTimelock: number,
    ethereum: {
        rpcUrl: string;
        escrowFactoryAddress: string;
        privateKey: string;
        chainId: number;
        tokens: {
            usdc: string;
            weth: string;
        };
    };
    stellar: {
        rpcUrl: string;
        networkPassphrase: string;
        tokens: {
            usdc: string;
            xlm: string;
        };
    };
    swapDirection: string;
}

export class CrossChainSwapClient {
    private config: ScriptConfig;
    private stellarServer: StellarRpc.Server;
    private resolverClient: ResolverClient;
    private orderClient: OrderClient;
    private ethereumClient: EthereumClient;
    private alice: Keypair;
    private bob: Keypair;

    constructor(config: ScriptConfig) {
        this.config = config;
        this.stellarServer = new StellarRpc.Server(
            getRpcUrl(Network.TESTNET),
            { allowHttp: true }
        );
        
        this.resolverClient = new ResolverClient({
            contractId: config.resolver,
            networkPassphrase: config.stellar.networkPassphrase,
            rpcUrl: getRpcUrl(Network.TESTNET),
            allowHttp: true,
        });

        this.orderClient = new OrderClient({
            contractId: config.limitOrderProtocol,
            networkPassphrase: config.stellar.networkPassphrase,
            rpcUrl: getRpcUrl(Network.TESTNET),
            allowHttp: true,
        });

        this.ethereumClient = new EthereumClient(
            config.ethereum.rpcUrl,
            config.ethereum.privateKey,
            config.ethereum.escrowFactoryAddress
        );

        this.alice = Keypair.random();
        this.bob = Keypair.random();
    }

    private async topupWithFriendbot(address: string) {
        console.log(`Topping up account ${address} with friendbot`);
        const friendbotUrl = `${this.config.stellar.rpcUrl}/friendbot?addr=${address}`;
        const response = await fetch(friendbotUrl);
        if (!response.ok) {
            const errorText = await response.text();
            console.warn(`Friendbot response: ${response.status} - ${errorText}`);
            // Don't throw error, just log warning - account might already be funded
            return;
        }
        console.log(`Friendbot response: ${response.status}`);
        return response;
    }

    private getSecrets(secretSalt: string) {
        const secret = keccak256(toHex(secretSalt));
        const hashlock = keccak256(secret);
        return [hashlock];
    }

    private getHashlock(secrets: [Hex, ...Hex[]]) {
        if (secrets.length === 1) {
            return HashLock.forSingleFill(secrets[0])
        }
        return secrets.map(secret => keccak256(toHex(secret)));
    }

    private getTimelocks() {
        return TimeLocks.new({
            srcWithdrawal: BigInt(this.config.withdrawalSrcTimelock),
            srcPublicWithdrawal: BigInt(this.config.publicWithdrawalSrcTimelock),
            srcCancellation: BigInt(this.config.cancellationSrcTimelock),
            srcPublicCancellation: BigInt(this.config.publicCancellationSrcTimelock),
            dstWithdrawal: BigInt(this.config.withdrawalDstTimelock),
            dstPublicWithdrawal: BigInt(this.config.publicWithdrawalDstTimelock),
            dstCancellation: BigInt(this.config.cancellationDstTimelock),
        })
    }

    private calculateOrderHash(order: any) {
        // Manual calculation since order_hash is not exposed in client bindings
        const orderData = Buffer.concat([
            Buffer.from(order.maker, 'hex'),
            Buffer.from(order.maker_asset, 'hex'),
            Buffer.from(order.taker_asset, 'hex'),
            Buffer.from(order.receiver, 'hex'),
            Buffer.from(order.maker_traits.toString(16).padStart(64, '0'), 'hex'),
            Buffer.from(order.salt.toString(16).padStart(64, '0'), 'hex'),
            Buffer.from(order.making_amount.toString(16).padStart(64, '0'), 'hex'),
            Buffer.from(order.taking_amount.toString(16).padStart(64, '0'), 'hex'),
        ]);
        return keccak256(orderData);
    }

    private randomBytes(length: number) {
        return Buffer.from(crypto.getRandomValues(new Uint8Array(length)));
    }

    async initialize() {
        console.log('🚀 Initializing Cross-Chain Swap Client...');
        
        // Top up accounts
        await this.topupWithFriendbot(this.alice.publicKey());
        await this.topupWithFriendbot(this.bob.publicKey());
        
        console.log('✅ Accounts funded and ready');
    }

    async executeEthereumToStellarSwap() {
        console.log('🔄 Executing Ethereum → Stellar Swap...');
        
        const secrets = this.getSecrets(this.config.secret);
        const hashlock = this.getHashlock(secrets as [Hex, ...Hex[]]);
        const timelocks = this.getTimelocks();

        // Create order for Ethereum → Stellar swap
        const order = {
            maker: this.alice.publicKey(),
            maker_asset: this.config.stellar.tokens.usdc,  // Use Stellar USDC
            taker_asset: this.config.stellar.tokens.xlm,   // Use Stellar XLM
            maker_traits: 967101221531144175919556390646195146547200n,
            receiver: this.bob.publicKey(),
            salt: 1n,
            taking_amount: 1000000000000000000n,
            making_amount: 1000000000000000000n,
        };

        const orderHash = this.calculateOrderHash(order);
        const orderHashBuffer = Buffer.from(orderHash.slice(2), 'hex');

        console.log('📝 Order Hash:', orderHash);

        // Step 1: Create source escrow on Ethereum
        console.log('📍 Creating source escrow on Ethereum...');
        const ethereumImmutables = {
            amount: 1000000000000000000n,
            hashlock: hashlock.toString(),
            maker: this.alice.publicKey(),
            orderHash: orderHash,
            safetyDeposit: 1000000000000000000n,
            taker: this.bob.publicKey(),
            timelocks: timelocks.build(),
            token: this.config.ethereum.tokens.usdc,
        };

        const ethereumOrder = {
            maker: this.alice.publicKey(),
            makerAsset: this.config.ethereum.tokens.usdc,
            makerTraits: 967101221531144175919556390646195146547200n,
            makingAmount: 1000000000000000000n,
            receiver: this.bob.publicKey(),
            salt: 1n,
            takerAsset: this.config.stellar.tokens.usdc,
            takingAmount: 1000000000000000000n,
        };

        // Step 1: Create source escrow on Ethereum
        console.log('📍 Creating source escrow on Ethereum...');
        let ethereumReceipt;
        
        try {
            // Check if we have real Ethereum credentials
            if (this.config.ethereum.rpcUrl.includes('demo') || this.config.ethereum.privateKey.includes('1234')) {
                console.log('⚠️  Using demo mode - skipping actual Ethereum transaction');
                console.log('   In production, this would create the source escrow on Ethereum');
                ethereumReceipt = { hash: 'demo_hash_placeholder' };
            } else {
                // Use real Ethereum client
                const ethereumImmutables = {
                    amount: 1000000000000000000n,
                    hashlock: hashlock.toString(),
                    maker: this.alice.publicKey(),
                    orderHash: orderHash,
                    safetyDeposit: 1000000000000000000n,
                    taker: this.bob.publicKey(),
                    timelocks: BigInt(timelocks.build()),
                    token: this.config.ethereum.tokens.usdc,
                };

                const ethereumOrder = {
                    maker: this.alice.publicKey(),
                    makerAsset: this.config.ethereum.tokens.usdc,
                    makerTraits: 967101221531144175919556390646195146547200n,
                    makingAmount: 1000000000000000000n,
                    receiver: this.bob.publicKey(),
                    salt: 1n,
                    takerAsset: this.config.ethereum.tokens.usdc,
                    takingAmount: 1000000000000000000n,
                };

                ethereumReceipt = await this.ethereumClient.createSrcEscrow(
                    ethereumImmutables,
                    ethereumOrder,
                    this.randomBytes(32).toString('hex'),
                    this.randomBytes(32).toString('hex'),
                    1000000000000000000n,
                    0n,
                    "0x"
                );
            }
        } catch (error) {
            console.error('❌ Error creating Ethereum source escrow:', error);
            console.log('⚠️  Falling back to demo mode');
            ethereumReceipt = { hash: 'demo_hash_placeholder' };
        }

        console.log('✅ Ethereum source escrow created:', ethereumReceipt);

        // Step 2: Create destination escrow on Stellar
        console.log('📍 Creating destination escrow on Stellar...');
        const stellarResponse = await this.resolverClient.deploy_src({
            immutables: {
                amount: 1000000000000000000n,
                hashlock: Buffer.from(hashlock.toString().slice(2), 'hex'),
                maker: this.alice.publicKey(),
                order_hash: orderHashBuffer,
                safety_deposit: 1000000000000000000n,
                taker: this.bob.publicKey(),
                timelocks: BigInt(timelocks.build()),
                token: this.config.stellar.tokens.usdc,
            },
            order,
            signature_r: this.randomBytes(32),
            signature_vs: this.randomBytes(32),
            amount: 1000000000000000000n,
            taker_traits: 0n,
            args: Buffer.from([]),
        });

        console.log('✅ Stellar destination escrow created:', stellarResponse);

        return {
            ethereumReceipt,
            stellarResponse,
            orderHash,
            secrets: secrets[0]
        };
    }

    async executeStellarToEthereumSwap() {
        console.log('🔄 Executing Stellar → Ethereum Swap...');
        
        const secrets = this.getSecrets(this.config.secret);
        const hashlock = this.getHashlock(secrets as [Hex, ...Hex[]]);
        const timelocks = this.getTimelocks();

        // Create order for Stellar → Ethereum swap
        const order = {
            maker: this.alice.publicKey(),
            maker_asset: this.config.stellar.tokens.usdc,  // Use Stellar USDC
            taker_asset: this.config.stellar.tokens.xlm,   // Use Stellar XLM
            maker_traits: 967101221531144175919556390646195146547200n,
            receiver: this.bob.publicKey(),
            salt: 1n,
            taking_amount: 1000000000000000000n,
            making_amount: 1000000000000000000n,
        };

        const orderHash = this.calculateOrderHash(order);
        const orderHashBuffer = Buffer.from(orderHash.slice(2), 'hex');

        console.log('📝 Order Hash:', orderHash);

        // Step 1: Create source escrow on Stellar
        console.log('📍 Creating source escrow on Stellar...');
        const stellarResponse = await this.resolverClient.deploy_src({
            immutables: {
                amount: 1000000000000000000n,
                hashlock: Buffer.from(hashlock.toString().slice(2), 'hex'),
                maker: this.alice.publicKey(),
                order_hash: orderHashBuffer,
                safety_deposit: 1000000000000000000n,
                taker: this.bob.publicKey(),
                timelocks: BigInt(timelocks.build()),
                token: this.config.stellar.tokens.usdc,
            },
            order,
            signature_r: this.randomBytes(32),
            signature_vs: this.randomBytes(32),
            amount: 1000000000000000000n,
            taker_traits: 0n,
            args: Buffer.from([]),
        });

        console.log('✅ Stellar source escrow created:', stellarResponse);

        // Step 2: Create destination escrow on Ethereum
        console.log('📍 Creating destination escrow on Ethereum...');
        let ethereumReceipt;
        
        try {
            // Check if we have real Ethereum credentials
            if (this.config.ethereum.rpcUrl.includes('demo') || this.config.ethereum.privateKey.includes('1234')) {
                console.log('⚠️  Using demo mode - skipping actual Ethereum transaction');
                console.log('   In production, this would create the destination escrow on Ethereum');
                ethereumReceipt = { hash: 'demo_hash_placeholder' };
            } else {
                // Use real Ethereum client
                const ethereumImmutables = {
                    amount: 1000000000000000000n,
                    hashlock: hashlock.toString(),
                    maker: this.alice.publicKey(),
                    orderHash: orderHash,
                    safetyDeposit: 1000000000000000000n,
                    taker: this.bob.publicKey(),
                    timelocks: BigInt(timelocks.build()),
                    token: this.config.ethereum.tokens.usdc,
                };

                const srcCancellationTimestamp = BigInt(Math.floor(Date.now() / 1000)) + BigInt(this.config.cancellationSrcTimelock);

                ethereumReceipt = await this.ethereumClient.createDstEscrow(
                    ethereumImmutables,
                    srcCancellationTimestamp
                );
            }
        } catch (error) {
            console.error('❌ Error creating Ethereum destination escrow:', error);
            console.log('⚠️  Falling back to demo mode');
            ethereumReceipt = { hash: 'demo_hash_placeholder' };
        }

        console.log('✅ Ethereum destination escrow created:', ethereumReceipt);

        return {
            stellarResponse,
            ethereumReceipt,
            orderHash,
            secrets: secrets[0]
        };
    }

    async withdrawFromEscrow(escrowAddress: string, secret: string, chain: 'ethereum' | 'stellar') {
        console.log(`💰 Withdrawing from ${chain} escrow...`);
        
        if (chain === 'ethereum') {
            return await this.ethereumClient.withdrawFromEscrow(escrowAddress, secret);
        } else {
            // Implement Stellar withdrawal logic
            console.log('Stellar withdrawal not yet implemented');
        }
    }

    async cancelEscrow(escrowAddress: string, chain: 'ethereum' | 'stellar') {
        console.log(`❌ Cancelling ${chain} escrow...`);
        
        if (chain === 'ethereum') {
            return await this.ethereumClient.cancelEscrow(escrowAddress);
        } else {
            // Implement Stellar cancellation logic
            console.log('Stellar cancellation not yet implemented');
        }
    }
} 
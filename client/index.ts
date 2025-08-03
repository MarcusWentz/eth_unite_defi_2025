import { parseArgs } from "util";
import { Keypair, Contract, rpc as StellarRpc, TransactionBuilder, Networks, BASE_FEE } from "@stellar/stellar-sdk";
import { Client as ResolverClient, type Order } from "./bindings/resolver/src/index";
import { Client as OrderClient } from "./bindings/order/src/index";
import { Client as TokenClient } from "./bindings/test-token/src/index";
import { keccak256, toHex, type Hex } from 'viem';
import {  
    HashLock,  
    TimeLocks,
} from '@1inch/cross-chain-sdk'  


enum Network {
    TESTNET = "http://localhost:8000",
    // MAINNET = "https://soroban-mainnet.stellar.org:443",
}

const getRpcUrl = (network: Network) => {
    return `${network}/soroban/rpc`
}

const server_url = Network.TESTNET;

const networkPassphrase = "Standalone Network ; February 2017"

const networkConfigs = {
    [Network.TESTNET]: {
        networkPassphrase,
        rpcUrl: server_url,
    }
}

const createAccount = (
    address?: string,
): Keypair => {
    return address ? Keypair.fromSecret(address) : Keypair.random();
}

type ScriptConfig = {
    escrowFactory: string,
    limitOrderProtocol: string,
    deployer: string,
    maker: string,
    srcToken: string,
    dstToken: string,
    resolver: string,
    srcAmount: number,
    dstAmount: number,
    safetyDeposit: number,

    // src timelocks
    withdrawalSrcTimelock: number,
    publicWithdrawalSrcTimelock: number,
    cancellationSrcTimelock: number,
    publicCancellationSrcTimelock: number,

    // dst timelocks
    withdrawalDstTimelock: number,
    publicWithdrawalDstTimelock: number,
    cancellationDstTimelock: number,
    
    secret: string,
}

const getConfig = async (): Promise<ScriptConfig> => {
    const config = await Bun.file("config/config.json").json();
    return {
        ...config,
    }
}

const topupWithFriendbot = async (
    address: string,
) => {
    console.log(`Topping up account ${address} with friendbot`);
    const friendbotUrl = `${server_url}/friendbot?addr=${address}`;
    const response = await fetch(friendbotUrl);
    if (!response.ok) {
        console.error(`Failed to topup account ${address} with friendbot`);
        throw new Error(`Failed to topup account ${address} with friendbot`);
    }
    console.log(`Friendbot response: ${response.status}`);
    return response;
}

const randomBytes = (length: number) => {
    return Buffer.from(crypto.getRandomValues(new Uint8Array(length)));
}

const getSecrets = (secretSalt: string) => {
    const secret = keccak256(toHex(secretSalt));
    const hashlock = keccak256(secret);

    // for partial fill it would be an array of secrets
    const secrets = [hashlock]

    return secrets;
}

const getHashlock = (secrets: [Hex, ...Hex[]]) => {
    if (secrets.length === 1) {
        return HashLock.forSingleFill(secrets[0])
    }
    return secrets.map(secret => keccak256(toHex(secret)));
}

// Helper function to convert hex string to Buffer
const hexToBuffer = (hex: string): Buffer => {
    // Remove '0x' prefix if present
    const cleanHex = hex.startsWith('0x') ? hex.slice(2) : hex;
    return Buffer.from(cleanHex, 'hex');
}

const getTimelocks = (config: ScriptConfig) => {
    return TimeLocks.new({
        srcWithdrawal: BigInt(config.withdrawalSrcTimelock),
        srcPublicWithdrawal: BigInt(config.publicWithdrawalSrcTimelock),
        srcCancellation: BigInt(config.cancellationSrcTimelock),
        srcPublicCancellation: BigInt(config.publicCancellationSrcTimelock),

        dstWithdrawal: BigInt(config.withdrawalDstTimelock),
        dstPublicWithdrawal: BigInt(config.publicWithdrawalDstTimelock),
        dstCancellation: BigInt(config.cancellationDstTimelock),
    })
}

const signOrder = (
    keypair: Keypair,
    orderHash: Buffer,
) => {
    const signature = keypair.signDecorated(orderHash);
    return signature;
}

const main = async (
) => {

    const server = new StellarRpc.Server(
        getRpcUrl(server_url),
        {
            allowHttp: server_url.startsWith("http://"),
        }
    );

    const scriptConfig = await getConfig();

    const secrets = getSecrets(scriptConfig.secret);
    if (!secrets.length) {
        throw new Error("No secrets found");
    }
    const hashlock = getHashlock(secrets as [Hex, ...Hex[]]);
    const secretHashes = secrets.map((s) => HashLock.hashSecret(s))

    const timelocks = getTimelocks(scriptConfig);

    const alice = createAccount();
    await topupWithFriendbot(alice.publicKey());

    const bob = createAccount();
    await topupWithFriendbot(bob.publicKey());

    console.log('Alice:', alice.publicKey());
    console.log('Bob:', bob.publicKey());

    const order = {
        maker: alice.publicKey(),
        maker_asset: scriptConfig.srcToken, // Use actual deployed test token
        taker_asset: scriptConfig.dstToken, // Use actual deployed test token
        maker_traits: 967101221531144175919556390646195146547200n,
        receiver: bob.publicKey(),
        salt: 1n,
        taking_amount: BigInt(scriptConfig.dstAmount * 10**18),
        making_amount: BigInt(scriptConfig.srcAmount * 10**18),
    }

    const resolver_client = new ResolverClient({
        contractId: scriptConfig.resolver,
        networkPassphrase: networkConfigs[Network.TESTNET].networkPassphrase,
        rpcUrl: getRpcUrl(networkConfigs[Network.TESTNET].rpcUrl),
        allowHttp: networkConfigs[Network.TESTNET].rpcUrl.startsWith("http://"),
    })

    const order_client = new OrderClient({
        contractId: scriptConfig.limitOrderProtocol,
        networkPassphrase: networkConfigs[Network.TESTNET].networkPassphrase,
        rpcUrl: getRpcUrl(networkConfigs[Network.TESTNET].rpcUrl),
        allowHttp: networkConfigs[Network.TESTNET].rpcUrl.startsWith("http://"),
    })

    /**
     * Author: @Skanislav
     * HERE I STOPPED BECAUSE I NEED TO SLEEP
     */

    const orderHash = await order_client.order_hash({ order });

    console.log(orderHash.result, '<<<<<<<<< order hash')

    // Create token client for maker token
    const token_client = new TokenClient({
        contractId: scriptConfig.srcToken,
        networkPassphrase: networkConfigs[Network.TESTNET].networkPassphrase,
        rpcUrl: getRpcUrl(networkConfigs[Network.TESTNET].rpcUrl),
        allowHttp: networkConfigs[Network.TESTNET].rpcUrl.startsWith("http://"),
    });

    // Skip token operations for now to test core functionality
    console.log('Skipping token operations, using XLM addresses...');

    const signature = signOrder(alice, orderHash.result);

    // Convert hashlock and order_hash to Buffer for the immutables
    const hashlockBuffer = hexToBuffer(hashlock.toString());
    
    // The orderHash.result is already a Buffer, so we can use it directly
    const orderHashBuffer = orderHash.result;

    // Use the build method to get the packed U256 value
    const timelocksValue = timelocks.build();

    const deployTx = await resolver_client.deploy_src({
        immutables: {
            amount: BigInt(scriptConfig.srcAmount * 10**18),
            hashlock: hashlockBuffer,
            maker: alice.publicKey(),
            order_hash: orderHashBuffer,
            safety_deposit: BigInt(scriptConfig.safetyDeposit * 10**18),
            taker: bob.publicKey(),
            timelocks: timelocksValue,
            token: scriptConfig.srcToken, // Use actual deployed test token
        },
        order,
        signature_r: randomBytes(32),
        signature_vs: randomBytes(32),
        amount: BigInt(scriptConfig.srcAmount * 10**18),
        taker_traits: 0n,
        args: Buffer.from([]),
    })

    // Sign and send the transaction
    const result = await deployTx.signAndSend({
        signTransaction: (tx: any) => {
            tx.sign(alice);
            return tx;
        }
    });
    
    console.log('Deploy transaction result:', result);
}


main()

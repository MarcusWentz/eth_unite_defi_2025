import { ethers } from 'ethers';
import { keccak256, toHex, type Hex } from 'viem';
import {  
    HashLock,  
    TimeLocks,
} from '@1inch/cross-chain-sdk';

// Ethereum contract ABIs (you'll need to import these from your Solidity contracts)
const ESCROW_FACTORY_ABI = [
    "function createSrcEscrow(tuple(uint256 amount, bytes32 hashlock, address maker, bytes32 orderHash, uint256 safetyDeposit, address taker, uint256 timelocks, address token) immutables, tuple(address maker, address makerAsset, uint256 makerTraits, uint256 makingAmount, address receiver, uint256 salt, address takerAsset, uint256 takingAmount) order, bytes32 signatureR, bytes32 signatureVS, uint256 amount, uint256 takerTraits, bytes args) external returns (address)",
    "function createDstEscrow(tuple(uint256 amount, bytes32 hashlock, address maker, bytes32 orderHash, uint256 safetyDeposit, address taker, uint256 timelocks, address token) dstImmutables, uint256 srcCancellationTimestamp) external returns (address)"
];

const BASE_ESCROW_ABI = [
    "function withdraw(bytes32 secret) external",
    "function cancel() external",
    "function rescue() external"
];

export class EthereumClient {
    private provider: ethers.JsonRpcProvider;
    private wallet: ethers.Wallet;
    private escrowFactory: ethers.Contract;

    constructor(
        rpcUrl: string,
        privateKey: string,
        escrowFactoryAddress: string
    ) {
        this.provider = new ethers.JsonRpcProvider(rpcUrl);
        this.wallet = new ethers.Wallet(privateKey, this.provider);
        this.escrowFactory = new ethers.Contract(
            escrowFactoryAddress,
            ESCROW_FACTORY_ABI,
            this.wallet
        );
    }

    async createSrcEscrow(
        immutables: {
            amount: bigint;
            hashlock: string;
            maker: string;
            orderHash: string;
            safetyDeposit: bigint;
            taker: string;
            timelocks: bigint;
            token: string;
        },
        order: {
            maker: string;
            makerAsset: string;
            makerTraits: bigint;
            makingAmount: bigint;
            receiver: string;
            salt: bigint;
            takerAsset: string;
            takingAmount: bigint;
        },
        signatureR: string,
        signatureVS: string,
        amount: bigint,
        takerTraits: bigint,
        args: string = "0x"
    ) {
        try {
            const tx = await this.escrowFactory.createSrcEscrow(
                immutables,
                order,
                signatureR,
                signatureVS,
                amount,
                takerTraits,
                args,
                { gasLimit: 500000 }
            );
            
            const receipt = await tx.wait();
            console.log('Src Escrow created on Ethereum:', receipt);
            return receipt;
        } catch (error) {
            console.error('Error creating src escrow on Ethereum:', error);
            throw error;
        }
    }

    async createDstEscrow(
        dstImmutables: {
            amount: bigint;
            hashlock: string;
            maker: string;
            orderHash: string;
            safetyDeposit: bigint;
            taker: string;
            timelocks: bigint;
            token: string;
        },
        srcCancellationTimestamp: bigint
    ) {
        try {
            const tx = await this.escrowFactory.createDstEscrow(
                dstImmutables,
                srcCancellationTimestamp,
                { gasLimit: 500000 }
            );
            
            const receipt = await tx.wait();
            console.log('Dst Escrow created on Ethereum:', receipt);
            return receipt;
        } catch (error) {
            console.error('Error creating dst escrow on Ethereum:', error);
            throw error;
        }
    }

    async withdrawFromEscrow(escrowAddress: string, secret: string) {
        try {
            const escrow = new ethers.Contract(escrowAddress, BASE_ESCROW_ABI, this.wallet);
            const tx = await escrow.withdraw(secret, { gasLimit: 200000 });
            const receipt = await tx.wait();
            console.log('Withdrawal successful on Ethereum:', receipt);
            return receipt;
        } catch (error) {
            console.error('Error withdrawing from escrow on Ethereum:', error);
            throw error;
        }
    }

    async cancelEscrow(escrowAddress: string) {
        try {
            const escrow = new ethers.Contract(escrowAddress, BASE_ESCROW_ABI, this.wallet);
            const tx = await escrow.cancel({ gasLimit: 200000 });
            const receipt = await tx.wait();
            console.log('Cancellation successful on Ethereum:', receipt);
            return receipt;
        } catch (error) {
            console.error('Error cancelling escrow on Ethereum:', error);
            throw error;
        }
    }

    async rescueEscrow(escrowAddress: string) {
        try {
            const escrow = new ethers.Contract(escrowAddress, BASE_ESCROW_ABI, this.wallet);
            const tx = await escrow.rescue({ gasLimit: 200000 });
            const receipt = await tx.wait();
            console.log('Rescue successful on Ethereum:', receipt);
            return receipt;
        } catch (error) {
            console.error('Error rescuing escrow on Ethereum:', error);
            throw error;
        }
    }
} 
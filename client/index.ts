import { parseArgs } from "util";
import { Keypair, Contract, rpc as StellarRpc, TransactionBuilder, Networks, BASE_FEE } from "@stellar/stellar-sdk";
import { Client as DutchAuctionClient } from "./bindings/dutch_auction/src/index";
import { Client as OrderClient } from "./bindings/order/src/index";


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

type Config = {
    orderContractAddress: string,
}

const main = async (
    config: Config,
) => {

    const server = new StellarRpc.Server(
        server_url,
        {
            allowHttp: server_url.startsWith("http://"),
        }
    );

    const alice = createAccount();
    await topupWithFriendbot(alice.publicKey());

    const bob = createAccount();
    await topupWithFriendbot(bob.publicKey());

    // const order_contract = new Contract(contract_addresses.order);

    const order = {
        maker: alice.publicKey(),
        maker_asset: "CAPXKPSVXRJ56ZKR6XRA7SB6UGQEZD2UNRO4OP6V2NYTQTV6RFJGIRZM",
        taker_asset: "CA7N3TLKV27AYBLL6AR7ICJ6C5AMPMCQOGFKI6ZU2FNHRRDN4CNBL5T5",
        maker_traits: 967101221531144175919556390646195146547200n,
        receiver: bob.publicKey(),
        salt: 1n,
        taking_amount: 1000000000000000000n,
        making_amount: 1000000000000000000n,
    }

    console.log(config.orderContractAddress);
    const order_client = new OrderClient({
        contractId: config.orderContractAddress,
        networkPassphrase: networkConfigs[Network.TESTNET].networkPassphrase,
        rpcUrl: getRpcUrl(networkConfigs[Network.TESTNET].rpcUrl),
        allowHttp: networkConfigs[Network.TESTNET].rpcUrl.startsWith("http://"),
    })

    const order_hash = randomBytes(32);

    const simulateTx = await order_client.calculate_making_amount({
        order,
        _extension: Buffer.from([]),
        requested_taking_amount: 1000000000000000000n,
        remaining_making_amount: 1000000000000000000n,
        order_hash: order_hash,
        auction_details: {
            auction_start_time: 1n,
            taking_amount_start: 1n,
            taking_amount_end: 1n,
        }
    })

    const data = simulateTx.result;

    console.log(data);
}

const { values } = parseArgs({
    args: Bun.argv,
    options: {
      orderContractAddress: {
        type: 'string',
      },
    },
    strict: true,
    allowPositionals: true,
  });


main(values as Config)

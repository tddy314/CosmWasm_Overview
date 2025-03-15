import { SigningCosmWasmClient, Secp256k1HdWallet, setupWebKeplr, coin, UploadResult, InstantiateResult, toBinary } from "cosmwasm";
import { CosmWasmClient } from "cosmwasm";
import * as dotenv from "dotenv"
import { Decimal } from "@cosmjs/math";
import * as fs from "fs";
import { get } from "http";
import { resourceLimits } from "worker_threads";
import { GasPrice } from "cosmwasm";
import { features, send } from "process";
import { getDefaultHighWaterMark } from "stream";


dotenv.config();

const getTxAPI = "https://testnet-lcd.orai.io/cosmos/"
const rpcEndpoint = "https://testnet-rpc.orai.io:443/";
const chainID = "Oraichain-testnet"
const admin = process.env.MNEMONIC!;
const user = process.env.MNEMONIC2!;
const contAddress = "orai1nzshdp85de6fpu6df74kzxrzt4azfcenz9l4gsykxqk5etmqf6aqms2vqy";

function hexToDecimal(hex: string): string {
    // Remove the '0x' prefix if present
    if (hex.startsWith('0x')) {
        hex = hex.slice(2);
    }

    // Convert the hexadecimal string to a decimal string
    const decimalString = BigInt(`0x${hex}`).toString();

    return decimalString;
}

function ReadFile(path: string): Uint8Array {
    var file = fs.readFileSync(path);
    return new Uint8Array(file);
}

async function getWallet(sender: any): Promise<Secp256k1HdWallet> {
    const wallet = await Secp256k1HdWallet.fromMnemonic(sender, { prefix: "orai" });
    return wallet;
}



async function getClient(sender: any): Promise<SigningCosmWasmClient> {
    const wallet = await getWallet(sender);
    const client = await SigningCosmWasmClient.connectWithSigner(
        rpcEndpoint,
        wallet,
        {
            gasPrice: { //dat gasPrice
                denom: "orai",
                amount: Decimal.fromUserInput("0.001", 6)
            }
        }
    )
    return client;
}

async function Upload(path: string): Promise<UploadResult> {
    const wallet = await getWallet(admin);
    const client = await getClient(admin);

    const sender = (await wallet.getAccounts())[0].address;
    const wasmCode = ReadFile(path);
    const fee = "auto";
    const memo: any = null;
    const res = await client.upload(sender, wasmCode, fee, memo)
    return res;
}

async function instantiate(codeID: number): Promise<InstantiateResult> {
    const wallet = await getWallet(admin);
    const client = await getClient(admin);
    const sender = (await wallet.getAccounts())[0].address;
    const msg = {
        admin: sender,
    }
    const label = "test";
    const fee = "auto";
    const res = await client.instantiate(sender, codeID, msg, label, fee);
    return res;
}

async function Query_admin(sender: any) {
    const client = await getClient(sender);
    const contractAddress = contAddress;
    const msg = {
        admin: {

        }
    }
   
    const res = await client.queryContractSmart(contractAddress, msg);
    return res;
}

async function Create_Poll(sender: any, id: string, ques: string, op: string[]) {
    const wallet = await getWallet(sender);
    const client = await getClient(sender);
    const senderAddress = (await wallet.getAccounts())[0].address;
    const contractAddress = contAddress;

    const msg = {
        create_poll: {
            poll_id: id,
            question: ques,
            options: op,
        }
    };

    console.log("Create a Poll: " + JSON.stringify(msg));
    const fee = "auto";
    const memo: any = null;
    const res = await client.execute(senderAddress, contractAddress, msg, fee, memo);
    return res;
}



async function main() {
    //Deploy
    // const resUpload = await Upload("./artifacts/cw-starter.wasm");
    // const resInit = await instantiate(resUpload.codeId);

    // console.log("Deployment: Success");
    // console.log("Contract address: " + resInit.contractAddress);

    //admin
    const walletAd = await getWallet(admin);
    const Admin = (await walletAd.getAccounts())[0].address;
    console.log("Admin: " + Admin);

    //user
    const walletus = await getWallet(user);
    const User = (await walletus.getAccounts())[0].address;
    console.log("User: " + User);

    //Query1 
    const resAdmin = await Query_admin(user);
    console.log(resAdmin);

    //Execute
    let res_CreatePoll;
    //Poll1
//    res_CreatePoll = await Create_Poll(user, "One", "Diar Diar?", ["yes", "no"]);
//    console.log("Success Create a Poll: " + res_CreatePoll);

    //Poll2
//    res_CreatePoll = await Create_Poll(user, "Two", "Hey Hey?", ["yes", "no"]);
//    console.log("Success Create a Poll: " + JSON.stringify(res_CreatePoll));
}
main();
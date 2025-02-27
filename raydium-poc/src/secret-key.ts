import bs58 from "bs58";
import fs from "fs/promises";
import os from "os";
import { Keypair } from "@solana/web3.js";

async function getKeypairFromFileAsync(filePath: string): Promise<Keypair> {
  try {
    const rawData = await fs.readFile(filePath, "utf-8");
    const secretKeyArray: number[] = JSON.parse(rawData);
    const secretKey = Uint8Array.from(secretKeyArray);
    return Keypair.fromSecretKey(secretKey);
  } catch (error) {
    console.error("Error reading keypair from file:", error);
    throw error; // Rethrow or handle the error as needed
  }
}

async function verifyKeypairFile(filePath: string) {
  try {
    await fs.access(filePath); // Check if file exists
    const keypair = await getKeypairFromFileAsync(filePath);
    console.log(
      "Valid keypair found. Public key:",
      keypair.publicKey.toString()
    );
    console.log(
      " => Top Secret key (Base58) for testing:",
      bs58.encode(keypair.secretKey)
    );
    return keypair;
  } catch (error) {
    console.log("No valid id.json found at:", filePath);
    throw error;
  }
}

async function main() {
  try {
    const homeDir = os.homedir();
    const filePath = `${homeDir}/.config/solana/id.json`;
    await verifyKeypairFile(filePath);
  } catch (error) {
    console.error("Error verifying keypair from file:", error);
  }
}

main();

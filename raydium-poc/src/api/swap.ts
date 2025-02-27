// https://docs.raydium.io/raydium/traders/trade-api

// swap.ts
import {
  Transaction,
  VersionedTransaction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import { NATIVE_MINT } from "@solana/spl-token";
import axios from "axios";
import { API_URLS } from "@raydium-io/raydium-sdk-v2";
import { connection, fetchTokenAccountData, owner } from "../config";

interface SwapCompute {
  id: string;
  success: true;
  version: "V0" | "V1";
  openTime?: undefined;
  msg: undefined;
  data: {
    swapType: "BaseIn" | "BaseOut";
    inputMint: string;
    inputAmount: string;
    outputMint: string;
    outputAmount: string;
    otherAmountThreshold: string;
    slippageBps: number;
    priceImpactPct: number;
    routePlan: {
      poolId: string;
      inputMint: string;
      outputMint: string;
      feeMint: string;
      feeRate: number;
      feeAmount: string;
    }[];
  };
}

// Configuration
const inputMint = NATIVE_MINT.toBase58(); // SOL
// https://github.com/raydium-io/raydium-sdk/issues/62
const outputMint = "9T7uw5dqaEmEC4McqyefzYsEg5hoC4e2oV8it1Uc4f1U"; // USDC
const amount = "10000000"; // 0.01 SOL in lamports (10^6 for SOL)
const slippage = 0.5; // 0.5% slippage
const txVersion: string = "V0"; // Use "V0" for VersionedTransaction, "LEGACY" for Transaction
const isV0Tx = txVersion === "V0";
const isInputSol = inputMint === NATIVE_MINT.toBase58();
const isOutputSol = outputMint === NATIVE_MINT.toBase58();

async function performSwap() {
  try {
    // Check if owner has SOL balance
    const balance = await connection.getBalance(owner.publicKey);
    if (balance < parseInt(amount) + 5000) {
      // 5000 lamports for fees
      throw new Error(
        `Insufficient SOL balance: ${balance} lamports. Need at least ${
          parseInt(amount) + 5000
        } lamports.`
      );
    }

    // Step 1: Fetch swap computation
    const { data: swapResponse } = await axios.get<SwapCompute>(
      `${
        API_URLS.SWAP_HOST
      }/compute/swap-base-in?inputMint=${inputMint}&outputMint=${outputMint}&amount=${amount}&slippageBps=${
        slippage * 100
      }&txVersion=${txVersion}`
    );
    console.log("Swap computation response:", swapResponse);

    // Step 2: Fetch token accounts (assuming fetchTokenAccountData is defined in config)
    const { tokenAccounts } = await fetchTokenAccountData();
    console.log("Token Accounts:", tokenAccounts);
    const inputTokenAcc = tokenAccounts.find(
      (a) => a.mint.toBase58() === inputMint
    )?.publicKey;
    console.log("Input Token:", inputTokenAcc);
    const outputTokenAcc = tokenAccounts.find(
      (a) => a.mint.toBase58() === outputMint
    )?.publicKey;
    console.log("Output Token:", outputTokenAcc);

    // Step 3: Fetch priority fee stats (optional)
    const { data: feeData } = await axios.get<{
      id: string;
      success: boolean;
      data: { default: { vh: number; h: number; m: number } };
    }>(`${API_URLS.BASE_HOST}${API_URLS.PRIORITY_FEE}`);
    console.log("Priority fee stats:", feeData.data.default);

    // Step 4: Request swap transactions
    const { data: swapTransactions } = await axios.post<{
      id: string;
      version: string;
      success: boolean;
      data: { transaction: string }[];
    }>(`${API_URLS.SWAP_HOST}/transaction/swap-base-in`, {
      computeUnitPriceMicroLamports: String(feeData.data.default.h), // Use high priority fee
      swapResponse,
      txVersion,
      wallet: owner.publicKey.toBase58(),
      wrapSol: isInputSol,
      unwrapSol: isOutputSol,
      inputAccount: inputTokenAcc?.toBase58(),
      outputAccount: outputTokenAcc?.toBase58(),
    });

    // Step 5: Deserialize transactions
    const allTxBuf = swapTransactions.data.map((tx) =>
      Buffer.from(tx.transaction, "base64")
    );
    const allTransactions = allTxBuf.map((txBuf) =>
      isV0Tx ? VersionedTransaction.deserialize(txBuf) : Transaction.from(txBuf)
    );
    console.log(`Total ${allTransactions.length} transactions prepared`);

    // Step 6: Sign and send transactions
    let idx = 0;
    if (!isV0Tx) {
      for (const tx of allTransactions) {
        console.log(`${++idx} transaction sending...`);
        const transaction = tx as Transaction;
        transaction.sign(owner);
        const txId = await sendAndConfirmTransaction(
          connection,
          transaction,
          [owner],
          {
            skipPreflight: true,
            commitment: "confirmed",
          }
        );
        console.log(`${idx} transaction confirmed, txId: ${txId}`);
      }
    } else {
      for (const tx of allTransactions) {
        idx++;
        const transaction = tx as VersionedTransaction;
        transaction.sign([owner]);
        const txId = await connection.sendTransaction(transaction, {
          skipPreflight: true,
        });
        const { lastValidBlockHeight, blockhash } =
          await connection.getLatestBlockhash({
            commitment: "finalized",
          });
        console.log(`${idx} transaction sending..., txId: ${txId}`);
        await connection.confirmTransaction(
          {
            blockhash,
            lastValidBlockHeight,
            signature: txId,
          },
          "confirmed"
        );
        console.log(`${idx} transaction confirmed`);
      }
    }
  } catch (error) {
    console.error("Error during swap:", error);
    throw error;
  }
}

performSwap().catch((error) => console.error("Swap failed:", error));

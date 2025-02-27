import { initSdk } from "./config";

async function init() {
  const raydium = await initSdk();
  const tokens = await raydium.fetchV3TokenList();
  console.log("Tokens", tokens);
}

// https://docs.raydium.io/raydium/protocol/developers/addresses

init();

# lp-learning

Learning Liquidity Provision

## References

- [Orca Whirlpools Environment Setup](https://dev.orca.so/Whirlpools%20SDKs/Whirlpools/Environment%20Setup)
- [Raydium API Docs](https://docs.raydium.io/raydium)
- [Solana Docs](https://solana.com/docs)

## Install Solana

- [Install Solana](https://solana.com/docs/intro/installation)

```shell
# Version check
rustc --version
solana --version
avm --version
anchor --version
node --version
```

- [Solana CLI Basics](https://solana.com/docs/intro/installation#solana-cli-basics)

```shell
# Solana Config
solana config get

solana config set -um    # For mainnet-beta
solana config set -ud    # For devnet
solana config set -ul    # For localhost
solana config set -ut    # For testnet

# Create Wallet
solana-keygen new
solana address

# Airdrop SOL
solana config set -ud
solana airdrop 2
solana balance
solana transfer ${SOL_ADDR}$ 1
```

## Instal Raydium SDK v2

```shell
npm install @raydium-io/raydium-sdk-v2
```

## Setup Orca Whirlpools SDK

```shell
cargo add orca_whirlpools
```

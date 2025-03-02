# What Are Liquidity Pools?

- [Creating Liquidity Pools on Orca](https://dev.orca.so/Whirlpools%20SDKs/Whirlpools/Whirlpool%20Management/Create%20Pool)

Both **Splash Pools** and **Concentrated Liquidity Pools** are used in decentralised finance (DeFi) on automated market makers (AMMs) like Orca. They allow users to trade tokens by swapping them through a pool, while liquidity providers (people who add funds to the pool) earn fees from these trades. The key difference lies in how these pools work and who they're designed for.

## Splash Pools

Splash Pools are the simpler option, designed for ease of use and accessibility.

- How They Work:
  - You only need to provide the mint addresses of the two tokens you want to pair and set an initial price. That's it-no complicated settings or strategies required.
  - Liquidity is spread across all possible prices, similar to traditional AMM pools.
- Key Features:
  - Simplicity: Minimal setup makes them beginner-friendly.
  - Quick to Launch: Ideal for new tokens, especially community-driven projects like memecoins.
  - Community Focus: They lower the barrier to entry, encouraging more people to participate in liquidity provision without needing technical expertise.
- Best For:
  - New token launches where speed and simplicity matter.
  - Projects that prioritise community engagement over optimisation.

## Concentrated Liquidity Pools

Concentrated Liquidity Pools are more advanced, offering greater control and efficiency but requiring more effort to manage.

- How They Work:
  - Liquidity providers can choose specific price ranges where their funds will be active. For example, if you think a token will trade between $1 and $2, you can concentrate your liquidity there.
  - Outside those ranges, your liquidity doesn't earn fees, but within them, it's highly efficient.
- Key Features:
  - Higher Capital Efficiency: By focusing liquidity where trading happens, you can earn more fees with less capital compared to spreading it across all prices.
  - More Control: You decide where your liquidity works, tailoring it to market conditions.
  - Complexity: Requires understanding price movements and strategic planning.
- Best For:
  - Experienced users who want to optimize their returns.
  - Established trading pairs where price behavior is easier to predict.

## Key Differences

Here's a side-by-side comparison to make it crystal clear:

| Aspect             | Splash Pools                                           | Concentrated Liquidity Pools                       |
| ------------------ | ------------------------------------------------------ | -------------------------------------------------- |
| Complexity         | Simple, minimal setup (just token addresses and price) | More complex, requires setting price ranges        |
| Control            | No control over liquidity placement                    | Full control over where liquidity is active        |
| Capital Efficiency | Lower-liquidity is spread across all prices            | Higher-liquidity is concentrated where it's needed |
| Best Use Case      | New tokens, community projects, beginners              | Experienced users, optimized trading pairs         |
| User Experience    | Easy and accessible                                    | Requires market knowledge and strategy             |

## Which Should You Choose?

- Choose Splash Pools if you're launching a new token, want a hassle-free setup, or are part of a community-driven project like a memecoin. They're perfect when simplicity and participation are your priorities.
- Choose Concentrated Liquidity Pools if you're an experienced user looking to maximize your earnings by strategically managing your liquidity. They shine when you can predict price ranges and want higher efficiency.

In short, Splash Pools are about ease and accessibility, while Concentrated Liquidity Pools are about control and efficiency.

# What Is Tick Spacing?

- [Understanding Ticks, Tick Spacing, and Fee Tiers on Orca](https://docs.orca.so/educational-documents/understanding-ticks-tick-spacing-and-fee-tiers-on-orca)

**Tick spacing** is a parameter that defines the granularity of price ranges in a concentrated liquidity pool. In simpler terms, it determines how finely you can divide the price spectrum into discrete "ticks" where liquidity can be positioned. Each tick represents a specific price point, and the spacing between ticks controls how close or far apart these price points are.

- **Ticks**: In concentrated liquidity models (inspired by Uniswap V3 and adopted by Orca, Raydium, etc.), the price range of a trading pair isn't continuous—it's broken into a series of discrete steps called ticks. Each tick corresponds to a price, and liquidity providers can allocate their funds to specific tick ranges (e.g., "I'll provide liquidity between $1 and $2").
- **Tick Spacing**: This is the distance between consecutive ticks, measured as a fixed increment in a logarithmic price scale. A smaller tick spacing means more ticks (finer granularity), while a larger tick spacing means fewer ticks (coarser granularity).

## How Does It Work?

Concentrated liquidity pools use a mathematical formula to map ticks to prices, based on a logarithmic scale (since token prices can vary widely). The formula approximates:

\[ Price = 1.0001^{tick} \]

- **1.0001**: A small constant representing a 0.01% price change per tick.
- **tick**: An integer index (e.g., -100, 0, 100).

The **tick spacing** is how many of these tick indices you jump between allowed price points. For example:

- **Tick Spacing = 1**: Every tick is available (e.g., -100, -99, -98, ...), giving the finest control over price ranges.
- **Tick Spacing = 64**: Only every 64th tick is available (e.g., -128, -64, 0, 64, 128), meaning price steps are larger.

In practice, pools use larger tick spacings (like 64 or 128) to balance flexibility with computational efficiency on-chain.

## Why Does It Matter?

- **Liquidity Placement**: When you create a pool or add liquidity, tick spacing defines the "grid" of price points where liquidity can be concentrated. A liquidity provider picks a range (e.g., between tick -64 and tick 64), and the pool only uses liquidity within that range for swaps.
- **Pool Uniqueness**: In Orca Whirlpools (and similar systems), the pool's address (PDA) is derived from the token pair (e.g., Wrapped SOL and devUSDC) _and_ the tick spacing. So, `Wrapped SOL/devUSDC` with tick spacing `64` has a different address from the same pair with tick spacing `128`, even though the tokens are identical.
- **Trading Efficiency**: Smaller tick spacing allows tighter price ranges (more precision), but increases complexity and gas costs. Larger tick spacing simplifies things but reduces precision.

## Example

Let's say you're creating a pool for Wrapped SOL (SOL) and devUSDC:

- **Tick Spacing = 64**:

  - Possible ticks: ..., -128, -64, 0, 64, 128, ...
  - Price at tick 0: \( 1.0001^0 = 1 \) (e.g., 1 SOL = 1 devUSDC).
  - Price at tick 64: \( 1.0001^{64} \approx 1.0064 \) (0.64% higher).
  - Price at tick -64: \( 1.0001^{-64} \approx 0.9936 \) (0.64% lower).
  - Liquidity providers can set ranges like "between tick -64 and 64" (~0.9936 to 1.0064 SOL/devUSDC).

- **Tick Spacing = 128**:
  - Possible ticks: ..., -256, -128, 0, 128, 256, ...
  - Price at tick 128: \( 1.0001^{128} \approx 1.0129 \) (1.29% higher).
  - Price at tick -128: \( 1.0001^{-128} \approx 0.9872 \) (1.28% lower).
  - Wider steps mean less granular control.

## Practical Impact

- **For Pool Creation**: Pick a tick spacing based on your needs:
  - Smaller (e.g., 8, 16): More precise ranges, higher fees for LPs, but more management.
  - Larger (e.g., 64, 128): Easier to manage, slightly less efficient, suitable for stable pairs.
- **For Swapping**: Tick spacing doesn't directly affect you as a trader—it's baked into the pool's design and affects how liquidity is distributed.

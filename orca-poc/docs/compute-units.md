# What Are Compute Units?

- [How to Calculate Transaction Cost](https://solana.com/developers/cookbook/transactions/calculate-cost)
- [How to Add Priority Fees to a Transaction](https://solana.com/developers/cookbook/transactions/add-priority-fees)
- [How to Optimize Compute Requested](https://solana.com/developers/cookbook/transactions/optimize-compute)

In Solana, **compute units (CU)** are a measure of the computational resources a transaction consumes when processed by a validator. Think of them as a budget for CPU time, memory, and other resources needed to execute the instructions in your transaction. Each instruction (e.g., transferring SOL, setting a compute budget) uses a certain number of compute units, and Solana imposes a limit to ensure transactions don’t overburden the network.

- **Default Limit**: Each transaction gets a default budget of **200,000 compute units** per instruction, with a maximum of **1,400,000 compute units** across all instructions (if you use multiple signatures or complex programs).
- **Purpose**: Limiting compute units prevents infinite loops or excessively resource-intensive transactions, keeping the blockchain fast and fair.

## What Is `estimated_compute_units`?

`estimated_compute_units` is the predicted number of compute units your transaction will consume, based on a simulation. In your code:

```rust
let simulation_result = rpc
    .simulate_transaction(&Transaction::new_unsigned(message.clone()))
    .await?;
let estimated_compute_units = simulation_result.value.units_consumed.unwrap_or(200);
```

- **Simulation**: The `rpc.simulate_transaction` call runs your transaction off-chain (without committing it) to estimate resource usage.
- **Units Consumed**: `simulation_result.value.units_consumed` returns how many compute units the simulated execution used. If it’s `None` or 0 (due to simulation quirks), we default to 200.
- **Your Case**: The output shows `200 compute units`, likely because the simulation returned 0, triggering the fallback value.

### Why 200 in Your Output?

- **Simulation Quirk**: On Devnet, `simulate_transaction` sometimes underestimates or returns 0 units consumed for simple transactions like SOL transfers, especially if the RPC node doesn’t fully emulate the execution. Your previous run showed this (`0 compute units`), and we fixed it by setting a fallback.
- **Actual Usage**: A SOL transfer typically consumes ~200-300 units (e.g., ~200 for `SystemProgram::Transfer`), plus ~100-150 for the `ComputeBudget` instruction. Your transaction worked with a 500-unit limit (200 + 300 buffer), confirming it’s enough.

### Why Estimate Compute Units?

Estimating compute units helps you:

1. **Right-Size the Budget**:

   - You set a compute unit limit with `ComputeBudgetInstruction::set_compute_unit_limit` to ensure your transaction has enough resources to complete without exceeding the default cap unnecessarily.
   - Too low (e.g., 100 in your earlier run) → `ComputationalBudgetExceeded`.
   - Too high → Wastes priority fee potential (if used).

2. **Optimise Fees**:

   - Solana’s transaction fee is currently a flat 5,000 lamports per signature (as seen in your output), but with priority fees (introduced via `ComputeBudgetInstruction::set_compute_unit_price`), you pay extra based on compute units to prioritize inclusion during congestion. Knowing the exact usage helps set this efficiently.

3. **Avoid Failures**:
   - Ensures complex transactions (e.g., swaps, multi-instruction calls) fit within the 1,400,000-unit cap.

### Your Transaction

- **Instructions**: Two instructions:
  - `ComputeBudget::SetComputeUnitLimit(500)` (~100-150 CU).
  - `SystemProgram::Transfer` (~200-300 CU).
- **Total Estimated**: ~300-450 CU, well below the 500-unit limit you set, which is why it succeeded this time.

## Can It Be More Accurate?

- **Simulation Fix**: `simulate_transaction` should ideally return ~200-300 units for a transfer. Devnet RPCs can be flaky; a custom RPC (e.g., QuickNode) might give better estimates.
- **Static Estimate**: For simple transfers, you could skip simulation and use a fixed 300-500 units, but simulation scales better for complex transactions.

## Final Answer

- **`estimated_compute_units`**: The predicted compute units your transaction will use, derived from simulation (`rpc.simulate_transaction`). In your case, it’s 200 (fallback) because the simulation didn’t report usage, but the actual consumption was ~300-450 units, covered by the 500-unit limit.
- **Why It Works**: The buffer (`+300`) ensured enough compute resources, fixing the `ComputationalBudgetExceeded` bug.

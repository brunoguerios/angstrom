## PoolSim Abstraction: Decoupling Uniswap Concepts and Preparing for Multi‑AMM

### Motivation

- Today, `BaselinePoolState` is Uniswap‑specific (ticks, sqrtPrice, tick bitmap) but is used broadly as if it were an AMM‑agnostic "pool state".
- To integrate additional venues (e.g., Balancer V3), we need an AMM‑neutral pool simulation interface while keeping Uniswap specifics encapsulated.

### Summary of the Proposal

- Rename `BaselinePoolState` → `UniswapPoolState` (clarifies venue coupling).
- Introduce an AMM‑neutral trait `PoolSim` that exposes only what downstream code actually needs:
  - Light state: block number, fee, current price
  - Swap primitives: simulate amount‑in, simulate to price, noop
- Implement `PoolSim` for `UniswapPoolState` and for a new `BalancerPoolState`.
- Update consumers to depend on `dyn PoolSim` instead of Uniswap types.

### Why This Is Sufficient

A survey of all `BaselinePoolState` call‑sites shows only these capabilities are used:
- Read: `block_number()`, `fee()`, `current_price()`
- Swap: `swap_current_with_amount(...)`, `swap_current_to_price(...)`, `noop()`
- Overloads `Add<Quantity>`/`Sub<Quantity>` just delegate to the same swap entrypoint

No external consumer requires Uniswap ticks/bitmaps directly; they are internal to Uniswap math. Therefore a small trait surface that supplies “current price” and “simulate swaps” is enough for matching, quoting, bundle building, and telemetry.

### Trait Sketch

```rust
// Neutral price and outcome wrappers to avoid leaking venue specifics.
// For Uniswap, Price wraps SqrtPriceX96; for Balancer, derived from weights/amp.
pub struct Price(/* opaque, venue-neutral */);

pub struct SwapOutcome {
    pub fee_ppm: u32,
    pub start_price: Price,
    pub end_price: Price,
    pub amount_in_t0: u128,
    pub amount_in_t1: u128,
    // Add fields as needed by current PoolSwapResult consumers
}

pub trait PoolSim: Send + Sync {
    fn block_number(&self) -> u64;
    fn fee_ppm(&self) -> u32;         // aka fee_e6
    fn current_price(&self) -> Price; // venue-neutral price

    fn simulate_in(&self, amount_in: u128, zero_for_one: bool) -> eyre::Result<SwapOutcome>;
    fn simulate_to_price(&self, price_limit: Price) -> eyre::Result<SwapOutcome>;
    fn noop(&self) -> SwapOutcome;
}

// Optional: ergonomic adapters so existing Quantity math continues to work
pub struct SimRef<'a>(pub &'a dyn PoolSim);
impl<'a> core::ops::Add<Quantity> for SimRef<'a> { /* delegate to simulate_in */ }
impl<'a> core::ops::Sub<Quantity> for SimRef<'a> { /* delegate to simulate_in */ }
```

### Incremental Migration Plan

1) Introduce neutral types
- Add `Price` and `SwapOutcome` to a new `types/amm/` module (or co‑locate near current uni structures) with From/Into conversions:
  - `impl From<SqrtPriceX96> for Price`
  - `impl TryFrom<Price> for SqrtPriceX96>` (for Uniswap‑specific formatting paths)

2) Define `PoolSim` trait
- Create `PoolSim` trait as above in `types/amm/sim.rs`.
- Provide `SimRef` wrapper to preserve arithmetic ergonomics where used (`Add<Quantity>`, `Sub<Quantity>`).

3) Rename Uniswap state
- Rename `crates/types/src/uni_structure/mod.rs::BaselinePoolState` → `UniswapPoolState` (type alias during transition to reduce churn):
  - `pub type BaselinePoolState = UniswapPoolState; // temporary`
- Update `crates/uniswap-v4/.../pool.rs::fetch_pool_snapshot()` to return `UniswapPoolState`.

4) Implement `PoolSim` for Uniswap
- Map existing methods:
  - `block_number`, `fee_ppm` → from existing fields
  - `current_price` → wrap `SqrtPriceX96` into `Price`
  - `simulate_in(amount, dir)` → `swap_current_with_amount`
  - `simulate_to_price` → `swap_current_to_price`
  - `noop` → `noop()`

5) Add Balancer implementation (skeleton)
- Create `BalancerPoolState` with weights/amp/fee.
- Implement `PoolSim` via Balancer math; derive `Price` from invariant.

6) Adapt consumers to `dyn PoolSim`
- Matching engine / order book
  - Change `OrderBook { amm: Option<BaselinePoolState> }` → `Option<Box<dyn PoolSim>>`
  - In `set_amm_if_missing`, box the venue state implementing `PoolSim`.
  - Replace direct `&amm + Quantity` with `SimRef(&*amm) + Quantity` (or helper).

- AMM Quoter
  - `book_snapshots: HashMap<PoolId, (VenuePoolId, BaselinePoolState)>` → `HashMap<PoolId, (VenuePoolId, Box<dyn PoolSim>)>`.
  - `BinarySearchStrategy::give_end_amm_state(&book, searcher)` continues to call through `PoolSim`.
  - Slot0Update: keep Uniswap path as‑is; for venues without ticks, emit venue‑specific updates or mark optional fields. Consider an enum or make `tick` optional.

- Consensus / Telemetry
  - `fetch_pool_snapshot()` returns `Box<dyn PoolSim>` instead of `BaselinePoolState`.
  - Telemetry stores per‑block `PoolSim` snapshots or derived neutral fields (price/fee).

7) Remove the temporary alias
- After all call‑sites are trait‑based, remove `type BaselinePoolState = UniswapPoolState`.

### Impacted Areas (high level)

- `crates/types/src/uni_structure/mod.rs` (rename + trait impl)
- `crates/uniswap-v4/src/uniswap/pool.rs` (snapshot return type)
- `crates/matching-engine/src/book/` and `manager.rs` (hold `Box<dyn PoolSim>`)
- `crates/amm-quoter/src/lib.rs` (book snapshots and computations)
- `crates/consensus/src/rounds/mod.rs` (pool snapshots)
- `crates/telemetry/src/*` (stored snapshot type)

### Conclusions

- The proposed `PoolSim` surface (block, fee, current price, simulate amount‑in, simulate to price, noop) covers all existing usages of `BaselinePoolState`.
- Uniswap‑specific tick/bitmap logic stays encapsulated in `UniswapPoolState`; Balancer math lives in `BalancerPoolState`.
- This keeps “Angstrom pool id” stable and venue‑agnostic, enabling additional AMMs without leaking venue details across the codebase.
- Optional follow‑up: generalize quoter update schema (e.g., make `tick` optional, or use an enum per venue) to avoid forcing Uniswap‑specific fields on non‑tick venues.



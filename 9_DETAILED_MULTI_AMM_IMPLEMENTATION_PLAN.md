# **Detailed Multi-AMM Implementation Plan: From Uniswap-Embedded to Generic**

## **🚨 Critical Analysis: The Real Scope**

After examining the codebase, you're absolutely correct - Uniswap concepts are **deeply embedded** throughout Angstrom. Here's what we're actually dealing with:

### **Uniswap-Specific Concepts Found Everywhere:**
- **2,070 matches** of tick/sqrt_price concepts across **121 files**
- **20 files** directly using `BaselinePoolState` and `BaselineLiquidity`
- Core types like `SqrtPriceX96`, `Tick`, `TickInfo`, `LiqRange` are fundamental to the system
- Matching engine, order pool, consensus, and validation all assume Uniswap mechanics

## **🎯 Phase 1: Foundation - Generic Pool Abstraction**

### **Step 1.1: Create Generic Pool State Types**
**Goal**: Replace Uniswap-specific types with generic ones

**Files to Create:**
```
crates/types/src/amm/
├── mod.rs
├── pool_state.rs
├── liquidity.rs
├── swap_result.rs
└── traits.rs
```

**Implementation:**
```rust
// crates/types/src/amm/pool_state.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PoolState {
    UniswapV4(UniswapV4PoolState),
    BalancerV3(BalancerV3PoolState),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniswapV4PoolState {
    // Existing BaselinePoolState fields
    pub liquidity: BaselineLiquidity,
    pub block: u64,
    pub fee: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalancerV3PoolState {
    // Balancer-specific fields
    pub pool_id: PoolId,
    pub balances: [U256; 2],
    pub weights: [U256; 2],
    pub swap_fee: U256,
    pub block: u64,
}

impl PoolState {
    pub fn current_price(&self) -> Price {
        match self {
            PoolState::UniswapV4(state) => {
                Price::SqrtPriceX96(state.current_price())
            }
            PoolState::BalancerV3(state) => {
                Price::WeightedRatio(state.balances[0], state.balances[1])
            }
        }
    }
    
    pub fn execute_swap(&self, amount: I256, direction: bool) -> Result<SwapResult> {
        match self {
            PoolState::UniswapV4(state) => {
                let result = state.swap_current_with_amount(amount, direction)?;
                Ok(SwapResult::UniswapV4(result))
            }
            PoolState::BalancerV3(state) => {
                let result = state.execute_swap(amount, direction)?;
                Ok(SwapResult::BalancerV3(result))
            }
        }
    }
}
```

Interfaces (rich PoolState; pure-math, snapshot-based)

```rust
// crates/types/src/amm/traits.rs
pub trait PriceInterface: Clone + Send + Sync + core::fmt::Debug + PartialOrd {
    fn to_ray(&self) -> crate::sol_bindings::Ray;
}

pub trait SwapResultInterface: Clone + Send + Sync + core::fmt::Debug {}

pub trait PoolStateInterface: Clone + Send + Sync + core::fmt::Debug {
    type Price: PriceInterface;
    type SwapResult: SwapResultInterface;

    fn block_number(&self) -> u64;
    fn fee(&self) -> u32;
    fn current_price(&self) -> Self::Price;

    fn execute_swap(&self, amount: alloy::primitives::I256, direction: bool)
        -> eyre::Result<Self::SwapResult>;

    fn execute_swap_to_price(&self, price_limit: Self::Price)
        -> eyre::Result<Self::SwapResult>;

    fn noop_swap(&self) -> Self::SwapResult;
}
```

Example implementations (sketches):

```rust
// Uniswap V4
impl PoolStateInterface for UniswapV4PoolState {
    type Price = crate::matching::SqrtPriceX96;
    type SwapResult = UniswapV4SwapResult;

    fn block_number(&self) -> u64 { self.block }
    fn fee(&self) -> u32 { self.fee }
    fn current_price(&self) -> Self::Price { self.liquidity.start_sqrt_price }
    fn execute_swap(&self, amount: I256, direction: bool) -> eyre::Result<Self::SwapResult> {
        self.swap_current_with_amount(amount, direction)
    }
    fn execute_swap_to_price(&self, price: Self::Price) -> eyre::Result<Self::SwapResult> {
        self.swap_current_to_price(price)
    }
    fn noop_swap(&self) -> Self::SwapResult { self.noop() }
}

// Balancer V3 (ReClamm via balancer-maths-rust)
impl PoolStateInterface for ReClammState {
    type Price = ReClammPrice;
    type SwapResult = BalancerV3SwapResult;

    fn block_number(&self) -> u64 { self.block }
    fn fee(&self) -> u32 { self.swap_fee_low_precision() }
    fn current_price(&self) -> Self::Price { ReClammPrice::from_state(self) }
    fn execute_swap(&self, amount: I256, direction: bool) -> eyre::Result<Self::SwapResult> {
        balancer::simulate_swap(self, amount, direction)
    }
    fn execute_swap_to_price(&self, price: Self::Price) -> eyre::Result<Self::SwapResult> {
        balancer::simulate_to_price(self, price)
    }
    fn noop_swap(&self) -> Self::SwapResult { BalancerV3SwapResult::empty(self) }
}
```

### **Step 1.2: Update Matching Engine to Use Generic Types**
**Goal**: Make matching engine work with any AMM type

**Files to Modify:**
- `crates/matching-engine/src/manager.rs`
- `crates/matching-engine/src/matcher/delta.rs`
- `crates/matching-engine/src/simulation/amm.rs`

**Implementation:**
```rust
// crates/matching-engine/src/manager.rs
pub struct MatchingManager {
    // Change from BaselinePoolState to PoolState
    pools: HashMap<PoolId, PoolState>,
    // ... other fields
}

impl MatchingManager {
    pub async fn solve_pools(
        &self,
        limit: Vec<BookOrder>,
        searcher: Vec<OrderWithStorageData<TopOfBlockOrder>>,
        pools: HashMap<PoolId, (Address, Address, PoolState, u16)> // Changed here
    ) -> Result<(Vec<PoolSolution>, BundleGasDetails), MatchingEngineError> {
        // Implementation now works with PoolState
        for (pool_id, (_, _, pool_state, _)) in pools {
            let price = pool_state.current_price();
            // Matching logic works with generic price
        }
    }
}
```

### **Step 1.3: Update Order Pool Types**
**Goal**: Make order pool work with generic pool states

**Files to Modify:**
- `crates/order-pool/src/order_storage.rs`
- `crates/order-pool/src/validator.rs`

## **🎯 Phase 2: AMM Provider Abstraction Layer**

### **Step 2.1: Create AMM Provider Traits**
**Goal**: Define the interface that all AMM providers must implement

**Files to Create:**
```
crates/amm-provider/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── traits.rs
│   ├── types.rs
│   ├── errors.rs
│   └── providers/
│       ├── mod.rs
│       ├── uniswap_v4.rs
│       └── balancer_v3.rs
```

**Implementation:**
```rust
// crates/amm-provider/src/traits.rs
#[async_trait::async_trait]
pub trait AMMProvider: Send + Sync + 'static {
    type PoolState: PoolStateInterface;

    async fn get_pool_state(&self, pool_id: PoolId) -> Result<Self::PoolState, AMMError>;

    // Optional convenience: remote swap using fresh state (not used in hot paths)
    async fn execute_swap(
        &self,
        pool_id: PoolId,
        amount: I256,
        direction: bool
    ) -> Result<<Self::PoolState as PoolStateInterface>::SwapResult, AMMError>;

    // Donation methods - AMM-specific
    async fn plan_donation(&self, pool_id: PoolId, amount: U256) -> Result<DonationPlan, AMMError>;
    async fn execute_donation(&self, plan: DonationPlan) -> Result<DonationResult, AMMError>;
}
```

### **Step 2.2: Create Balancer V3 Provider**
**Goal**: Implement Balancer V3 as a concrete AMM provider

**Implementation:**
```rust
// crates/amm-provider/src/providers/balancer_v3.rs
pub struct BalancerV3Provider {
    vault: Arc<BalancerVault>,
}

#[async_trait]
impl AMMProvider for BalancerV3Provider {
    type PoolState = BalancerV3PoolState;
    type SwapResult = BalancerV3SwapResult;
    
    async fn get_pool_state(&self, pool_id: PoolId) -> Result<Self::PoolState, AMMError> {
        // Get Balancer pool state
        let pool_data = self.vault.get_pool_data(pool_id).await?;
        Ok(BalancerV3PoolState::from_pool_data(pool_data))
    }
    
    async fn plan_donation(&self, pool_id: PoolId, amount: U256) -> Result<DonationPlan, AMMError> {
        // Pool-level donation for Balancer
        Ok(DonationPlan::BalancerPool { pool_id, amount })
    }
}
```

## **🎯 Phase 3: Update Core Components**

### **Step 3.1: Update Matching Engine Manager**
**Goal**: Make matching engine work with any AMM provider

**Files to Modify:**
- `crates/matching-engine/src/manager.rs`

**Implementation:**
```rust
// crates/matching-engine/src/manager.rs
pub struct MatchingManager<P: AMMProvider> {
    amm_provider: Arc<P>,
    order_books: HashMap<PoolId, OrderBook>,
}

impl<P: AMMProvider> MatchingManager<P> {
    pub async fn solve_pools(
        &self,
        limit: Vec<BookOrder>,
        searcher: Vec<OrderWithStorageData<TopOfBlockOrder>>,
        pools: HashMap<PoolId, (Address, Address, u16)>
    ) -> Result<(Vec<PoolSolution>, BundleGasDetails), MatchingEngineError> {
        // Get pool states from AMM provider
        let mut pool_states = HashMap::new();
        for (pool_id, (token0, token1, fee)) in pools {
            let pool_state = self.amm_provider.get_pool_state(pool_id).await?;
            pool_states.insert(pool_id, (token0, token1, pool_state, fee));
        }
        
        // Matching logic now works with generic pool states
        self.execute_matching(limit, searcher, pool_states).await
    }
}
```

Operational guidance:
- Fetch cadence: fetch `PoolState` once per consensus round (block), reuse across matching/quoting.
- Caching: memoize by `(pool_id, block)`; invalidate on block change or explicit pool update.
- Latency: all hot-path math calls are pure methods on `PoolStateInterface`; avoid I/O.

Pure-math snapshot guidance (minimal fields and helper functions)

- Uniswap V4 snapshot (minimal):
  - sqrt_price_x96 (current), current_tick, tick_spacing, fee, current_liquidity
  - initialized_ticks: HashMap<i32, TickInfo>, tick_bitmap: HashMap<i16, U256>

- Balancer V3 ReClamm snapshot:
  - Use `ReClammState` from `balancer-maths-rust` (balances, virtual params, fees, invariants)

Example pure functions (sketches):
```rust
// Uniswap
fn uni_price(s: &UniswapV4PoolState) -> SqrtPriceX96;
fn uni_execute_swap(s: &UniswapV4PoolState, amount: I256, direction: bool)
    -> eyre::Result<UniswapV4SwapResult>;

// ReClamm
fn bal_price(s: &ReClammState) -> ReClammPrice;
fn bal_execute_swap(s: &ReClammState, amount: I256, direction: bool)
    -> eyre::Result<BalancerV3SwapResult>;
```

### **Step 3.2: Update AMM Quoter**
**Goal**: Generic, pure-math, book-aware quoter with a single `get_quote` API (no Uniswap leaks)

**Files to Modify:**
- `crates/amm-quoter/src/lib.rs`

**Implementation:**
```rust
// Generic price and request/response types
pub enum Price { SqrtPriceX96(U256), ReClammPrice(U256) }

pub struct QuoteRequest {
    pub pool_id: PoolId,
    pub zero_for_one: bool,
    pub amount: I256,
    pub amount_is_input: bool,
    pub price_limit: Option<Price>,
}

pub struct QuoteResponse {
    pub amount_in: U256,
    pub amount_out: U256,
    pub end_price: Price,
    pub end_liquidity: u128,
    pub fee_charged: U256,
}

pub struct QuoterManager<P: AMMProvider, B: BlockSyncConsumer> {
    amm_provider: Arc<P>,
    block_sync: B,
    orders: Arc<OrderStorage>,
}

impl<P: AMMProvider, B: BlockSyncConsumer> QuoterManager<P, B> {
    pub async fn get_quote(&self, req: QuoteRequest) -> eyre::Result<QuoteResponse> {
        // 1) Fetch immutable snapshot (Uniswap baseline or ReClamm state)
        let snapshot = self.amm_provider.get_pool_state(req.pool_id).await?;
        // 2) Book-aware path using existing strategy over a generic book
        let OrderSet { limit, searcher } = self.orders.get_all_orders();
        let book = build_book(req.pool_id, Some(snapshot.clone()), limit);
        let end = BinarySearchStrategy::give_end_state(&book, searcher.into_iter().next());

        Ok(QuoteResponse {
            amount_in: end.input(),
            amount_out: end.output(),
            end_price: end.end_price(),
            end_liquidity: end.end_liquidity(),
            fee_charged: end.fee_amount(),
        })
    }
}
```

Refactor notes within `amm-quoter` (book-aware is required):
- Replace Uniswap-only `Slot0Update` usages with AMM-agnostic `QuoteUpdate { price: Price, liquidity: u128 }`; for Uniswap you may add `tick: Option<i32>` temporarily.
- Generalize `build_non_proposal_books` to accept generic `PoolStateInterface` instead of `BaselinePoolState`.
- Add `BinarySearchStrategy::give_end_state` accessor methods that do not require `tick`.
 - If a raw AMM-only quote is needed for diagnostics, gate it behind a feature flag and keep it out of the primary API path.

## **🎯 Phase 4: Multi-AMM Manager**

### **Step 4.1: Create Multi-AMM Manager**
**Goal**: Enable runtime selection between different AMM providers

**Files to Create:**
- `crates/amm-provider/src/managers/multi_amm_manager.rs`

**Implementation:**
```rust
// crates/amm-provider/src/managers/multi_amm_manager.rs
pub struct MultiAMMManager {
    providers: HashMap<AMMType, Box<dyn AMMProvider<PoolState = PoolState, SwapResult = SwapResult>>>,
    pool_registry: Arc<PoolRegistry>,
}

impl MultiAMMManager {
    pub async fn new(config: AMMConfig) -> Result<Self, AMMError> {
        let mut providers = HashMap::new();
        
        if config.uniswap_v4_enabled {
            let provider = UniswapV4Provider::new(config.uniswap_v4_config).await?;
            providers.insert(AMMType::UniswapV4, Box::new(provider));
        }
        
        if config.balancer_v3_enabled {
            let provider = BalancerV3Provider::new(config.balancer_v3_config).await?;
            providers.insert(AMMType::BalancerV3, Box::new(provider));
        }
        
        Ok(Self { providers, pool_registry: config.pool_registry })
    }
    
    pub async fn get_pool_state(&self, amm_type: AMMType, pool_id: PoolId) -> Result<PoolState, AMMError> {
        if let Some(provider) = self.providers.get(&amm_type) {
            provider.get_pool_state(pool_id).await.map(PoolState::from)
        } else {
            Err(AMMError::ProviderNotFound)
        }
    }
}
```

## **🎯 Phase 5: Update Main Application**

### **Step 5.1: Update Main Binary**
**Goal**: Wire everything together in the main application

**Files to Modify:**
- `bin/angstrom/src/main.rs`

**Implementation:**
```rust
// bin/angstrom/src/main.rs
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    
    // Create multi-AMM manager
    let multi_amm_manager = MultiAMMManager::new(config.amm_config).await?;
    
    // Create generic quoter manager
    let quoter_manager = QuoterManager::new(
        multi_amm_manager.clone(),
        config.quoter_config,
    ).await?;
    
    // Create generic matching engine
    let matching_engine = MatchingManager::new(
        multi_amm_manager.clone(),
        config.matching_config,
    ).await?;
    
    // Rest of application setup...
}
```

## **🎯 Phase 6: Testing & Validation**

### **Step 6.1: Comprehensive Test Suite**
**Goal**: Ensure all functionality works with both AMM types

**Files to Create:**
```
testing-tools/src/amm_provider_tests/
├── mod.rs
├── uniswap_v4_tests.rs
├── balancer_v3_tests.rs
├── multi_amm_tests.rs
└── integration_tests.rs
```

**Implementation:**
```rust
// testing-tools/src/amm_provider_tests/integration_tests.rs
#[tokio::test]
async fn test_multi_amm_workflow() {
    let multi_amm = MultiAMMManager::new(test_config()).await.unwrap();
    
    // Test Uniswap V4
    let uni_pool_state = multi_amm.get_pool_state(AMMType::UniswapV4, test_pool_id()).await.unwrap();
    let uni_swap_result = multi_amm.execute_swap(AMMType::UniswapV4, test_pool_id(), test_amount(), test_direction()).await.unwrap();
    
    // Test Balancer V3
    let balancer_pool_state = multi_amm.get_pool_state(AMMType::BalancerV3, test_pool_id()).await.unwrap();
    let balancer_swap_result = multi_amm.execute_swap(AMMType::BalancerV3, test_pool_id(), test_amount(), test_direction()).await.unwrap();
    
    // Verify both work
    assert!(uni_swap_result.amount_0_delta != 0);
    assert!(balancer_swap_result.amount_0_delta != 0);
}
```

## **📋 Implementation Checklist**

### **Phase 1: Foundation (Weeks 1-2)**
- [ ] Create generic pool state types
- [ ] Update matching engine to use generic types
- [ ] Update order pool types
- [ ] Create comprehensive tests for generic types

### **Phase 2: AMM Provider Layer (Weeks 3-4)**
- [ ] Create AMM provider traits
- [ ] Implement Uniswap V4 provider
- [ ] Implement Balancer V3 provider
- [ ] Create donation planning interfaces

### **Phase 3: Core Components (Weeks 5-6)**
- [ ] Update matching engine manager
- [ ] Update AMM quoter
- [ ] Update consensus components
- [ ] Update validation components

### **Phase 4: Multi-AMM Manager (Week 7)**
- [ ] Create multi-AMM manager
- [ ] Implement runtime AMM selection
- [ ] Add configuration management
- [ ] Create AMM switching logic

### **Phase 5: Integration (Week 8)**
- [ ] Update main application
- [ ] Update configuration system
- [ ] Update deployment scripts
- [ ] End-to-end testing

### **Phase 6: Testing & Validation (Weeks 9-10)**
- [ ] Comprehensive test suite
- [ ] Performance benchmarking
- [ ] Integration testing
- [ ] Documentation updates

## **🚨 Critical Success Factors**

1. **Incremental Migration**: Each phase must maintain backward compatibility
2. **Comprehensive Testing**: Every change must be thoroughly tested
3. **Performance**: No degradation in matching engine performance
4. **Donation Logic**: AMM-specific donation strategies must be preserved
5. **Type Safety**: Rust's type system must enforce AMM boundaries

## **📊 Expected Outcomes**

- **Generic Pool Abstraction**: All components work with any AMM type
- **Runtime AMM Selection**: Can switch between Uniswap V4 and Balancer V3
- **Preserved Functionality**: All existing features continue to work
- **Extensibility**: Easy to add new AMM providers in the future
- **Performance**: No significant performance degradation

This plan provides the detailed, step-by-step approach you requested, with clear handholding for each phase of the implementation.

---

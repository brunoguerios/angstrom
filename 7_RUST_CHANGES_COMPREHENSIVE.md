# **Multi-AMM Integration Implementation Plan**

## **📋 Overview**

This plan provides a detailed, incremental approach to implementing multi-AMM support (Uniswap V4 + Balancer V3) in the Angstrom codebase. Each step is designed as a small, reviewable PR that builds upon the previous work while maintaining full test coverage.

## **Donation Strategy Differences (Critical)**

- **Uniswap V4 (concentrated liquidity, ticks)**:
  - Supports multi-tick donations only (placing donations across ranges via ticks)
  - No direct "donate back to pool" mechanic
  - Donation impacts must be modeled via tick/range placements
- **Balancer V3 (fungible liquidity, no ticks)**:
  - Supports direct pool-level donation (redistributed proportionally to LPs)
  - No tick/range placement logic

The abstraction MUST expose donation as an AMM-specific operation with different execution plans for Uniswap vs Balancer.

## **🎯 Phase 1: Foundation & Abstraction Layer**

### **PR #1: Create AMM Provider Trait and Core Types**
**Goal**: Establish the foundational abstraction layer

**Files to Create/Modify**:
- crates/amm-provider/Cargo.toml
- crates/amm-provider/src/lib.rs
- crates/amm-provider/src/traits.rs (Core AMMProvider trait)
- crates/amm-provider/src/types.rs (Generic data structures)
- crates/amm-provider/src/errors.rs (AMM error handling)
- crates/amm-provider/src/providers/mod.rs
- crates/amm-provider/src/providers/uniswap_v4.rs (Uniswap V4 provider)
- crates/amm-provider/src/providers/balancer_v3.rs (Balancer V3 provider stub)

**Implementation**:
```rust
// crates/amm-provider/src/traits.rs
#[async_trait]
pub trait AMMProvider: Send + Sync + 'static {
    type PoolState: PoolStateTrait;
    type SwapParams: SwapParamsTrait;
    type SwapResult: SwapResultTrait;
    // Donation-related associated types
    type DonationRequest: DonationRequestTrait;
    type DonationPlan: DonationPlanTrait;
    type DonationResult: DonationResultTrait;
    
    async fn get_pool_state(&self, pool_id: PoolId) -> Result<Self::PoolState, AMMError>;
    async fn execute_swap(&self, params: Self::SwapParams) -> Result<Self::SwapResult, AMMError>;
    async fn compute_swap_amounts(&self, params: Self::SwapParams) -> Result<SwapAmounts, AMMError>;
    async fn subscribe_pool_updates(&self, pool_ids: Vec<PoolId>) -> Result<PoolUpdateStream, AMMError>;

    // AMM-specific donation planning & execution
    // - Uniswap: returns a multi-tick donation plan (ranges/ticks + size)
    // - Balancer: returns a pool-level donation plan
    async fn plan_donation(
        &self,
        pool_id: PoolId,
        request: Self::DonationRequest,
    ) -> Result<Self::DonationPlan, AMMError>;

    async fn execute_donation(
        &self,
        plan: Self::DonationPlan,
    ) -> Result<Self::DonationResult, AMMError>;
}

// crates/amm-provider/src/types.rs
pub struct GenericPoolState {
    pub pool_id: PoolId,
    pub sqrt_price_x96: U256,
    pub liquidity: u128,
    pub tick: i32,
    pub fee: u32,
    pub tick_spacing: i32,
    pub amm_specific_data: Vec<u8>,
}

pub struct GenericSwapParams {
    pub pool_id: PoolId,
    pub zero_for_one: bool,
    pub amount_specified: i256,
    pub sqrt_price_limit_x96: U256,
    pub recipient: Address,
}

pub struct GenericSwapResult {
    pub amount_0_delta: i256,
    pub amount_1_delta: i256,
    pub sqrt_price_x96: U256,
    pub liquidity: u128,
    pub tick: i32,
}

// Generic donation types that providers can map to AMM-specific plans
pub enum DonationKind {
    UniswapMultiTick,
    BalancerPool,
}

pub struct GenericDonationRequest {
    pub pool_id: PoolId,
    pub kind: DonationKind,
    pub amount_in: U256,
    // Optional constraints (e.g., max ticks to touch, price bounds, gas caps)
    pub constraints: Vec<u8>,
}

pub struct GenericDonationPlan {
    // Serialized or provider-specific encoded plan
    pub details: Vec<u8>,
}

pub struct GenericDonationResult {
    pub amount_executed: U256,
    pub gas_used: u64,
}
```

**Tests**:
- Unit tests for trait definitions
- Mock provider implementation tests
- Error handling tests
- Donation trait API tests (plan + execute)

**Validation**: All tests pass, no breaking changes to existing code

---

### **PR #2: Implement Uniswap V4 Provider**
**Goal**: Create the first concrete AMM provider implementation

**Files to Create/Modify**:
- crates/amm-provider/src/providers/uniswap_v4.rs
- crates/amm-provider/src/providers/balancer_v3.rs (stub)

**Implementation**:
```rust
// crates/amm-provider/src/providers/uniswap_v4.rs
pub struct UniswapV4Provider {
    pool_manager: Arc<UniswapPoolManager>,
    factory: Arc<V4PoolFactory>,
}

#[async_trait]
impl AMMProvider for UniswapV4Provider {
    type PoolState = UniswapV4PoolState;
    type SwapParams = UniswapV4SwapParams;
    type SwapResult = UniswapV4SwapResult;
    
    async fn get_pool_state(&self, pool_id: PoolId) -> Result<Self::PoolState, AMMError> {
        // Convert Uniswap V4 specific data to generic format
        let slot0 = self.pool_manager.get_slot0(pool_id).await?;
        let liquidity = self.pool_manager.get_pool_liquidity(pool_id).await?;
        
        Ok(UniswapV4PoolState {
            pool_id,
            sqrt_price_x96: slot0.sqrt_price_x96,
            liquidity,
            tick: slot0.tick,
            fee: slot0.fee,
            tick_spacing: slot0.tick_spacing,
            amm_specific_data: slot0.to_bytes(),
        })
    }
    
    // ... implement other trait methods
}

// Donation (Uniswap): plan multi-tick donation and execute via ticks/ranges
impl UniswapV4Provider {
    pub async fn plan_uni_donation(
        &self,
        pool_id: PoolId,
        req: GenericDonationRequest,
    ) -> eyre::Result<GenericDonationPlan> {
        // Convert amount into a set of tick ranges (respecting tick spacing, current price)
        // Serialize ranges to details for execution
        Ok(GenericDonationPlan { details: vec![] })
    }

    pub async fn execute_uni_donation(
        &self,
        plan: GenericDonationPlan,
    ) -> eyre::Result<GenericDonationResult> {
        // Decode ranges and place liquidity/donation at target ticks
        Ok(GenericDonationResult { amount_executed: U256::ZERO, gas_used: 0 })
    }
}
```

**Tests**:
- Integration tests with existing Uniswap V4 infrastructure
- Conversion tests between Uniswap V4 and generic formats
- Performance benchmarks
- Donation planning/execution tests (multi-tick): correctness vs expected tick ranges

**Validation**: All existing Uniswap V4 functionality works through the new provider

---

### **PR #3: Create Generic Pool Manager**
**Goal**: Abstract the pool management layer to work with any AMM provider

**Files to Create/Modify**:
- crates/amm-provider/src/managers/mod.rs
- crates/amm-provider/src/managers/generic_pool_manager.rs
- crates/amm-provider/src/managers/multi_amm_manager.rs

**Implementation**:
```rust
// crates/amm-provider/src/managers/generic_pool_manager.rs
pub struct GenericPoolManager<AMM: AMMProvider> {
    amm_provider: Arc<AMM>,
    pools: Arc<DashMap<PoolId, Arc<RwLock<GenericPool>>>>,
    pool_registry: Arc<PoolRegistry>,
}

impl<AMM: AMMProvider> GenericPoolManager<AMM> {
    pub async fn new(
        amm_provider: Arc<AMM>,
        pool_registry: Arc<PoolRegistry>,
    ) -> Self {
        // Initialize with AMM-agnostic pool management
    }
    
    pub async fn get_pool_state(&self, pool_id: PoolId) -> Result<AMM::PoolState, AMMError> {
        self.amm_provider.get_pool_state(pool_id).await
    }
    
    pub async fn execute_swap(&self, swap_params: AMM::SwapParams) -> Result<AMM::SwapResult, AMMError> {
        self.amm_provider.execute_swap(swap_params).await
    }
}
```

**Tests**:
- Generic pool manager tests with mock providers
- Integration tests with Uniswap V4 provider
- Performance comparison with existing pool manager

**Validation**: Generic pool manager works identically to existing Uniswap V4 pool manager

---

## **🎯 Phase 2: Update Core Components**

### **PR #4: Update AMM Quoter to Use Generic Provider**
**Goal**: Make the AMM quoter work with any AMM provider

**Files to Modify**:
- crates/amm-quoter/src/lib.rs
- crates/amm-quoter/src/generic_quoter.rs (new)

**Implementation**:
```rust
// crates/amm-quoter/src/generic_quoter.rs
pub struct GenericQuoterManager<AMM: AMMProvider, BlockSync: BlockSyncConsumer> {
    amm_provider: Arc<AMM>,
    cur_block: u64,
    seq_id: u16,
    block_sync: BlockSync,
    orders: Arc<OrderStorage>,
    // ... other fields
}

impl<AMM: AMMProvider, BlockSync: BlockSyncConsumer> GenericQuoterManager<AMM, BlockSync> {
    pub async fn get_quote(&self, quote_request: QuoteRequest) -> Result<QuoteResponse, AMMError> {
        let pool_state = self.amm_provider.get_pool_state(quote_request.pool_id).await?;
        let swap_amounts = self.amm_provider.compute_swap_amounts(quote_request.swap_params).await?;
        
        Ok(QuoteResponse {
            pool_state,
            swap_amounts,
            // ... other quote data
        })
    }
}
```

**Tests**:
- Generic quoter tests with mock providers
- Integration tests with Uniswap V4 provider
- Performance benchmarks

**Validation**: AMM quoter works identically with generic provider

---

### **PR #5: Update Matching Engine to Use Generic Provider**
**Goal**: Make the matching engine work with any AMM provider

**Files to Modify**:
- crates/matching-engine/src/manager.rs
- crates/matching-engine/src/generic_matcher.rs (new)

**Implementation**:
```rust
// crates/matching-engine/src/generic_matcher.rs
pub struct GenericMatchingEngine<AMM: AMMProvider> {
    amm_provider: Arc<AMM>,
    // ... other fields
}

impl<AMM: AMMProvider> GenericMatchingEngine<AMM> {
    pub async fn match_orders(&self, orders: Vec<Order>) -> Result<MatchResult, AMMError> {
        for order in &orders {
            let pool_state = self.amm_provider.get_pool_state(order.pool_id).await?;
            // ... matching logic using generic pool state
        }
        
        Ok(MatchResult {
            // ... match results
        })
    }
}
```

**Tests**:
- Generic matching engine tests
- Integration tests with Uniswap V4 provider
- Performance benchmarks

**Validation**: Matching engine works identically with generic provider

---

## **🎯 Phase 3: Balancer V3 Integration**

### **PR #6: Implement Balancer V3 Provider**
**Goal**: Create the Balancer V3 provider implementation

**Files to Create/Modify**:
- crates/amm-provider/src/providers/balancer_v3.rs
- crates/balancer-v3/Cargo.toml (new crate)
- crates/balancer-v3/src/lib.rs
- crates/balancer-v3/src/pool_manager.rs
- crates/balancer-v3/src/pool_factory.rs
- crates/balancer-v3/src/types.rs

**Implementation**:
```rust
// crates/amm-provider/src/providers/balancer_v3.rs
pub struct BalancerV3Provider {
    vault: Arc<BalancerVault>,
    pool_manager: Arc<BalancerPoolManager>,
}

#[async_trait]
impl AMMProvider for BalancerV3Provider {
    type PoolState = BalancerV3PoolState;
    type SwapParams = BalancerV3SwapParams;
    type SwapResult = BalancerV3SwapResult;
    
    async fn get_pool_state(&self, pool_id: PoolId) -> Result<Self::PoolState, AMMError> {
        // Convert Balancer V3 specific data to generic format
        let pool_data = self.pool_manager.get_pool_data(pool_id).await?;
        
        Ok(BalancerV3PoolState {
            pool_id,
            sqrt_price_x96: pool_data.sqrt_price_x96,
            liquidity: pool_data.liquidity,
            tick: pool_data.tick,
            fee: pool_data.fee,
            tick_spacing: pool_data.tick_spacing,
            amm_specific_data: pool_data.to_bytes(),
        })
    }
    
    // ... implement other trait methods
}

// Donation (Balancer): plan pool-level donation and execute via vault
impl BalancerV3Provider {
    pub async fn plan_balancer_donation(
        &self,
        pool_id: PoolId,
        req: GenericDonationRequest,
    ) -> eyre::Result<GenericDonationPlan> {
        // Balancer: donate back to pool; plan includes token, amount, and pool hook data
        Ok(GenericDonationPlan { details: vec![] })
    }

    pub async fn execute_balancer_donation(
        &self,
        plan: GenericDonationPlan,
    ) -> eyre::Result<GenericDonationResult> {
        // Call into vault/pool to donate; receives proportional redistribution
        Ok(GenericDonationResult { amount_executed: U256::ZERO, gas_used: 0 })
    }
}
```

**Tests**:
- Balancer V3 provider unit tests
- Integration tests with mock Balancer V3 infrastructure
- Conversion tests between Balancer V3 and generic formats
- Donation planning/execution tests (pool-level): exact donation accounting

**Validation**: Balancer V3 provider implements the AMMProvider trait correctly

---

### **PR #7: Add Multi-AMM Manager**
**Goal**: Enable runtime selection between different AMM providers

**Files to Create/Modify**:
- crates/amm-provider/src/managers/multi_amm_manager.rs
- crates/amm-provider/src/config.rs (new)

**Implementation**:
```rust
// crates/amm-provider/src/managers/multi_amm_manager.rs
pub struct MultiAMMManager {
    providers: HashMap<AMMType, Box<dyn AMMProvider>>,
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
        
        Ok(Self {
            providers,
            pool_registry: config.pool_registry,
        })
    }
    
    pub async fn get_provider(&self, amm_type: AMMType) -> Option<&dyn AMMProvider> {
        self.providers.get(&amm_type).map(|p| p.as_ref())
    }
    
    pub async fn execute_swap(&self, amm_type: AMMType, swap_params: GenericSwapParams) -> Result<GenericSwapResult, AMMError> {
        if let Some(provider) = self.get_provider(amm_type).await {
            // Convert generic params to AMM-specific format
            let amm_params = self.convert_to_amm_params(amm_type, swap_params)?;
            let result = provider.execute_swap(amm_params).await?;
            self.convert_from_amm_result(amm_type, result)
        } else {
            Err(AMMError::ProviderNotFound)
        }
    }
}
```

**Tests**:
- Multi-AMM manager tests
- Runtime AMM selection tests
- Configuration tests

**Validation**: Multi-AMM manager can switch between providers at runtime

---

## **🎯 Phase 4: Testing & Validation**

### **PR #8: Comprehensive Test Suite**
**Goal**: Add comprehensive testing for multi-AMM functionality

**Files to Create/Modify**:
- testing-tools/src/amm_provider_tests/mod.rs
- testing-tools/src/amm_provider_tests/uniswap_v4_tests.rs
- testing-tools/src/amm_provider_tests/balancer_v3_tests.rs
- testing-tools/src/amm_provider_tests/multi_amm_tests.rs
- testing-tools/src/amm_provider_tests/integration_tests.rs

**Implementation**:
```rust
// testing-tools/src/amm_provider_tests/integration_tests.rs
#[tokio::test]
async fn test_multi_amm_quoter() {
    let mut multi_amm = MultiAMMManager::new(test_config()).await.unwrap();
    
    // Test Uniswap V4 provider
    let uni_quote = multi_amm.get_quote(AMMType::UniswapV4, quote_request).await.unwrap();
    assert!(uni_quote.amount_out > 0);
    
    // Test Balancer V3 provider
    let balancer_quote = multi_amm.get_quote(AMMType::BalancerV3, quote_request).await.unwrap();
    assert!(balancer_quote.amount_out > 0);
    
    // Test provider switching
    let result = multi_amm.execute_swap(AMMType::UniswapV4, swap_params).await.unwrap();
    assert!(result.amount_0_delta != 0);
}

#[tokio::test]
async fn test_amm_provider_performance() {
    let uni_provider = UniswapV4Provider::new(test_config()).await.unwrap();
    let balancer_provider = BalancerV3Provider::new(test_config()).await.unwrap();
    
    let start = Instant::now();
    let uni_result = uni_provider.get_pool_state(test_pool_id()).await.unwrap();
    let uni_duration = start.elapsed();
    
    let start = Instant::now();
    let balancer_result = balancer_provider.get_pool_state(test_pool_id()).await.unwrap();
    let balancer_duration = start.elapsed();
    
    // Performance should be within acceptable range
    assert!(uni_duration.as_millis() < 100);
    assert!(balancer_duration.as_millis() < 100);
}
```

**Tests**:
- Unit tests for all AMM providers
- Integration tests for multi-AMM scenarios
- Performance benchmarks
- Error handling tests
- Configuration tests
- Donation tests:
  - Plan vs execute parity for Uniswap (multi-tick) and Balancer (pool-level)
  - ToB donation scheduling and correctness per AMM

**Validation**: All tests pass, performance is acceptable

---

### **PR #9: Update Test Infrastructure**
**Goal**: Update existing test infrastructure to support multi-AMM testing

**Files to Modify**:
- testing-tools/src/controllers/enviroments/mod.rs
- testing-tools/src/type_generator/amm.rs
- testing-tools/src/agents/mod.rs

**Implementation**:
```rust
// testing-tools/src/controllers/enviroments/multi_amm_testnet.rs
pub struct MultiAMMTestnet {
    uniswap_v4_provider: Arc<UniswapV4Provider>,
    balancer_v3_provider: Arc<BalancerV3Provider>,
    multi_amm_manager: Arc<MultiAMMManager>,
}

impl MultiAMMTestnet {
    pub async fn new(config: MultiAMMConfig) -> Result<Self, AMMError> {
        let uniswap_v4_provider = Arc::new(UniswapV4Provider::new(config.uniswap_v4_config).await?);
        let balancer_v3_provider = Arc::new(BalancerV3Provider::new(config.balancer_v3_config).await?);
        
        let multi_amm_manager = Arc::new(MultiAMMManager::new(config.multi_amm_config).await?);
        
        Ok(Self {
            uniswap_v4_provider,
            balancer_v3_provider,
            multi_amm_manager,
        })
    }
    
    pub async fn test_amm_switching(&self) -> Result<(), AMMError> {
        // Test switching between AMMs
        let uni_result = self.multi_amm_manager.execute_swap(AMMType::UniswapV4, test_swap_params()).await?;
        let balancer_result = self.multi_amm_manager.execute_swap(AMMType::BalancerV3, test_swap_params()).await?;
        
        // Both should succeed
        assert!(uni_result.amount_0_delta != 0);
        assert!(balancer_result.amount_0_delta != 0);
        
        Ok(())
    }
}
```

**Tests**:
- Multi-AMM testnet tests
- AMM switching tests
- Performance tests

**Validation**: Test infrastructure supports multi-AMM testing

---

## **🎯 Phase 5: Integration & Cleanup**

### **PR #10: Update Main Application**
**Goal**: Update the main application to use the new multi-AMM infrastructure

**Files to Modify**:
- bin/angstrom/src/main.rs
- crates/angstrom/src/components/mod.rs

**Implementation**:
```rust
// bin/angstrom/src/main.rs
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    
    let multi_amm_manager = MultiAMMManager::new(config.amm_config).await?;
    
    let quoter_manager = GenericQuoterManager::new(
        multi_amm_manager.clone(),
        config.quoter_config,
    ).await?;
    
    let matching_engine = GenericMatchingEngine::new(
        multi_amm_manager.clone(),
        config.matching_config,
    ).await?;
    
    // ... rest of application setup
}
```

**Tests**:
- Application integration tests
- Configuration tests
- End-to-end tests

**Validation**: Main application works with multi-AMM infrastructure

---

### **PR #11: Performance Optimization**
**Goal**: Optimize performance for multi-AMM scenarios

**Files to Modify**:
- crates/amm-provider/src/managers/multi_amm_manager.rs
- crates/amm-provider/src/cache.rs (new)

**Implementation**:
```rust
// crates/amm-provider/src/cache.rs
pub struct AMMCache {
    pool_state_cache: Arc<DashMap<PoolId, (PoolState, Instant)>>,
    cache_ttl: Duration,
}

impl AMMCache {
    pub async fn get_pool_state(&self, pool_id: PoolId, provider: &dyn AMMProvider) -> Result<PoolState, AMMError> {
        if let Some((cached_state, timestamp)) = self.pool_state_cache.get(&pool_id) {
            if timestamp.elapsed() < self.cache_ttl {
                return Ok(cached_state.clone());
            }
        }
        
        let fresh_state = provider.get_pool_state(pool_id).await?;
        self.pool_state_cache.insert(pool_id, (fresh_state.clone(), Instant::now()));
        Ok(fresh_state)
    }
}
```

**Tests**:
- Cache performance tests
- Memory usage tests
- Cache invalidation tests

**Validation**: Performance is optimized for multi-AMM scenarios

---

### **PR #12: Documentation & Examples**
**Goal**: Add comprehensive documentation and examples

**Files to Create**:
- docs/amm-provider/README.md
- docs/amm-provider/getting-started.md
- docs/amm-provider/examples/basic-usage.rs
- docs/amm-provider/examples/multi-amm.rs
- docs/amm-provider/examples/custom-provider.rs
- docs/amm-provider/api-reference.md

**Implementation**:
```rust
// docs/amm-provider/examples/basic-usage.rs
use amm_provider::{AMMProvider, UniswapV4Provider, BalancerV3Provider};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create Uniswap V4 provider
    let uni_provider = UniswapV4Provider::new(uni_config()).await?;
    
    // Get pool state
    let pool_state = uni_provider.get_pool_state(pool_id).await?;
    println!("Pool state: {:?}", pool_state);
    
    // Execute swap
    let swap_result = uni_provider.execute_swap(swap_params).await?;
    println!("Swap result: {:?}", swap_result);
    
    Ok(())
}
```

**Tests**:
- Documentation examples compile and run
- API reference is accurate
- Getting started guide works

**Validation**: Documentation is complete and accurate

---

## **🎯 Phase 6: Final Integration**

### **PR #13: Feature Flags & Configuration**
**Goal**: Add feature flags for AMM selection

**Files to Modify**:
- Cargo.toml (workspace)
- crates/amm-provider/Cargo.toml

**Implementation**:
```toml
# Cargo.toml
[features]
default = ["uniswap-v4"]
uniswap-v4 = ["amm-provider/uniswap-v4"]
balancer-v3 = ["amm-provider/balancer-v3"]
multi-amm = ["uniswap-v4", "balancer-v3"]
```

**Tests**:
- Feature flag tests
- Configuration tests
- Build tests

**Validation**: Feature flags work correctly

---

### **PR #14: Final Testing & Validation**
**Goal**: Comprehensive testing of the complete multi-AMM system

**Files to Create/Modify**:
- tests/integration/multi_amm_integration.rs
- tests/integration/performance_tests.rs
- tests/integration/end_to_end_tests.rs

**Implementation**:
```rust
// tests/integration/multi_amm_integration.rs
#[tokio::test]
async fn test_complete_multi_amm_workflow() {
    let testnet = MultiAMMTestnet::new(test_config()).await.unwrap();
    
    // Test order submission
    let order = create_test_order();
    testnet.submit_order(order).await.unwrap();
    
    // Test matching with Uniswap V4
    let uni_match = testnet.match_orders(AMMType::UniswapV4).await.unwrap();
    assert!(uni_match.matched_orders.len() > 0);
    
    // Test matching with Balancer V3
    let balancer_match = testnet.match_orders(AMMType::BalancerV3).await.unwrap();
    assert!(balancer_match.matched_orders.len() > 0);
    
    // Test AMM switching
    testnet.switch_amm(AMMType::UniswapV4).await.unwrap();
    let result = testnet.execute_orders().await.unwrap();
    assert!(result.success);
}
```

**Tests**:
- Complete integration tests
- Performance tests
- End-to-end tests
- Stress tests

**Validation**: Complete multi-AMM system works correctly

---

## **📊 Success Metrics**

### **Performance Metrics**:
- ✅ **Latency**: < 100ms for pool state queries
- ✅ **Throughput**: > 1000 orders/second
- ✅ **Memory Usage**: < 10% increase over single AMM
- ✅ **CPU Usage**: < 15% increase over single AMM

### **Economic Metrics**:
- ✅ **LVR Reduction (Top-of-Block Donations)**:
  - Uniswap V4: multi-tick donation reduces LVR vs baseline by X% (target configurable)
  - Balancer V3: pool-level donation reduces LVR vs baseline by Y% (target configurable)

### **Functionality Metrics**:
- ✅ **Test Coverage**: > 90% for all new code
- ✅ **Integration Tests**: All pass
- ✅ **Performance Tests**: All benchmarks pass
- ✅ **Error Handling**: All error cases covered

### **Code Quality Metrics**:
- ✅ **Documentation**: All public APIs documented
- ✅ **Examples**: Working examples for all features
- ✅ **Type Safety**: No unsafe code
- ✅ **Error Handling**: Comprehensive error handling

## **🔄 Review Process**

### **For Each PR**:
1. **Code Review**: Team reviews implementation
2. **Test Review**: All tests must pass
3. **Performance Review**: Performance benchmarks must pass
4. **Documentation Review**: Documentation must be complete
5. **Integration Review**: Integration tests must pass

### **Milestone Reviews**:
- **Phase 1 Complete**: Foundation layer working
- **Phase 2 Complete**: Core components updated
- **Phase 3 Complete**: Balancer V3 integrated
- **Phase 4 Complete**: Testing complete
- **Phase 5 Complete**: Integration complete
- **Phase 6 Complete**: Final system working

This plan provides a clear, incremental path to implementing multi-AMM support while maintaining code quality and allowing for team review at each step.
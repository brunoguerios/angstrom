# Rust Infrastructure Changes for Multi-AMM Support (Revised)

## **Overview**

This document outlines the conceptual changes needed in the Rust infrastructure to support multiple AMMs (Uniswap V4 and Balancer V3) while leveraging Balancer's existing abstraction for multiple pool types.

**🚨 KEY INSIGHT**: Balancer V3's maths crate already handles multiple pool types (Weighted, Stable, ReClamm, etc.) through a unified `PoolState` enum. We can leverage this existing abstraction instead of building our own.

## **🏗️ Current Architecture Analysis**

### **Current State:**
```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Order Pool    │    │ Matching Engine │    │   Validation    │
│                 │    │                 │    │                 │
│ - Generic       │    │ - Generic       │    │ - Generic       │
│ - AMM-agnostic  │    │ - AMM-agnostic  │    │ - AMM-agnostic  │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         └───────────────────────┼───────────────────────┘
                                 │
                    ┌─────────────────┐
                    │ Uniswap V4 Crate│
                    │                 │
                    │ - Pool Manager  │
                    │ - Pool Factory  │
                    │ - Enhanced Pool │
                    └─────────────────┘
```

### **Problem:**
- The `uniswap-v4` crate is tightly coupled to Uniswap V4
- AMM-specific logic is scattered throughout the codebase
- No abstraction layer for different AMM implementations

## **🎯 Target Architecture (Revised)**

### **Desired State:**
```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Order Pool    │    │ Matching Engine │    │   Validation    │
│                 │    │                 │    │                 │
│ - Generic       │    │ - Generic       │    │ - Generic       │
│ - AMM-agnostic  │    │ - AMM-agnostic  │    │ - AMM-agnostic  │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         └───────────────────────┼───────────────────────┘
                                 │
                    ┌─────────────────┐
                    │  AMM Provider   │
                    │   Trait Layer   │
                    │                 │
                    │ - Pool State    │
                    │ - Swap Logic    │
                    │ - Liquidity     │
                    └─────────────────┘
                                 │
                    ┌─────────────────┐
                    │  AMM Providers  │
                    │                 │
                    │ ┌─────────────┐ │
                    │ │Uniswap V4   │ │
                    │ │Provider     │ │
                    │ └─────────────┘ │
                    │ ┌─────────────┐ │
                    │ │Balancer V3  │ │
                    │ │Provider     │ │
                    │ │(All Pool    │ │
                    │ │ Types)      │ │
                    │ └─────────────┘ │
                    └─────────────────┘
                                 │
                    ┌─────────────────┐
                    │ Balancer V3     │
                    │ Maths Crate     │
                    │                 │
                    │ - PoolState     │
                    │ - Swap Logic    │
                    │ - All Pool      │
                    │   Types         │
                    └─────────────────┘
```

## **🔧 Key Changes Required (Simplified)**

### **1. AMM Abstraction Layer**

#### **A. Core AMM Provider Trait (Simplified)**
```rust
#[async_trait]
pub trait AMMProvider: Send + Sync + 'static {
    // Pool state management
    async fn get_pool_state(&self, pool_id: PoolId) -> Result<GenericPoolState, AMMError>;
    async fn get_pool_liquidity(&self, pool_id: PoolId) -> Result<U256, AMMError>;
    
    // Swap execution
    async fn execute_swap(&self, swap_params: GenericSwapParams) -> Result<GenericSwapResult, AMMError>;
    async fn compute_swap_amounts(&self, swap_params: GenericSwapParams) -> Result<(U256, U256), AMMError>;
    
    // Asset management
    async fn take_assets(&self, assets: Vec<AssetRequest>) -> Result<(), AMMError>;
    async fn settle_assets(&self, assets: Vec<AssetSettlement>) -> Result<(), AMMError>;
    
    // Event handling
    async fn subscribe_pool_updates(&self, pool_ids: Vec<PoolId>) -> Result<PoolUpdateStream, AMMError>;
}
```

#### **B. Generic Data Structures (Simplified)**
```rust
// Generic pool state that works across AMMs
pub struct GenericPoolState {
    pub pool_id: PoolId,
    pub pool_type: PoolType,
    pub tokens: Vec<Address>,
    pub balances: Vec<U256>,
    pub fee: U256,
    pub total_supply: U256,
    
    // AMM-specific data stored as generic bytes
    pub amm_specific_data: Vec<u8>,
}

pub enum PoolType {
    UniswapV4,
    BalancerV3,  // Covers all Balancer pool types
}

// Generic swap parameters
pub struct GenericSwapParams {
    pub pool_id: PoolId,
    pub token_in: Address,
    pub token_out: Address,
    pub amount_in: U256,
    pub amount_out_min: U256,
    pub swap_kind: SwapKind,
}

pub enum SwapKind {
    GivenIn,   // Amount in specified
    GivenOut,  // Amount out specified
}
```

### **2. AMM Provider Implementations**

#### **A. Uniswap V4 Provider (Unchanged)**
```rust
pub struct UniswapV4Provider {
    pool_manager: Arc<UniswapPoolManager>,
    factory: Arc<V4PoolFactory>,
    // ... Uniswap V4 specific fields
}

#[async_trait]
impl AMMProvider for UniswapV4Provider {
    async fn get_pool_state(&self, pool_id: PoolId) -> Result<GenericPoolState, AMMError> {
        // Convert Uniswap V4 specific data to generic PoolState
        let slot0 = self.pool_manager.get_slot0(pool_id).await?;
        let liquidity = self.pool_manager.get_pool_liquidity(pool_id).await?;
        
        Ok(GenericPoolState {
            pool_id,
            pool_type: PoolType::UniswapV4,
            tokens: self.get_tokens(pool_id).await?,
            balances: self.get_balances(pool_id).await?,
            fee: U256::from(slot0.fee),
            total_supply: U256::from(liquidity),
            amm_specific_data: slot0.to_bytes(), // Store Uniswap V4 specific data
        })
    }
    
    // ... implement other trait methods
}
```

#### **B. Balancer V3 Provider (Leverages Balancer's Abstraction)**
```rust
pub struct BalancerV3Provider {
    // Use Balancer's maths crate directly
    balancer_maths: Arc<BalancerMaths>,
    vault: Arc<BalancerVault>,
    // ... Balancer V3 specific fields
}

#[async_trait]
impl AMMProvider for BalancerV3Provider {
    async fn get_pool_state(&self, pool_id: PoolId) -> Result<GenericPoolState, AMMError> {
        // Get Balancer pool state (handles all pool types automatically)
        let balancer_pool_state = self.balancer_maths.get_pool_state(pool_id).await?;
        
        // Convert to generic format
        Ok(GenericPoolState {
            pool_id,
            pool_type: PoolType::BalancerV3,
            tokens: balancer_pool_state.base().tokens.iter().map(|t| t.parse().unwrap()).collect(),
            balances: balancer_pool_state.base().balances_live_scaled_18.iter().map(|b| U256::from_bigint(b)).collect(),
            fee: U256::from_bigint(&balancer_pool_state.base().swap_fee),
            total_supply: U256::from_bigint(&balancer_pool_state.base().total_supply),
            amm_specific_data: balancer_pool_state.to_bytes(), // Store all Balancer pool type data
        })
    }
    
    async fn execute_swap(&self, swap_params: GenericSwapParams) -> Result<GenericSwapResult, AMMError> {
        // Convert to Balancer format
        let balancer_pool_state = self.balancer_maths.get_pool_state(swap_params.pool_id).await?;
        let balancer_swap_params = self.convert_to_balancer_params(swap_params)?;
        
        // Let Balancer handle all pool type logic internally
        let result = self.balancer_maths.execute_swap(balancer_pool_state, balancer_swap_params)?;
        
        // Convert back to generic format
        Ok(self.convert_from_balancer_result(result))
    }
    
    // ... implement other trait methods
}
```

### **3. Balancer V3 Integration (Leveraging Existing Abstraction)**

#### **A. Dependencies**
```toml
# Cargo.toml
[dependencies]
# Use Balancer's maths crate directly
balancer-maths-rust = { path = "../../Development/Balancer/balancer-maths/rust" }

[features]
default = ["uniswap-v4"]
uniswap-v4 = ["dep:uniswap-v4"]
balancer-v3 = ["dep:balancer-maths-rust"]
```

#### **B. Balancer Pool Type Support (Automatic)**
```rust
// Balancer V3 supports all these pool types automatically
pub enum BalancerPoolType {
    Weighted,           // Standard weighted pools
    Stable,             // Stable pools
    GyroECLP,           // Gyro ECLP pools
    ReClamm,            // High priority for Angstrom
    ReClammV2,          // ReClamm V2
    QuantAmm,           // QuantAMM pools
    LiquidityBootstrapping, // LBP pools
}

// No need to implement each pool type - Balancer handles it!
impl BalancerV3Provider {
    pub async fn get_pool_type(&self, pool_id: PoolId) -> Result<BalancerPoolType, AMMError> {
        let pool_state = self.balancer_maths.get_pool_state(pool_id).await?;
        Ok(match pool_state {
            PoolState::Weighted(_) => BalancerPoolType::Weighted,
            PoolState::Stable(_) => BalancerPoolType::Stable,
            PoolState::ReClamm(_) => BalancerPoolType::ReClamm,
            PoolState::ReClammV2(_) => BalancerPoolType::ReClammV2,
            // ... all other pool types
        })
    }
}
```

### **4. Generic Pool Manager (Simplified)**

#### **A. Multi-AMM Pool Manager**
```rust
pub struct GenericPoolManager<AMM: AMMProvider> {
    amm_provider: Arc<AMM>,
    pools: Arc<DashMap<PoolId, Arc<RwLock<GenericPool>>>>,
    pool_registry: Arc<PoolRegistry>,
    // ... other fields
}

impl<AMM: AMMProvider> GenericPoolManager<AMM> {
    pub async fn new(
        amm_provider: Arc<AMM>,
        pool_registry: Arc<PoolRegistry>,
        // ... other params
    ) -> Self {
        // Initialize with AMM-agnostic pool management
    }
    
    pub async fn get_pool_state(&self, pool_id: PoolId) -> Result<GenericPoolState, AMMError> {
        // Delegate to AMM provider
        self.amm_provider.get_pool_state(pool_id).await
    }
    
    pub async fn execute_swap(&self, swap_params: GenericSwapParams) -> Result<GenericSwapResult, AMMError> {
        // Delegate to AMM provider
        self.amm_provider.execute_swap(swap_params).await
    }
    
    // ... other methods
}
```

### **5. Updated Components**

#### **A. AMM Quoter (Simplified)**
```rust
pub struct GenericQuoterManager<AMM: AMMProvider> {
    amm_provider: Arc<AMM>,
    pools: Arc<DashMap<PoolId, Arc<RwLock<GenericPool>>>>,
    // ... other fields
}

impl<AMM: AMMProvider> GenericQuoterManager<AMM> {
    pub async fn get_quote(&self, quote_request: QuoteRequest) -> Result<QuoteResponse, AMMError> {
        // Use AMM provider to get pricing information
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

#### **B. Matching Engine (Simplified)**
```rust
pub struct GenericMatchingEngine<AMM: AMMProvider> {
    amm_provider: Arc<AMM>,
    // ... other fields
}

impl<AMM: AMMProvider> GenericMatchingEngine<AMM> {
    pub async fn match_orders(&self, orders: Vec<Order>) -> Result<MatchResult, AMMError> {
        // Use AMM provider for pricing and execution
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

### **6. Configuration and Feature Flags**

#### **A. Feature-based AMM Selection**
```rust
// Cargo.toml
[features]
default = ["uniswap-v4"]
uniswap-v4 = ["dep:uniswap-v4"]
balancer-v3 = ["dep:balancer-maths-rust"]

// Runtime AMM selection
pub enum AMMType {
    UniswapV4,
    BalancerV3,  // Covers all Balancer pool types
}

pub struct AMMConfig {
    pub amm_type: AMMType,
    pub provider_config: ProviderConfig,
}

pub async fn create_amm_provider(config: AMMConfig) -> Result<Box<dyn AMMProvider>, AMMError> {
    match config.amm_type {
        AMMType::UniswapV4 => {
            let provider = UniswapV4Provider::new(config.provider_config).await?;
            Ok(Box::new(provider))
        }
        AMMType::BalancerV3 => {
            let provider = BalancerV3Provider::new(config.provider_config).await?;
            Ok(Box::new(provider))
        }
    }
}
```

#### **B. Multi-AMM Support**
```rust
pub struct MultiAMMManager {
    providers: HashMap<AMMType, Box<dyn AMMProvider>>,
    // ... other fields
}

impl MultiAMMManager {
    pub async fn get_provider(&self, amm_type: AMMType) -> Option<&dyn AMMProvider> {
        self.providers.get(&amm_type).map(|p| p.as_ref())
    }
    
    pub async fn execute_swap(&self, amm_type: AMMType, swap_params: GenericSwapParams) -> Result<GenericSwapResult, AMMError> {
        if let Some(provider) = self.get_provider(amm_type).await {
            provider.execute_swap(swap_params).await
        } else {
            Err(AMMError::ProviderNotFound)
        }
    }
}
```

## **🔄 Migration Strategy (Simplified)**

### **Phase 1: Create Abstraction Layer**
1. **Define AMM Provider trait** with simplified interface
2. **Create generic data structures** for pool state, swaps, etc.
3. **Implement Uniswap V4 provider** as first implementation

### **Phase 2: Integrate Balancer V3**
1. **Add Balancer V3 maths crate** as dependency
2. **Implement Balancer V3 provider** that leverages Balancer's abstraction
3. **Test with all Balancer pool types** (especially ReClamm)

### **Phase 3: Refactor Existing Components**
1. **Update Pool Manager** to use generic AMM provider
2. **Update AMM Quoter** to work with any AMM provider
3. **Update Matching Engine** to use generic pool state

### **Phase 4: Cleanup and Optimization**
1. **Remove direct Uniswap V4 dependencies** from generic components
2. **Optimize performance** for multi-AMM scenarios
3. **Add comprehensive testing** for both AMMs

## **📊 Benefits of This Approach (Enhanced)**

### **1. Leverage Balancer's Expertise**
- ✅ **All Balancer pool types** supported automatically
- ✅ **ReClamm support** comes "for free"
- ✅ **Future Balancer pool types** supported without changes
- ✅ **Balancer's battle-tested math** for all pool types

### **2. Simplified Architecture**
- ✅ **Less code** to maintain
- ✅ **Fewer bugs** from custom implementations
- ✅ **Faster development** by leveraging existing work
- ✅ **Better reliability** from using official implementations

### **3. Modularity**
- ✅ **Separation of Concerns**: Each AMM has its own provider
- ✅ **Easy Extension**: Adding new AMMs requires only implementing the trait
- ✅ **Independent Testing**: Each AMM provider can be tested in isolation

### **4. Maintainability**
- ✅ **Single Interface**: All components work with the same AMM interface
- ✅ **Reduced Complexity**: Generic components don't need to know AMM specifics
- ✅ **Easier Debugging**: Clear separation between generic and AMM-specific logic

## **🎯 Key Considerations (Updated)**

### **1. Balancer V3 Integration**
- **Challenge**: Integrating Balancer's maths crate with Angstrom's architecture
- **Solution**: Create wrapper that implements AMM Provider trait

### **2. Pool Type Support**
- **Challenge**: Supporting all Balancer pool types
- **Solution**: Let Balancer handle it - no additional work needed

### **3. Performance Optimization**
- **Challenge**: Maintaining performance with abstraction layer
- **Solution**: Use async traits and efficient data structures

### **4. Testing Strategy**
- **Challenge**: Testing multiple AMM implementations
- **Solution**: Comprehensive test suite with mock AMM providers

## **🏁 Conclusion (Revised)**

The revised approach is **significantly simpler** and **more robust** than the original plan:

**Key Improvements:**
- ✅ **Leverage Balancer's existing abstraction** for multiple pool types
- ✅ **ReClamm support** immediately available
- ✅ **All Balancer pool types** supported automatically
- ✅ **Much less code** to implement and maintain
- ✅ **Better reliability** from using official implementations

**Estimated effort**: 2-4 months for full Balancer V3 integration (reduced from 3-6 months)
**Risk level**: Low (leverage existing, tested implementations)
**Recommended approach**: Integrate Balancer V3 maths crate directly while maintaining clean AMM abstraction

This architecture provides a clean, maintainable, and extensible foundation for supporting multiple AMMs while leveraging the expertise of each AMM's official implementations.

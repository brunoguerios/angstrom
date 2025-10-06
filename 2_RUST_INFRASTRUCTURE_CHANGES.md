# Rust Infrastructure Changes for Multi-AMM Support

## **Overview**

This document outlines the conceptual changes needed in the Rust infrastructure to support multiple AMMs (Uniswap V4 and Balancer V3) while maintaining the existing Angstrom network architecture.

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

## **🎯 Target Architecture**

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
                    │ └─────────────┘ │
                    └─────────────────┘
```

## **🔧 Key Changes Required**

### **1. AMM Abstraction Layer**

#### **A. Core AMM Provider Trait**
```rust
#[async_trait]
pub trait AMMProvider: Send + Sync + 'static {
    // Pool state management
    async fn get_pool_state(&self, pool_id: PoolId) -> Result<PoolState, AMMError>;
    async fn get_pool_liquidity(&self, pool_id: PoolId) -> Result<u128, AMMError>;
    
    // Swap execution
    async fn execute_swap(&self, swap_params: SwapParams) -> Result<SwapResult, AMMError>;
    async fn compute_swap_amounts(&self, swap_params: SwapParams) -> Result<SwapAmounts, AMMError>;
    
    // Position management
    async fn get_position_liquidity(&self, pool_id: PoolId, position_key: PositionKey) -> Result<u128, AMMError>;
    async fn get_position_rewards(&self, pool_id: PoolId, position_key: PositionKey) -> Result<u128, AMMError>;
    
    // Asset management
    async fn take_assets(&self, assets: Vec<AssetRequest>) -> Result<(), AMMError>;
    async fn settle_assets(&self, assets: Vec<AssetSettlement>) -> Result<(), AMMError>;
    
    // Event handling
    async fn subscribe_pool_updates(&self, pool_ids: Vec<PoolId>) -> Result<PoolUpdateStream, AMMError>;
}
```

#### **B. Common Data Structures**
```rust
// Generic pool state that works across AMMs
pub struct PoolState {
    pub pool_id: PoolId,
    pub sqrt_price_x96: U256,
    pub liquidity: u128,
    pub tick: i32,
    pub fee: u32,
    pub tick_spacing: i32,
    // AMM-specific data stored as generic bytes
    pub amm_specific_data: Vec<u8>,
}

// Generic swap parameters
pub struct SwapParams {
    pub pool_id: PoolId,
    pub zero_for_one: bool,
    pub amount_specified: i256,
    pub sqrt_price_limit_x96: U256,
    pub recipient: Address,
}

// Generic swap result
pub struct SwapResult {
    pub amount_0_delta: i256,
    pub amount_1_delta: i256,
    pub sqrt_price_x96: U256,
    pub liquidity: u128,
    pub tick: i32,
}
```

### **2. AMM Provider Implementations**

#### **A. Uniswap V4 Provider**
```rust
pub struct UniswapV4Provider {
    pool_manager: Arc<UniswapPoolManager>,
    factory: Arc<V4PoolFactory>,
    // ... Uniswap V4 specific fields
}

#[async_trait]
impl AMMProvider for UniswapV4Provider {
    async fn get_pool_state(&self, pool_id: PoolId) -> Result<PoolState, AMMError> {
        // Convert Uniswap V4 specific data to generic PoolState
        let slot0 = self.pool_manager.get_slot0(pool_id).await?;
        let liquidity = self.pool_manager.get_pool_liquidity(pool_id).await?;
        
        Ok(PoolState {
            pool_id,
            sqrt_price_x96: slot0.sqrt_price_x96,
            liquidity,
            tick: slot0.tick,
            fee: slot0.fee,
            tick_spacing: slot0.tick_spacing,
            amm_specific_data: slot0.to_bytes(), // Store Uniswap V4 specific data
        })
    }
    
    async fn execute_swap(&self, swap_params: SwapParams) -> Result<SwapResult, AMMError> {
        // Convert generic params to Uniswap V4 specific format
        let uni_swap_params = self.convert_to_uniswap_params(swap_params)?;
        let result = self.pool_manager.execute_swap(uni_swap_params).await?;
        
        // Convert Uniswap V4 result to generic format
        Ok(self.convert_from_uniswap_result(result))
    }
    
    // ... implement other trait methods
}
```

#### **B. Balancer V3 Provider**
```rust
pub struct BalancerV3Provider {
    vault: Arc<BalancerVault>,
    pool_manager: Arc<BalancerPoolManager>,
    // ... Balancer V3 specific fields
}

#[async_trait]
impl AMMProvider for BalancerV3Provider {
    async fn get_pool_state(&self, pool_id: PoolId) -> Result<PoolState, AMMError> {
        // Convert Balancer V3 specific data to generic PoolState
        let pool_data = self.pool_manager.get_pool_data(pool_id).await?;
        
        Ok(PoolState {
            pool_id,
            sqrt_price_x96: pool_data.sqrt_price_x96,
            liquidity: pool_data.liquidity,
            tick: pool_data.tick,
            fee: pool_data.fee,
            tick_spacing: pool_data.tick_spacing,
            amm_specific_data: pool_data.to_bytes(), // Store Balancer V3 specific data
        })
    }
    
    async fn execute_swap(&self, swap_params: SwapParams) -> Result<SwapResult, AMMError> {
        // Convert generic params to Balancer V3 specific format
        let balancer_swap_params = self.convert_to_balancer_params(swap_params)?;
        let result = self.pool_manager.execute_swap(balancer_swap_params).await?;
        
        // Convert Balancer V3 result to generic format
        Ok(self.convert_from_balancer_result(result))
    }
    
    // ... implement other trait methods
}
```

### **3. Generic Pool Manager**

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
    
    pub async fn get_pool_state(&self, pool_id: PoolId) -> Result<PoolState, AMMError> {
        // Delegate to AMM provider
        self.amm_provider.get_pool_state(pool_id).await
    }
    
    pub async fn execute_swap(&self, swap_params: SwapParams) -> Result<SwapResult, AMMError> {
        // Delegate to AMM provider
        self.amm_provider.execute_swap(swap_params).await
    }
    
    // ... other methods
}
```

### **4. Updated Components**

#### **A. AMM Quoter**
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

#### **B. Matching Engine**
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

### **5. Configuration and Feature Flags**

#### **A. Feature-based AMM Selection**
```rust
// Cargo.toml
[features]
default = ["uniswap-v4"]
uniswap-v4 = ["dep:uniswap-v4"]
balancer-v3 = ["dep:balancer-v3"]

// Runtime AMM selection
pub enum AMMType {
    UniswapV4,
    BalancerV3,
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
    
    pub async fn execute_swap(&self, amm_type: AMMType, swap_params: SwapParams) -> Result<SwapResult, AMMError> {
        if let Some(provider) = self.get_provider(amm_type).await {
            provider.execute_swap(swap_params).await
        } else {
            Err(AMMError::ProviderNotFound)
        }
    }
}
```

## **🔄 Migration Strategy**

### **Phase 1: Create Abstraction Layer**
1. **Define AMM Provider trait** with common interface
2. **Create generic data structures** for pool state, swaps, etc.
3. **Implement Uniswap V4 provider** as first implementation

### **Phase 2: Refactor Existing Components**
1. **Update Pool Manager** to use generic AMM provider
2. **Update AMM Quoter** to work with any AMM provider
3. **Update Matching Engine** to use generic pool state

### **Phase 3: Add Balancer V3 Support**
1. **Implement Balancer V3 provider**
2. **Add Balancer V3 specific data conversion**
3. **Test multi-AMM functionality**

### **Phase 4: Cleanup and Optimization**
1. **Remove direct Uniswap V4 dependencies** from generic components
2. **Optimize performance** for multi-AMM scenarios
3. **Add comprehensive testing** for both AMMs

## **📊 Benefits of This Approach**

### **1. Modularity**
- ✅ **Separation of Concerns**: Each AMM has its own provider implementation
- ✅ **Easy Extension**: Adding new AMMs requires only implementing the trait
- ✅ **Independent Testing**: Each AMM provider can be tested in isolation

### **2. Maintainability**
- ✅ **Single Interface**: All components work with the same AMM interface
- ✅ **Reduced Complexity**: Generic components don't need to know AMM specifics
- ✅ **Easier Debugging**: Clear separation between generic and AMM-specific logic

### **3. Performance**
- ✅ **Type Safety**: Compile-time guarantees for AMM provider implementations
- ✅ **Efficient Delegation**: Direct calls to AMM-specific implementations
- ✅ **Minimal Overhead**: Generic layer adds minimal runtime cost

### **4. Flexibility**
- ✅ **Runtime Selection**: Can choose AMM provider at runtime
- ✅ **Feature Flags**: Can compile with specific AMM support
- ✅ **Incremental Rollout**: Can add AMMs without affecting existing functionality

## **🎯 Key Considerations**

### **1. Data Conversion Overhead**
- **Challenge**: Converting between AMM-specific and generic data formats
- **Solution**: Efficient serialization/deserialization with minimal allocations

### **2. Error Handling**
- **Challenge**: Different AMMs have different error types
- **Solution**: Generic error enum that can represent all AMM-specific errors

### **3. Performance Optimization**
- **Challenge**: Maintaining performance with abstraction layer
- **Solution**: Use async traits and efficient data structures

### **4. Testing Strategy**
- **Challenge**: Testing multiple AMM implementations
- **Solution**: Comprehensive test suite with mock AMM providers

This architecture provides a clean, maintainable, and extensible foundation for supporting multiple AMMs while preserving the existing Angstrom network functionality.

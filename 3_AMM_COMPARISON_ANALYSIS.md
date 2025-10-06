# AMM Comparison Analysis: Uniswap V4 vs Balancer V3

## **Overview**

This document analyzes the key differences between Uniswap V4 and Balancer V3 based on the Balancer V3 maths crate, identifying common patterns and AMM-specific concepts that need to be abstracted in the multi-AMM architecture.

## **🏗️ Core Architectural Differences**

### **1. Liquidity Model**

#### **Uniswap V4:**
```rust
// Non-fungible liquidity positions
pub struct Position {
    pub liquidity: u128,
    pub tick_lower: i24,
    pub tick_upper: i24,
    pub fee_growth_inside_0_last_x128: u256,
    pub fee_growth_inside_1_last_x128: u256,
}

// Tick-based liquidity distribution
pub struct Tick {
    pub liquidity_gross: u128,
    pub liquidity_net: i128,
    pub fee_growth_outside_0_x128: u256,
    pub fee_growth_outside_1_x128: u256,
}
```

#### **Balancer V3:**
```rust
// Fungible liquidity (LP tokens)
pub struct BasePoolState {
    pub total_supply: BigInt,  // BPT (Balancer Pool Tokens)
    pub balances_live_scaled_18: Vec<BigInt>,
    pub scaling_factors: Vec<BigInt>,
    pub token_rates: Vec<BigInt>,
}

// No ticks - weight-based pricing
pub struct WeightedState {
    pub weights: Vec<BigInt>,  // Normalized weights (scaled 18)
}
```

**Key Difference**: Uniswap V4 uses **non-fungible positions with tick ranges**, while Balancer V3 uses **fungible LP tokens with weight-based pricing**.

### **2. Price Discovery Mechanism**

#### **Uniswap V4:**
```rust
// Tick-based price discovery
pub struct Slot0 {
    pub sqrt_price_x96: U256,  // Current sqrt price in X96 format
    pub tick: i32,             // Current tick
    pub fee: u16,              // Pool fee
    pub tick_spacing: i16,     // Tick spacing
}

// Price calculation from ticks
pub fn get_sqrt_ratio_at_tick(tick: i32) -> U256 {
    // Complex mathematical formula based on tick
}
```

#### **Balancer V3:**
```rust
// Weight-based price discovery
pub fn compute_out_given_exact_in(
    balance_in: &BigInt,
    weight_in: &BigInt,
    balance_out: &BigInt,
    weight_out: &BigInt,
    amount_in: &BigInt,
) -> Result<BigInt, PoolError> {
    // Weight-based formula: out = balance_out * (1 - (balance_in / (balance_in + amount_in))^(weight_in / weight_out))
}
```

**Key Difference**: Uniswap V4 uses **tick-based price discovery with sqrt_price_x96**, while Balancer V3 uses **weight-based pricing with invariant calculations**.

### **3. Pool State Representation**

#### **Uniswap V4:**
```rust
// Tick-based pool state
pub struct PoolState {
    pub slot0: Slot0,
    pub liquidity: u128,
    pub tick_bitmap: HashMap<i16, u256>,
    pub positions: HashMap<PositionKey, Position>,
}
```

#### **Balancer V3:**
```rust
// Weight-based pool state
pub struct BasePoolState {
    pub pool_address: String,
    pub pool_type: String,
    pub tokens: Vec<String>,
    pub balances_live_scaled_18: Vec<BigInt>,
    pub swap_fee: BigInt,
    pub total_supply: BigInt,
}

pub struct WeightedState {
    pub base: BasePoolState,
    pub weights: Vec<BigInt>,  // Normalized weights
}
```

**Key Difference**: Uniswap V4 tracks **tick ranges and positions**, while Balancer V3 tracks **token balances and weights**.

## **🔧 Mathematical Differences**

### **1. Swap Calculations**

#### **Uniswap V4:**
```rust
// Constant product formula with tick-based pricing
pub fn compute_amount_out(
    amount_in: u256,
    reserve_in: u256,
    reserve_out: u256,
    fee: u32,
) -> u256 {
    let amount_in_with_fee = amount_in * (10000 - fee);
    let numerator = amount_in_with_fee * reserve_out;
    let denominator = reserve_in * 10000 + amount_in_with_fee;
    numerator / denominator
}
```

#### **Balancer V3:**
```rust
// Weight-based formula
pub fn compute_out_given_exact_in(
    balance_in: &BigInt,
    weight_in: &BigInt,
    balance_out: &BigInt,
    weight_out: &BigInt,
    amount_in: &BigInt,
) -> Result<BigInt, PoolError> {
    let denominator = balance_in + amount_in;
    let base = div_up_fixed(balance_in, &denominator)?;
    let exponent = div_down_fixed(weight_in, weight_out)?;
    let power = pow_up_fixed(&base, &exponent)?;
    mul_down_fixed(balance_out, &complement_fixed(&power)?)
}
```

**Key Difference**: Uniswap V4 uses **constant product formula**, while Balancer V3 uses **weight-based power formula**.

### **2. Invariant Calculations**

#### **Uniswap V4:**
```rust
// Simple constant product invariant
pub fn compute_invariant(reserve0: u256, reserve1: u256) -> u256 {
    reserve0 * reserve1
}
```

#### **Balancer V3:**
```rust
// Weight-based invariant
pub fn compute_invariant_down(
    normalized_weights: &[BigInt],
    balances: &[BigInt],
) -> Result<BigInt, PoolError> {
    let mut invariant = WAD.clone();
    for i in 0..normalized_weights.len() {
        let pow_result = pow_down_fixed(&balances[i], &normalized_weights[i])?;
        invariant = mul_down_fixed(&invariant, &pow_result)?;
    }
    Ok(invariant)
}
```

**Key Difference**: Uniswap V4 uses **simple multiplication**, while Balancer V3 uses **weighted power function**.

## **📊 Fee Structure Differences**

### **1. Fee Collection**

#### **Uniswap V4:**
```rust
// Fee accumulation per tick range
pub struct FeeGrowth {
    pub fee_growth_global_0_x128: u256,
    pub fee_growth_global_1_x128: u256,
    pub fee_growth_outside_0_x128: u256,
    pub fee_growth_outside_1_x128: u256,
}
```

#### **Balancer V3:**
```rust
// Simple fee percentage
pub struct BasePoolState {
    pub swap_fee: BigInt,           // Swap fee (scaled 18)
    pub aggregate_swap_fee: BigInt, // Aggregate fee
}
```

**Key Difference**: Uniswap V4 tracks **fee growth per position**, while Balancer V3 uses **simple percentage fees**.

## **🎯 Abstraction Strategy**

### **1. Generic Pool State**

```rust
// Common pool state that works for both AMMs
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
    BalancerV3,
}
```

### **2. Generic Swap Parameters**

```rust
// Common swap parameters
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

### **3. Generic Swap Result**

```rust
// Common swap result
pub struct GenericSwapResult {
    pub amount_in: U256,
    pub amount_out: U256,
    pub fee: U256,
    pub new_pool_state: GenericPoolState,
}
```

### **4. AMM Provider Trait**

```rust
#[async_trait]
pub trait AMMProvider: Send + Sync + 'static {
    // Pool state management
    async fn get_pool_state(&self, pool_id: PoolId) -> Result<GenericPoolState, AMMError>;
    async fn get_pool_liquidity(&self, pool_id: PoolId) -> Result<U256, AMMError>;
    
    // Swap execution
    async fn execute_swap(&self, params: GenericSwapParams) -> Result<GenericSwapResult, AMMError>;
    async fn compute_swap_amounts(&self, params: GenericSwapParams) -> Result<(U256, U256), AMMError>;
    
    // Position management (AMM-specific)
    async fn get_position_liquidity(&self, pool_id: PoolId, position_key: PositionKey) -> Result<U256, AMMError>;
    async fn get_position_rewards(&self, pool_id: PoolId, position_key: PositionKey) -> Result<U256, AMMError>;
    
    // Asset management
    async fn take_assets(&self, assets: Vec<AssetRequest>) -> Result<(), AMMError>;
    async fn settle_assets(&self, assets: Vec<AssetSettlement>) -> Result<(), AMMError>;
}
```

## **🔄 Data Conversion Strategy**

### **1. Uniswap V4 to Generic**

```rust
impl UniswapV4Provider {
    fn convert_pool_state(&self, uni_state: UniswapPoolState) -> GenericPoolState {
        GenericPoolState {
            pool_id: uni_state.pool_id,
            pool_type: PoolType::UniswapV4,
            tokens: uni_state.tokens,
            balances: uni_state.balances,
            fee: uni_state.fee,
            total_supply: uni_state.total_supply,
            amm_specific_data: uni_state.to_bytes(), // Store tick data, positions, etc.
        }
    }
    
    fn convert_swap_params(&self, generic_params: GenericSwapParams) -> UniswapSwapParams {
        // Convert to Uniswap V4 specific format
        UniswapSwapParams {
            pool_id: generic_params.pool_id,
            zero_for_one: self.is_zero_for_one(generic_params.token_in, generic_params.token_out),
            amount_specified: generic_params.amount_in,
            sqrt_price_limit_x96: self.calculate_price_limit(generic_params),
            recipient: generic_params.recipient,
        }
    }
}
```

### **2. Balancer V3 to Generic**

```rust
impl BalancerV3Provider {
    fn convert_pool_state(&self, balancer_state: WeightedState) -> GenericPoolState {
        GenericPoolState {
            pool_id: balancer_state.base.pool_address.parse().unwrap(),
            pool_type: PoolType::BalancerV3,
            tokens: balancer_state.base.tokens.iter().map(|t| t.parse().unwrap()).collect(),
            balances: balancer_state.base.balances_live_scaled_18.iter().map(|b| U256::from_bigint(b)).collect(),
            fee: U256::from_bigint(&balancer_state.base.swap_fee),
            total_supply: U256::from_bigint(&balancer_state.base.total_supply),
            amm_specific_data: balancer_state.to_bytes(), // Store weights, etc.
        }
    }
    
    fn convert_swap_params(&self, generic_params: GenericSwapParams) -> BalancerSwapParams {
        // Convert to Balancer V3 specific format
        BalancerSwapParams {
            swap_kind: match generic_params.swap_kind {
                SwapKind::GivenIn => BalancerSwapKind::GivenIn,
                SwapKind::GivenOut => BalancerSwapKind::GivenOut,
            },
            token_in_index: self.get_token_index(generic_params.token_in),
            token_out_index: self.get_token_index(generic_params.token_out),
            amount_scaled_18: BigInt::from(generic_params.amount_in),
            balances_live_scaled_18: self.get_balances_scaled_18(),
        }
    }
}
```

## **📋 Implementation Priorities**

### **1. High Priority (Core Functionality)**
- ✅ **Pool State Conversion**: Convert between AMM-specific and generic pool states
- ✅ **Swap Parameter Conversion**: Convert swap parameters for each AMM
- ✅ **Basic Swap Execution**: Execute swaps through AMM providers

### **2. Medium Priority (Advanced Features)**
- 🔄 **Position Management**: Handle different liquidity models
- 🔄 **Fee Calculation**: Abstract fee collection mechanisms
- 🔄 **Price Discovery**: Abstract different pricing models

### **3. Low Priority (Optimization)**
- 📈 **Performance Optimization**: Optimize data conversion overhead
- 📈 **Caching**: Cache converted pool states
- 📈 **Batch Operations**: Support batch operations across AMMs

## **🎯 Key Insights**

### **1. Fundamental Differences**
- **Uniswap V4**: Tick-based, non-fungible positions, constant product
- **Balancer V3**: Weight-based, fungible tokens, power function

### **2. Common Patterns**
- Both have **pool state**, **swap operations**, **fee collection**
- Both use **BigInt** for precise calculations
- Both support **multiple token pairs**

### **3. Abstraction Strategy**
- **Generic data structures** for common concepts
- **AMM-specific data storage** for unique features
- **Conversion functions** for data transformation
- **Trait-based interface** for AMM operations

This analysis provides the foundation for designing a robust multi-AMM architecture that can handle both Uniswap V4 and Balancer V3 while maintaining clean separation of concerns.

# ToB Bid Parameter - ETH Value for LPs

## **🎯 The ToB Bid Parameter**

The **`tob_reward`** parameter is the key field that represents the **ETH bid amount** that will be returned to LPs (Liquidity Providers) in the ToB auction mechanism.

## **📋 Parameter Details**

### **Parameter Name:** `tob_reward`
- **Type:** `U256` (256-bit unsigned integer)
- **Purpose:** Represents the ETH bid amount for the ToB auction
- **Destination:** Distributed to LPs via the reward system

### **Location in Code:**

#### **1. Order Builder (Rust)**
**File:** `testing-tools/src/type_generator/orders/mod.rs`
```rust
// Lines 50, 83-84
pub struct OrderWithStorageDataBuilder<O> {
    // ... other fields
    tob_reward: Option<U256>
}

impl<O> OrderWithStorageDataBuilder<O> {
    pub fn tob_reward(self, tob_reward: U256) -> Self {
        Self { tob_reward: Some(tob_reward), ..self }
    }
}
```

#### **2. Order Structure (Rust)**
**File:** `crates/types/src/sol_bindings/ext/grouped_orders.rs`
```rust
// Lines 79
pub struct OrderWithStorageData<O> {
    // ... other fields
    pub tob_reward: U256,  // This is the ToB bid amount
}
```

#### **3. Priority Validation (Rust)**
**File:** `crates/types/src/sol_bindings/ext/mod.rs`
```rust
// Lines 144, 148-149
pub struct OrderValidationPriority {
    // ... other fields
    pub tob_bid_amount: u128  // Used for priority comparison
}

impl OrderValidationPriority {
    pub fn set_tob_bid(&mut self, bid: u128) {
        self.tob_bid_amount = bid;
    }
}
```

## **🔧 How to Set the ToB Bid**

### **Method 1: Using the Order Builder**
```rust
use testing_tools::type_generator::orders::{ToBOrderBuilder, OrderWithStorageDataBuilder};

// Create a ToB order with a bid
let tob_order = ToBOrderBuilder::new()
    .quantity_in(1000000)
    .quantity_out(950000)
    .build();

// Wrap it with the bid amount
let order_with_bid = OrderWithStorageDataBuilder::new(tob_order)
    .tob_reward(U256::from(1000000000000000000u128))  // 1 ETH bid
    .build();
```

### **Method 2: Direct Assignment**
```rust
let mut order = OrderWithStorageData::with_default(tob_order);
order.tob_reward = U256::from(500000000000000000u128);  // 0.5 ETH bid
```

### **Method 3: Priority Setting**
```rust
let mut priority = OrderValidationPriority::default();
priority.set_tob_bid(1000000000000000000u128);  // 1 ETH bid
```

## **💰 Bid Amount Guidelines**

### **Typical Bid Ranges:**
- **Small MEV opportunities:** 0.01 - 0.1 ETH
- **Medium MEV opportunities:** 0.1 - 1.0 ETH  
- **Large MEV opportunities:** 1.0 - 10.0 ETH
- **Very large opportunities:** 10.0+ ETH

### **Bid Calculation Factors:**
1. **Expected MEV:** The potential profit from the arbitrage
2. **Gas costs:** Transaction execution costs
3. **Competition:** Other searchers' expected bids
4. **Risk tolerance:** How much profit to share with LPs

## **🎯 How the Bid Works in the Auction**

### **1. Bid Submission**
```rust
// Searcher submits ToB order with bid
let tob_order = OrderWithStorageDataBuilder::new(tob_order)
    .tob_reward(U256::from(bid_amount_wei))  // Set your bid here
    .build();
```

### **2. Priority Comparison**
```rust
// Higher bids get higher priority
if searcher.tob_reward > current.tob_reward {
    // This searcher wins the auction
}
```

### **3. Value Distribution**
- **Searcher receives:** MEV - bid_amount
- **LPs receive:** bid_amount (distributed via reward system)
- **Protocol receives:** Trading fees

## **📊 Example Usage**

### **Complete ToB Order with Bid**
```rust
use testing_tools::type_generator::orders::{ToBOrderBuilder, OrderWithStorageDataBuilder};
use alloy::primitives::U256;

// Create the base ToB order
let tob_order = ToBOrderBuilder::new()
    .asset_in(token_a_address)
    .asset_out(token_b_address)
    .quantity_in(1000000000000000000000u128)  // 1000 tokens
    .quantity_out(950000000000000000000u128)  // 950 tokens
    .max_gas(500000u128)
    .valid_block(block_number)
    .signing_key(Some(signer))
    .build();

// Add the bid amount (0.5 ETH)
let order_with_bid = OrderWithStorageDataBuilder::new(tob_order)
    .tob_reward(U256::from(500000000000000000u128))  // 0.5 ETH in wei
    .pool_id(pool_id)
    .is_bid(true)
    .build();
```

## **🔍 Key Points**

1. **`tob_reward` is the bid amount** that goes to LPs
2. **Higher bids = higher priority** in the auction
3. **Bid is in wei** (1 ETH = 1,000,000,000,000,000,000 wei)
4. **Bid should be less than expected MEV** to ensure profit
5. **Bid is distributed to LPs** via the reward system

## **⚠️ Important Considerations**

- **Bid too low:** Order might not win the auction
- **Bid too high:** Might reduce your profit margin
- **Gas costs:** Factor in execution costs when setting bid
- **Competition:** Consider what other searchers might bid
- **MEV estimation:** Accurately estimate the arbitrage opportunity

The `tob_reward` parameter is essentially your "auction bid" - the amount of ETH you're willing to pay to LPs in exchange for executing your ToB order and capturing the MEV opportunity.

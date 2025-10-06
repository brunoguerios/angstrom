# Angstrom Protocol Overview

## **Angstrom: A Hybrid AMM/Orderbook Exchange**

Angstrom is a **trustless, hybrid AMM/Orderbook exchange** that settles on Ethereum and aims to mitigate MEV (Maximal Extractable Value) for both users and liquidity providers. Here's how it all works:

### **🏗️ Architecture Overview**

**Two-Layer System:**
- **On-Chain**: Smart contracts for order settlement and AMM integration
- **Off-Chain**: Rust-based network of staked nodes for order matching and bundle building

### **🎯 Key Entry Points & User Flows**

#### **1. User Order Submission Flow**
```
User → RPC API → Order Pool → Validation → Matching Engine → Bundle Building → On-Chain Execution
```

**Entry Points:**
- **RPC API** (`crates/rpc/src/api/orders.rs`): Users submit orders via JSON-RPC
  - `sendOrder()` - Submit individual orders
  - `sendOrders()` - Submit multiple orders
  - `cancelOrder()` - Cancel existing orders
  - `orderStatus()` - Check order status

**Order Types:**
- **Limit Orders**: Traditional orderbook-style orders
- **Flash Orders**: Immediate execution orders
- **Standing Orders**: Orders that persist across blocks
- **ToB (Top-of-Block) Orders**: Searcher orders for MEV extraction

#### **2. Node Network Flow**
```
Angstrom Node → P2P Network → Order Propagation → Consensus → Bundle Building → Ethereum
```

**Node Components:**
- **Order Pool** (`crates/order-pool/`): Manages all order types
- **Matching Engine** (`crates/matching-engine/`): Matches orders against each other and AMM
- **Validation** (`crates/validation/`): Validates orders and user balances
- **Network** (`crates/angstrom-net/`): P2P communication between nodes

### **🔧 Core Components**

#### **Smart Contracts (`contracts/`)**
- **`Angstrom.sol`**: Main contract with `execute()` entry point
- **Uniswap V4 Integration**: Uses Uniswap V4 as the underlying AMM
- **Pool Config Store**: Manages enabled trading pairs
- **Reward System**: Tracks and distributes LP rewards

#### **Rust Infrastructure (`crates/`)**
- **`angstrom-net/`**: P2P network for order propagation
- **`order-pool/`**: Order management and storage
- **`matching-engine/`**: Order matching algorithms
- **`validation/`**: Order and state validation
- **`consensus/`**: Network consensus mechanisms
- **`rpc/`**: JSON-RPC API for user interactions

### **💰 Economic Model**

#### **MEV Mitigation**
- **For Users**: Uniform clearing prices prevent sandwich attacks
- **For LPs**: ToB auctions internalize MEV extraction, redistributing value to LPs

#### **Fee Structure**
- **Exchange Fees**: Set per pair (`fee_in_e6`)
- **Gas Fees**: Charged in trading assets (not ETH)
- **Referral Fees**: Optional fees for referrers
- **LP Rewards**: Distributed based on tick ranges

### **🔄 Key User Flows**

#### **1. Trading Flow**
1. **User submits order** via RPC API
2. **Order validated** for signature, balance, nonce
3. **Order propagated** across P2P network
4. **Order matched** against other orders and AMM
5. **Bundle built** containing matched orders
6. **Bundle executed** on-chain via `execute()` function
7. **User receives** tokens and pays fees

#### **2. Liquidity Provision Flow**
1. **LP adds liquidity** to Uniswap V4 pools
2. **Angstrom integrates** with these pools
3. **LP earns rewards** from ToB auctions and trading fees
4. **Rewards distributed** based on tick range participation

#### **3. Node Operation Flow**
1. **Node stakes** tokens to participate
2. **Node receives** orders via P2P network
3. **Node validates** and matches orders
4. **Node builds** execution bundles
5. **Node submits** bundles to Ethereum
6. **Node earns** fees for successful execution

### **📊 Outputs & Results**

#### **For Users:**
- **Fair execution**: No MEV extraction via uniform clearing
- **Gas efficiency**: Fees paid in trading assets
- **Immediate settlement**: Trades settle on Ethereum L1
- **Order types**: Multiple order types for different strategies

#### **For LPs:**
- **MEV protection**: Value extraction internalized
- **Reward distribution**: Based on tick range participation
- **Fee sharing**: From both trading and ToB auctions

#### **For Nodes:**
- **Fee revenue**: From successful bundle execution
- **Staking rewards**: For maintaining network security
- **Slashing risk**: For malicious behavior

### **🔗 Integration Points**

#### **External Dependencies:**
- **Uniswap V4**: Underlying AMM infrastructure
- **Ethereum L1**: Settlement layer
- **P2P Network**: Order propagation
- **MEV-Boost**: Bundle submission

#### **User Interfaces:**
- **RPC API**: Programmatic access
- **WebSocket Subscriptions**: Real-time order updates
- **CLI Tools**: Node operation and testing

### **🏛️ Trust Model & Assumptions**

#### **Economic Security Assumptions:**
- **Staked Nodes**: All nodes must stake tokens to participate
- **Slashing Mechanism**: Malicious behavior results in stake loss
- **Sufficient Stake**: Total stake must cover potential user losses
- **Rational Actors**: Nodes act in their economic self-interest

#### **Technical Assumptions:**
- **Canonical Ethereum**: Only deployed on Ethereum L1/mainnet
- **Sound Controllers**: Controller maintains approved node set
- **No Fee-on-Transfer**: Only whitelisted tokens without rebasing
- **Well-Behaving Routers**: Users use sound liquidity routers

### **⚡ Bundle Execution Process**

The core execution flow in Angstrom works as follows:

1. **Load & Validate Assets**: Verify all assets referenced in bundle
2. **Load & Validate Pairs**: Check trading pairs and configurations
3. **Take Assets from Uniswap**: Flash loan or settlement amounts
4. **Update Pools**: Execute swaps and update reward accumulators
5. **Execute ToB Orders**: Process top-of-block orders
6. **Execute User Orders**: Process regular user orders
7. **Settle Deltas**: Repay Uniswap and finalize balances
8. **Emit Events**: Record fee summaries and outcomes

### **🎯 Key Innovations**

#### **Hybrid Architecture:**
- Combines orderbook efficiency with AMM liquidity
- Prevents MEV extraction through uniform clearing
- Internalizes arbitrage value for LP benefit

#### **Gas Optimization:**
- PADE encoding for efficient bundle transmission
- SSTORE2 pattern for pool configuration storage
- Batch processing of multiple orders per bundle

#### **Economic Security:**
- Staked node network with slashing mechanisms
- Economic incentives aligned with protocol health
- Fair fee distribution and reward mechanisms

---

This system creates a sophisticated DeFi infrastructure that combines the efficiency of orderbooks with the liquidity of AMMs while protecting participants from MEV extraction. The hybrid approach allows for better price discovery while maintaining the benefits of automated market making.

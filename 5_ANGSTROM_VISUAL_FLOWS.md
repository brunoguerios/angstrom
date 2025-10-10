# Angstrom Visual Flows & ToB Auction Mechanism

## **🏗️ High-Level System Architecture**

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              ANSTROM NETWORK                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐  │
│  │   User 1    │    │   User 2    │    │   User N    │    │   Searcher  │  │
│  │ (Trader)    │    │ (Trader)    │    │ (Trader)    │    │ (ToB Order) │  │
│  └─────┬───────┘    └─────┬───────┘    └─────┬───────┘    └─────┬───────┘  │
│        │                  │                  │                  │          │
│        └──────────────────┼──────────────────┼──────────────────┘          │
│                           │                  │                             │
│                    ┌──────▼──────┐    ┌──────▼──────┐                      │
│                    │   RPC API   │    │   RPC API   │                      │
│                    │  (Orders)   │    │  (Orders)   │                      │
│                    └──────┬──────┘    └──────┬──────┘                      │
│                           │                  │                             │
│                    ┌──────▼──────────────────▼──────┐                      │
│                    │        ORDER POOL              │                      │
│                    │  ┌─────────┐  ┌─────────┐     │                      │
│                    │  │ Limit   │  │ Flash   │     │                      │
│                    │  │ Orders  │  │ Orders  │     │                      │
│                    │  └─────────┘  └─────────┘     │                      │
│                    │  ┌─────────┐  ┌─────────┐     │                      │
│                    │  │Standing │  │ ToB     │     │                      │
│                    │  │Orders   │  │Orders   │     │                      │
│                    │  └─────────┘  └─────────┘     │                      │
│                    └─────────────┬─────────────────┘                      │
│                                  │                                        │
│                    ┌─────────────▼─────────────────┐                      │
│                    │      MATCHING ENGINE          │                      │
│                    │  ┌─────────────────────────┐  │                      │
│                    │  │   Order Matching        │  │                      │
│                    │  │   + AMM Integration     │  │                      │
│                    │  └─────────────────────────┘  │                      │
│                    └─────────────┬─────────────────┘                      │
│                                  │                                        │
│  ┌─────────────┐    ┌─────────────▼─────────────────┐    ┌─────────────┐  │
│  │   Node 1    │    │        BUNDLE BUILDER         │    │   Node N    │  │
│  │ (Staked)    │◄───┤  ┌─────────────────────────┐  ├───►│ (Staked)    │  │
│  └─────────────┘    │  │   • Order Selection     │  │    └─────────────┘  │
│                     │  │   • ToB Auction         │  │                     │
│  ┌─────────────┐    │  │   • Gas Optimization    │  │    ┌─────────────┐  │
│  │   Node 2    │    │  │   • Bundle Assembly     │  │    │  Ethereum    │  │
│  │ (Staked)    │◄───┤  └─────────────────────────┘  ├───►│     L1       │  │
│  └─────────────┘    └─────────────┬─────────────────┘    └─────────────┘  │
│                                  │                                        │
│                    ┌─────────────▼─────────────────┐                      │
│                    │      CONSENSUS LAYER          │                      │
│                    │  ┌─────────────────────────┐  │                      │
│                    │  │   • Bundle Validation   │  │                      │
│                    │  │   • Winner Selection    │  │                      │
│                    │  │   • Slashing Logic      │  │                      │
│                    │  └─────────────────────────┘  │                      │
│                    └─────────────────────────────────┘                      │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## **🎯 Top of Block (ToB) Auction Flow**

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           TOP OF BLOCK AUCTION                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐  │
│  │ Searcher 1  │    │ Searcher 2  │    │ Searcher 3  │    │ Searcher N  │  │
│  │ (MEV Bot)   │    │ (MEV Bot)   │    │ (MEV Bot)   │    │ (MEV Bot)   │  │
│  └─────┬───────┘    └─────┬───────┘    └─────┬───────┘    └─────┬───────┘  │
│        │                  │                  │                  │          │
│        │                  │                  │                  │          │
│        └──────────────────┼──────────────────┼──────────────────┘          │
│                           │                  │                             │
│                    ┌──────▼──────────────────▼──────────────────▼──────┐    │
│                    │              TOB ORDER SUBMISSION                │    │
│                    │  ┌─────────────────────────────────────────────┐  │    │
│                    │  │ • Bid Amount (in ETH)                      │  │    │
│                    │  │ • Gas Price                                │  │    │
│                    │  │ • Bundle Contents                          │  │    │
│                    │  │ • MEV Extraction Strategy                  │  │    │
│                    │  └─────────────────────────────────────────────┘  │    │
│                    └─────────────────────────────────────────────────────┘    │
│                                                                             │
│                    ┌─────────────────────────────────────────────────────┐    │
│                    │              AUCTION PROCESS                       │    │
│                    │  ┌─────────────────────────────────────────────┐  │    │
│                    │  │ 1. Collect all ToB orders for the block     │  │    │
│                    │  │ 2. Calculate potential MEV for each bundle  │  │    │
│                    │  │ 3. Rank bundles by profit potential         │  │    │
│                    │  │ 4. Select winning bundle                    │  │    │
│                    │  └─────────────────────────────────────────────┘  │    │
│                    └─────────────────────────────────────────────────────┘    │
│                                                                             │
│                    ┌─────────────────────────────────────────────────────┐    │
│                    │              WINNER SELECTION                      │    │
│                    │  ┌─────────────────────────────────────────────┐  │    │
│                    │  │ • Highest MEV potential                     │  │    │
│                    │  │ • Valid bundle structure                    │  │    │
│                    │  │ • Sufficient gas for execution              │  │    │
│                    │  │ • Passes all validations                    │  │    │
│                    │  └─────────────────────────────────────────────┘  │    │
│                    └─────────────────────────────────────────────────────┘    │
│                                                                             │
│                    ┌─────────────────────────────────────────────────────┐    │
│                    │              VALUE DISTRIBUTION                     │    │
│                    │  ┌─────────────────────────────────────────────┐  │    │
│                    │  │ • Winning Searcher: MEV - Auction Fee      │  │    │
│                    │  │ • Angstrom Protocol: Auction Fee           │  │    │
│                    │  │ • LPs: Auction Fee (via reward system)     │  │    │
│                    │  │ • Nodes: Execution fees                    │  │    │
│                    │  └─────────────────────────────────────────────┘  │    │
│                    └─────────────────────────────────────────────────────┘    │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## **🔄 Node Competition & Bundle Building Flow**

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        NODE COMPETITION & BUNDLE BUILDING                   │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐  │
│  │   Node 1    │    │   Node 2    │    │   Node 3    │    │   Node N    │  │
│  │ (Staked)    │    │ (Staked)    │    │ (Staked)    │    │ (Staked)    │  │
│  └─────┬───────┘    └─────┬───────┘    └─────┬───────┘    └─────┬───────┘  │
│        │                  │                  │                  │          │
│        │                  │                  │                  │          │
│        └──────────────────┼──────────────────┼──────────────────┘          │
│                           │                  │                             │
│                    ┌──────▼──────────────────▼──────────────────▼──────┐    │
│                    │              ORDER POOL (Shared)                 │    │
│                    │  ┌─────────────────────────────────────────────┐  │    │
│                    │  │ • All nodes see the same orders             │  │    │
│                    │  │ • Real-time order updates                   │  │    │
│                    │  │ • Order validation status                   │  │    │
│                    │  │ • AMM state information                     │  │    │
│                    │  └─────────────────────────────────────────────┘  │    │
│                    └─────────────────────────────────────────────────────┘    │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │                    BUNDLE BUILDING COMPETITION                     │    │
│  │                                                                     │    │
│  │  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐            │    │
│  │  │   Node 1    │    │   Node 2    │    │   Node 3    │            │    │
│  │  │ Bundle A    │    │ Bundle B    │    │ Bundle C    │            │    │
│  │  │ • Orders    │    │ • Orders    │    │ • Orders    │            │    │
│  │  │ • ToB Bid   │    │ • ToB Bid   │    │ • ToB Bid   │            │    │
│  │  │ • Gas Opt   │    │ • Gas Opt   │    │ • Gas Opt   │            │    │
│  │  │ • Profit    │    │ • Profit    │    │ • Profit    │            │    │
│  │  └─────────────┘    └─────────────┘    └─────────────┘            │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
│                                                                             │
│                    ┌─────────────────────────────────────────────────────┐    │
│                    │              BUNDLE EVALUATION                     │    │
│                    │  ┌─────────────────────────────────────────────┐  │    │
│                    │  │ • Calculate total MEV potential             │  │    │
│                    │  │ • Estimate gas costs                        │  │    │
│                    │  │ • Validate bundle structure                 │  │    │
│                    │  │ • Check order validity                      │  │    │
│                    │  │ • Rank by net profit                        │  │    │
│                    │  └─────────────────────────────────────────────┘  │    │
│                    └─────────────────────────────────────────────────────┘    │
│                                                                             │
│                    ┌─────────────────────────────────────────────────────┐    │
│                    │              WINNER SELECTION                      │    │
│                    │  ┌─────────────────────────────────────────────┐  │    │
│                    │  │ • Highest net profit bundle wins            │  │    │
│                    │  │ • Winning node gets execution rights        │  │    │
│                    │  │ • Other nodes can challenge if needed       │  │    │
│                    │  │ • Slashing occurs for invalid bundles       │  │    │
│                    │  └─────────────────────────────────────────────┘  │    │
│                    └─────────────────────────────────────────────────────┘    │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## **⚡ Bundle Execution Flow**

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                            BUNDLE EXECUTION FLOW                            │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────┐                                                           │
│  │   Winning   │                                                           │
│  │    Node     │                                                           │
│  └─────┬───────┘                                                           │
│        │                                                                   │
│        │  ┌─────────────────────────────────────────────────────────────┐  │
│        │  │              BUNDLE SUBMISSION                              │  │
│        │  │  ┌─────────────────────────────────────────────────────┐  │  │
│        │  │  │ • Submit to MEV-Boost                               │  │  │
│        │  │  │ • Include ToB order                                 │  │  │
│        │  │  │ • Specify gas price                                 │  │  │
│        │  │  │ • Set execution deadline                            │  │  │
│        │  │  └─────────────────────────────────────────────────────┘  │  │
│        │  └─────────────────────────────────────────────────────────────┘  │
│        │                                                                   │
│        │  ┌─────────────────────────────────────────────────────────────┐  │
│        │  │              BLOCK PRODUCER                                │  │
│        │  │  ┌─────────────────────────────────────────────────────┐  │  │
│        │  │  │ • Receives bundle from MEV-Boost                    │  │  │
│        │  │  │ • Validates bundle structure                         │  │  │
│        │  │  │ • Checks gas requirements                            │  │  │
│        │  │  │ • Includes in block                                 │  │  │
│        │  │  └─────────────────────────────────────────────────────┘  │  │
│        │  └─────────────────────────────────────────────────────────────┘  │
│        │                                                                   │
│        │  ┌─────────────────────────────────────────────────────────────┐  │
│        │  │              ANSTROM CONTRACT                              │  │
│        │  │  ┌─────────────────────────────────────────────────────┐  │  │
│        │  │  │ 1. Load & Validate Assets                           │  │  │
│        │  │  │ 2. Load & Validate Pairs                            │  │  │
│        │  │  │ 3. Take Assets from Uniswap                         │  │  │
│        │  │  │ 4. Update Pools (swaps + rewards)                   │  │  │
│        │  │  │ 5. Execute ToB Orders                               │  │  │
│        │  │  │ 6. Execute User Orders                              │  │  │
│        │  │  │ 7. Settle Deltas                                    │  │  │
│        │  │  │ 8. Emit Events                                      │  │  │
│        │  │  └─────────────────────────────────────────────────────┘  │  │
│        │  └─────────────────────────────────────────────────────────────┘  │
│        │                                                                   │
│        │  ┌─────────────────────────────────────────────────────────────┐  │
│        │  │              VALUE DISTRIBUTION                             │  │
│        │  │  ┌─────────────────────────────────────────────────────┐  │  │
│        │  │  │ • Users: Receive tokens, pay fees                   │  │  │
│        │  │  │ • LPs: Receive rewards from swaps                   │  │  │
│        │  │  │ • Searchers: Receive MEV - auction fee              │  │  │
│        │  │  │ • Nodes: Receive execution fees                     │  │  │
│        │  │  │ • Protocol: Receive auction fees                    │  │  │
│        │  │  └─────────────────────────────────────────────────────┘  │  │
│        │  └─────────────────────────────────────────────────────────────┘  │
│        │                                                                   │
│        └───────────────────────────────────────────────────────────────────┘
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## **💰 Economic Flow & Value Distribution**

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        ECONOMIC FLOW & VALUE DISTRIBUTION                   │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐  │
│  │   Users     │    │ Searchers   │    │     LPs     │    │   Nodes     │  │
│  │ (Traders)   │    │ (MEV Bots)  │    │(Liquidity)  │    │(Staked)     │  │
│  └─────┬───────┘    └─────┬───────┘    └─────┬───────┘    └─────┬───────┘  │
│        │                  │                  │                  │          │
│        │                  │                  │                  │          │
│        └──────────────────┼──────────────────┼──────────────────┘          │
│                           │                  │                             │
│                    ┌──────▼──────────────────▼──────────────────▼──────┐    │
│                    │              VALUE POOL                           │    │
│                    │  ┌─────────────────────────────────────────────┐  │    │
│                    │  │ • Trading Fees                              │  │    │
│                    │  │ • MEV Extraction                            │  │    │
│                    │  │ • ToB Auction Fees                          │  │    │
│                    │  │ • Gas Fees                                  │  │    │
│                    │  └─────────────────────────────────────────────┘  │    │
│                    └─────────────────────────────────────────────────────┘    │
│                                                                             │
│                    ┌─────────────────────────────────────────────────────┐    │
│                    │              FEE DISTRIBUTION                       │    │
│                    │  ┌─────────────────────────────────────────────┐  │    │
│                    │  │ • Trading Fees → LPs (70%) + Protocol (30%) │  │    │
│                    │  │ • ToB Auction Fees → LPs                    │  │    │
│                    │  │ • MEV → Searchers (minus auction fee)       │  │    │
│                    │  │ • Gas Fees → Nodes                          │  │    │
│                    │  │ • Protocol Fees → Treasury                  │  │    │
│                    │  └─────────────────────────────────────────────┘  │    │
│                    └─────────────────────────────────────────────────────┘    │
│                                                                             │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐  │
│  │   Users     │    │ Searchers   │    │     LPs     │    │   Nodes     │  │
│  │ • Pay fees  │    │ • Get MEV   │    │ • Get fees  │    │ • Get fees  │  │
│  │ • Get tokens│    │ • Pay auction│   │ • Get rewards│   │ • Stake req │  │
│  │ • Fair exec │    │   fees      │    │ • MEV prot  │    │ • Slash risk│  │
│  └─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## **🔄 Node Interaction & Consensus Flow**

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        NODE INTERACTION & CONSENSUS                         │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐  │
│  │   Node 1    │    │   Node 2    │    │   Node 3    │    │   Node N    │  │
│  │ (Staked)    │    │ (Staked)    │    │ (Staked)    │    │ (Staked)    │  │
│  └─────┬───────┘    └─────┬───────┘    └─────┬───────┘    └─────┬───────┘  │
│        │                  │                  │                  │          │
│        │                  │                  │                  │          │
│        └──────────────────┼──────────────────┼──────────────────┘          │
│                           │                  │                             │
│                    ┌──────▼──────────────────▼──────────────────▼──────┐    │
│                    │              P2P NETWORK                          │    │
│                    │  ┌─────────────────────────────────────────────┐  │    │
│                    │  │ • Order propagation                         │  │    │
│                    │  │ • Bundle sharing                            │  │    │
│                    │  │ • State synchronization                     │  │    │
│                    │  │ • Consensus messages                        │  │    │
│                    │  └─────────────────────────────────────────────┘  │    │
│                    └─────────────────────────────────────────────────────┘    │
│                                                                             │
│                    ┌─────────────────────────────────────────────────────┐    │
│                    │              CONSENSUS PROCESS                      │    │
│                    │  ┌─────────────────────────────────────────────┐  │    │
│                    │  │ 1. Bundle submission deadline               │  │    │
│                    │  │ 2. Bundle validation by all nodes           │  │    │
│                    │  │ 3. Winner selection based on profit         │  │    │
│                    │  │ 4. Challenge period for invalid bundles     │  │    │
│                    │  │ 5. Slashing for malicious behavior          │  │    │
│                    │  └─────────────────────────────────────────────┘  │    │
│                    └─────────────────────────────────────────────────────┘    │
│                                                                             │
│                    ┌─────────────────────────────────────────────────────┐    │
│                    │              SLASHING MECHANISM                    │    │
│                    │  ┌─────────────────────────────────────────────┐  │    │
│                    │  │ • Invalid bundle submission                 │  │    │
│                    │  │ • Censorship of valid orders                │  │    │
│                    │  │ • Double-spending attempts                  │  │    │
│                    │  │ • Stake confiscation                        │  │    │
│                    │  │ • Node removal from network                 │  │    │
│                    │  └─────────────────────────────────────────────┘  │    │
│                    └─────────────────────────────────────────────────────┘    │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## **🎯 Key Insights from Visual Flows**

### **1. ToB Auction Mechanism:**
- **Competitive bidding** among searchers for MEV extraction rights
- **Value redistribution** to LPs instead of external MEV extractors
- **Fair auction process** with transparent winner selection
- **Economic incentives** aligned across all participants

### **2. Node Competition:**
- **All nodes see the same orders** from the shared order pool
- **Bundle building competition** based on profit optimization
- **Winner selection** based on net profit (MEV - costs)
- **Staking requirements** ensure honest behavior

### **3. Value Distribution:**
- **Users**: Fair execution, gas efficiency, MEV protection
- **LPs**: Fee sharing, MEV protection, reward distribution
- **Searchers**: MEV extraction minus auction fees
- **Nodes**: Execution fees, staking rewards, slashing risk
- **Protocol**: Treasury fees for sustainability

### **4. Economic Security:**
- **Staked nodes** with slashing mechanisms
- **Transparent auction** process
- **Fair value distribution** to all participants
- **MEV internalization** benefits LPs instead of external extractors

This visual representation shows how Angstrom creates a **fair, efficient, and secure** trading environment where all participants benefit from the system's design.

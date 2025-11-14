# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Angstrom is a trustless, hybrid AMM/Orderbook exchange that settles on Ethereum L1. It's designed to mitigate MEV for both users and liquidity providers by using a network of staked nodes for order matching and bundle creation.

## Key Commands

### Rust Development (via justfile)

**Recommended**: Use `just` commands when available for consistency and convenience.

```bash
# Quick Commands (preferred)
just                  # Show available commands
just ci               # Run full CI suite (format check, clippy, all tests)
just check            # Check format, clippy, and run unit tests
just fix              # Auto-fix formatting and clippy issues
just test             # Run unit tests
just test-integration # Run integration tests
just build            # Build release version
just clean            # Clean build artifacts

# Detailed just commands
just check-format     # Check code formatting
just fix-format       # Auto-fix formatting issues
just check-clippy     # Run clippy linter
just fix-clippy       # Auto-fix clippy issues
```

### Direct Cargo Commands (when needed)

Use these when the justfile doesn't provide the specific functionality:

```bash
# Building
cargo build --workspace                    # Build all workspace members
cargo build --workspace --all-features     # Build with all features
cargo build --profile maxperf              # Build with maximum performance

# Testing with specific options
cargo test --workspace -- --nocapture      # Run tests with output

# Running binaries
cargo run --bin angstrom                   # Run main node
cargo run --bin testnet                    # Run testnet
cargo run --bin counter-matcher            # Run order matcher
```

### Smart Contract Development

```bash
cd contracts

# Setup Python environment (required for FFI tests)
uv venv .venv
source .venv/bin/activate
uv pip install -r requirements.txt

# Building & Testing
forge build                                # Build contracts
forge test --ffi                           # Run tests (FFI required)
forge test -vvv --ffi                      # Run tests with verbosity
forge test --match-test <name> --ffi       # Run specific test

# Formatting
forge fmt                                  # Format Solidity code
forge fmt --check                          # Check formatting
```

## Architecture Overview

### Smart Contracts (`/contracts`)
- **Core Contract**: Handles order validation, settlement, and AMM reward management
- **Periphery Contracts**: Access control and fee distribution
- Uses Uniswap V4 and Balancer V3 as underlying AMMs
- PADE encoding for efficient data packing
- "Code as storage" (SSTORE2) pattern for gas optimization

### Node Implementation (`/crates`)
- **angstrom-net**: Custom P2P network layer for order propagation
- **consensus**: Leader selection and bundle finalization
- **order-pool**: Manages limit orders, searcher orders, and finalization pool
- **matching-engine**: Implements uniform clearing for order matching
- **validation**: Validates orders and bundles
- **eth**: Ethereum integration layer using Reth
- **rpc**: JSON-RPC and gRPC interfaces
- **uniswap-v4**: Uniswap V4 pool state management and swap calculations
- **balancer-v3**: Balancer V3 pool state management and swap calculations

### Key Design Decisions
1. **No Events**: Contracts avoid events to save gas
2. **Explicit Imports**: No auto-exports in contracts
3. **Custom Auth**: Uses controller logic instead of standard Ownable
4. **Economic Security**: Relies on staked nodes for censorship resistance
5. **Uniform Clearing**: Batches limit orders at common prices to prevent MEV

## Testing Strategy

- Unit tests in each crate/contract
- Integration tests for cross-module functionality
- Invariant tests for critical contract properties
- FFI tests using Python scripts for complex scenarios
- Testnet binary for end-to-end testing

## Important Notes

1. **Dependencies**:
   - Requires Rust 1.88.0+ (edition 2024)
   - Requires Foundry for contracts
   - Requires Python 3.12 for FFI tests
   - Requires nightly Rust for formatting
   - Requires `just` command runner for simplified development workflow

2. **Assumptions**:
   - Only standard ERC20 tokens (no fee-on-transfer)
   - Deployment only on Ethereum L1 mainnet or canonical testnets
   - Nodes must provide adequate stake for slashing

3. **Performance Profiles**:
   - `dev`: Development with some optimizations
   - `release`: Standard release build
   - `maxperf`: Maximum performance with aggressive optimizations

4. **Known Limitations**:
   - Limited encoding capabilities (intentional for gas)
   - Single Uniswap AMM pool configuration at a time
   - No cross-pair price consistency guarantees

## Development Roadmap

### Balancer V3 Integration (Steps 9-15)

The Balancer V3 integration has been broken down into focused, sequential steps:

- **Step 9** (✅ COMPLETE): Balancer RPC Query System & Pool State Management
  - RPC infrastructure for querying Balancer V3 pools
  - ReClammPoolState and BalancerPoolManager
  - Integration tests with live Base mainnet data
  - Plan: `.cursor/plans/plan-step-9-balancer-rpc-and-state.plan.md`

- **Step 10** (Pending): Balancer Swap Calculation Integration
  - Adapter for balancer-maths-rust
  - Swap calculations (GivenIn/GivenOut)
  - Plan: `.cursor/plans/plan-step-10-balancer-swap-calculations.plan.md`

- **Step 11** (Pending): Balancer Pool Update Logic & Mocking
  - Pool state update methods
  - Mock DataLoader and builders for testing
  - Plan: `.cursor/plans/plan-step-11-balancer-pool-updates.plan.md`

- **Step 12** (Pending): Balancer State Synchronization & Bundle Building
  - Bundle building from matched orders (TOB only)
  - Consensus integration
  - Plan: `.cursor/plans/plan-step-12-balancer-state-sync.plan.md`

- **Step 13** (Pending): Balancer Math Validation Tests
  - End-to-end integration tests
  - Validation against balancer-maths testData
  - Plan: `.cursor/plans/plan-step-13-balancer-math-validation.plan.md`

- **Step 14** (Pending): Balancer Integration with Matching Engine
  - Wire BalancerPoolManager into matching engine
  - Multi-pool support (Uniswap + Balancer)
  - Plan: `.cursor/plans/plan-step-14-balancer-matching-engine.plan.md`

- **Step 15** (Deferred): Event-Driven Pool Manager
  - Production event-driven architecture
  - Dynamic pool discovery
  - Deferred until production deployment
  - Plan: `.cursor/plans/plan-step-15-event-driven-pool-manager.plan.md`

### Testing Balancer Integration

```bash
# Run all balancer-v3 tests
cargo test -p balancer-v3

# Run integration test with live Base RPC
cargo test -p balancer-v3 test_reclamm_pool_data_loader_base --test test_pool_data_loader -- --ignored --nocapture

# Set custom RPC endpoint
BASE_RPC_URL="https://your-rpc-url" cargo test -p balancer-v3 --test test_pool_data_loader -- --ignored
```
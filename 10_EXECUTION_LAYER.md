## Execution Layer Overview

This document explains what code the Angstrom node executes, how it is assembled, and who controls which responsibilities at runtime.

### What runs (entrypoint)
- **Primary binary**: `bin/angstrom`.
- **Entrypoint**: `bin/angstrom/src/main.rs` calls `angstrom::run()`.
- **Build outputs**: compiled to `target/{debug,release}/angstrom` by Cargo.

### High-level startup sequence
1. **CLI and configuration**
   - Parse `AngstromConfig` (metrics, endpoints, signer config, consensus timing).
   - Initialize chain id via `init_with_chain_id` based on Reth chainspec.
   - Optionally start Prometheus metrics exporter.

2. **Bootstrap Ethereum context**
   - Build an Alloy provider to the `--boot-node` RPC.
   - Read the validator node set from the onchain `ControllerV1` contract.

3. **Build the P2P overlay**
   - Initialize Angstrom network builder via `init_network_builder` (derives peer status and verification sidecar from the signer and validator set).

4. **Launch embedded Reth node**
   - Construct a Reth `EthereumNode` with `AngstromNetworkBuilder` plugged into the network stack.
   - Attach Reth add‑ons and extend JSON‑RPC with Angstrom modules:
     - `OrderApi` (orderflow, quoting/validation hooks)
     - `ConsensusApi` (consensus coordination/control)
   - Launch and obtain the `node` handle and exit future.

5. **Initialize Angstrom services (post‑launch)**
   `initialize_strom_components` wires internal services against Reth’s provider/streams and the network:
   - Ethereum data ingestion: `EthDataCleanser` (canonical state subscriptions, event decoding)
   - Uniswap pool manager (fetch pools, maintain pool state, tick data, streams)
   - Token price generator and price update stream
   - Validation pipeline (`init_validation`) over Reth DB + live streams
   - Order pool and `PoolManager` (storage, updates, network integration)
   - Matching engine (`MatchingManager`)
   - AMM quoter (`QuoterManager`)
   - Consensus manager (`ConsensusManager`) with validator weights and timing
   - Submission handler (handles bundle/mev‑boost and submission nodes)
   - Global block sync coordinator used across modules

### Who controls what
- **Reth (execution client)**
  - Ethereum P2P and chain sync
  - EVM execution and on‑disk DB
  - Canonical state notifications and base JSON‑RPC server

- **Angstrom (this repo)**
  - Custom P2P overlay (`AngstromNetworkBuilder`) for validator coordination
  - Order pool, matching engine, validation, AMM quoting
  - Consensus orchestration and onchain submission
  - Extra JSON‑RPC modules: `OrderApi`, `ConsensusApi`
  - Startup/bootstrap logic (contract reads, pool discovery, config stores)

### How to run
From the workspace root:
```bash
cargo run -p angstrom -- \
  --boot-node https://eth.drpc.org \
  --normal-nodes=https://rpc.ankr.com/eth \
  --metrics-enabled \
  --local-secret-key-location /path/to/key.hex
```

Or use an HSM signer with the `--hsm-*` flags.

### Process lifecycle and shutdown
- Services are spawned on Reth’s task executor with critical supervision.
- The node runs until the exit future resolves; graceful shutdown signals propagate to services like consensus.

### Source map (key files)
- Binary entry and assembly: `bin/angstrom/src/main.rs`, `bin/angstrom/src/lib.rs`
- Service wiring and startup: `bin/angstrom/src/components.rs`
- CLI and signer config: `bin/angstrom/src/cli.rs`
- Core subsystems: `crates/*` (e.g., `angstrom-net`, `consensus`, `matching-engine`, `validation`, `rpc`, `types`)

### Notes
- `bin/` holds executable crates (CLI apps). Most logic lives in `crates/*` and is composed by the `bin/angstrom` node binary.
- Built artifacts are emitted under `target/`, not `bin/`.



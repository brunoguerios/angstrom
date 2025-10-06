## Angstrom Bundle Data Flow and PADE Encoding

### Overview

- **What is sent**: The Rust-side `AngstromBundle`.
- **How it’s sent**: The bundle is PADE-encoded and passed as the single `bytes` argument to the contract’s `execute(bytes)`.
- **Where it’s decoded**: Inside `Angstrom.unlockCallback(bytes)` using specialized calldata readers for each section.

Related docs: [Payload Types](contracts/docs/payload-types.md) · [PADE Encoding](contracts/docs/pade-encoding-format.md) · [`Angstrom.sol`](contracts/src/Angstrom.sol)

### Bundle Shape (Rust)

```rust
pub struct AngstromBundle {
    pub assets:              Vec<Asset>,
    pub pairs:               Vec<Pair>,
    pub pool_updates:        Vec<PoolUpdate>,
    pub top_of_block_orders: Vec<TopOfBlockOrder>,
    pub user_orders:         Vec<UserOrder>,
}
```

### Encoding and Submission (Rust)

The bundle is PADE-encoded and wrapped into the ABI call data for `Angstrom.execute(bytes)`:

```rust
let encoded = Angstrom::executeCall::new((bundle.pade_encode().into(),)).abi_encode();
```

This encoded payload is used to build and sign the transaction that calls the on-chain `execute(bytes)`.

See: `crates/types/src/submission/mod.rs`

### Contract Entry and Decoding (Solidity)

`execute(bytes)` authenticates the caller and forwards the raw PADE payload to Uniswap v4’s `unlock`, which calls back into `unlockCallback(bytes)` on `Angstrom` with the same data:

```solidity
function execute(bytes calldata encoded) external {
    _nodeBundleLock();
    if (encoded.length == 0) return;
    UNI_V4.unlock(encoded);
}

function unlockCallback(bytes calldata data) external override returns (bytes memory) {
    _onlyUniV4();

    CalldataReader reader = CalldataReaderLib.from(data);

    AssetArray assets;
    (reader, assets) = AssetLib.readFromAndValidate(reader);
    PairArray pairs;
    (reader, pairs) = PairLib.readFromAndValidate(reader, assets, _configStore);

    _takeAssets(assets);
    reader = _updatePools(reader, pairs);
    reader = _validateAndExecuteToBOrders(reader, pairs);
    reader = _validateAndExecuteUserOrders(reader, pairs);
    reader.requireAtEndOf(data);
    _saveAndSettle(assets);

    return new bytes(0);
}
```

See: `contracts/src/Angstrom.sol`

### Processing Order

1. Read and validate `assets` (sorted; determines `save`/`take`/`settle`).
2. Read and validate `pairs` (clearing prices; sorted; config store access).
3. Apply `pool_updates` against Uniswap v4.
4. Validate and execute `top_of_block_orders`.
5. Validate and execute `user_orders`.
6. Check that decoding consumed all bytes; finalize and settle balances.

### Key Points

- The bundle contents are encoded with PADE for calldata efficiency; the transaction itself remains ABI-encoded.
- The same bytes passed to `execute(bytes)` are forwarded to `unlockCallback(bytes)` for decoding and execution.
- Validation (e.g., ordering, price consistency, limits) is enforced during decoding/execution, with some invariants delegated to off-chain economics for gas efficiency.



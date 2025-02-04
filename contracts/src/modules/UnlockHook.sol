// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {IBeforeSwapHook} from "../interfaces/IHooks.sol";
import {UniConsumer} from "./UniConsumer.sol";
import {TopLevelAuth} from "./TopLevelAuth.sol";
import {PoolConfigStoreLib} from "../libraries/PoolConfigStore.sol";

import {PoolKey} from "v4-core/src/types/PoolKey.sol";
import {IPoolManager} from "v4-core/src/interfaces/IPoolManager.sol";
import {BeforeSwapDelta} from "v4-core/src/types/BeforeSwapDelta.sol";
import {LPFeeLibrary} from "v4-core/src/libraries/LPFeeLibrary.sol";

/// @author philogy <https://github.com/philogy>
abstract contract UnlockHook is UniConsumer, TopLevelAuth, IBeforeSwapHook {
    error CannotSwapWhileLocked();

    function beforeSwap(
        address,
        PoolKey calldata key,
        IPoolManager.SwapParams calldata,
        bytes calldata
    ) external view returns (bytes4 response, BeforeSwapDelta delta, uint24 swapFee) {
        _onlyUniV4();
        if (!_isUnlocked()) revert CannotSwapWhileLocked();
        swapFee = _unlockedFee(_addr(key.currency0), _addr(key.currency1))
            | LPFeeLibrary.OVERRIDE_FEE_FLAG;
        return (IBeforeSwapHook.beforeSwap.selector, delta, swapFee);
    }
}

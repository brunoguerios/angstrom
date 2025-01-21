// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {IBeforeSwapHook} from "../interfaces/IHooks.sol";
import {UniConsumer} from "../modules/UniConsumer.sol";

import {PoolKey} from "v4-core/src/types/PoolKey.sol";
import {BalanceDelta} from "v4-core/src/types/BalanceDelta.sol";
import {IPoolManager} from "v4-core/src/interfaces/IPoolManager.sol";
import {BeforeSwapDelta} from "v4-core/src/types/BeforeSwapDelta.sol";

/// @author philogy <https://github.com/philogy>
abstract contract SandwichResistantHook is UniConsumer, IBeforeSwapHook {
    function beforeSwap(
        address sender,
        PoolKey calldata key,
        IPoolManager.SwapParams calldata params,
        bytes calldata hookData
    ) external returns (bytes4 response, BeforeSwapDelta, uint24) {
        _onlyUniV4();

        BeforeSwapDelta delta;

        return (IBeforeSwapHook.beforeSwap.selector, delta, 0);
    }
}

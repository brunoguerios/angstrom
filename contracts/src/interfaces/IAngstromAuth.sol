// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

import {StoreKey} from "../libraries/PoolConfigStore.sol";
import {PoolConfigStore} from "../libraries/PoolConfigStore.sol";

interface IAngstromAuth {
    function setController(address newController) external;
    function configurePool(
        address assetA,
        address assetB,
        uint16 tickSpacing,
        uint24 bundleFee,
        uint24 unlockedFee
    ) external;
    function removePool(StoreKey key, PoolConfigStore expectedStore, uint256 storeIndex) external;
    function pullFee(address asset, uint256 amount) external;
    function toggleNodes(address[] calldata nodes) external;
    function extsload(uint256 slot) external view returns (uint256);
}

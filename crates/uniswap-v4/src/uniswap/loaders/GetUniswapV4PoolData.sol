//SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {IERC20} from "forge-std/interfaces/IERC20.sol";
import {IPoolManager} from "v4-core/src/interfaces/IPoolManager.sol";
import {PoolId} from "v4-core/src/types/PoolId.sol";
import {Slot0} from "v4-core/src/types/Slot0.sol";
import {IUniV4} from "core/src/interfaces/IUniV4.sol";

contract GetUniswapV4PoolData {
    struct PoolData {
        uint8 token0Decimals;
        uint8 token1Decimals;
        uint128 liquidity;
        uint160 sqrtPrice;
        int24 tick;
        int128 liquidityNet;
    }

    error NoPoolManager();
    event Start();
    event ValidPoolAddress();
    event GotSlot0(Slot0 slot0);
    event GotLiquidity(uint128 liquidity);
    event GotNetLiquidity(int128 liquidityNet);
    event GotPoolData(PoolData poolData);
    event MadeAbiEncoded(bytes abiEncodedData);
    event FinishedAssembly();

    constructor(
        PoolId poolId,
        address poolManager,
        address asset0,
        address asset1
    ) {
        emit Start();
        if (codeSizeIsZero(poolManager)) revert NoPoolManager();
        emit ValidPoolAddress();
        PoolData memory poolData;

        Slot0 slot0 = IUniV4.getSlot0(IPoolManager(poolManager), poolId);
        emit GotSlot0(slot0);


        uint128 liquidity = IUniV4.getPoolLiquidity(
            IPoolManager(poolManager),
            poolId
        );


        emit GotLiquidity(liquidity);
        (, int128 liquidityNet) = IUniV4.getTickLiquidity(
            IPoolManager(poolManager),
            poolId,
            slot0.tick()
        );
        revert NoPoolManager();

        emit GotNetLiquidity(liquidityNet);
        poolData.token0Decimals = IERC20(asset0).decimals();
        poolData.token1Decimals = IERC20(asset1).decimals();
        poolData.liquidity = liquidity;
        poolData.sqrtPrice = slot0.sqrtPriceX96();
        poolData.tick = slot0.tick();
        poolData.liquidityNet = liquidityNet;
        emit GotPoolData(poolData);
        bytes memory abiEncodedData = abi.encode(poolData);
        emit MadeAbiEncoded(abiEncodedData);
        assembly {
            let dataStart := add(abiEncodedData, 0x20)
            let dataSize := 192
            return(dataStart, dataSize)
        }
        // emit FinishedAssembly();
    }

    function codeSizeIsZero(address target) internal view returns (bool) {
        return target.code.length == 0;
    }
}

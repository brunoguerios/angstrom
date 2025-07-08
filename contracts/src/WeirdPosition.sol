// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {IUniV4} from "./interfaces/IUniV4.sol";
import {IPoolManager} from "./interfaces/IUniV4.sol";

/// @author philogy <https://github.com/philogy>
library WeirdPosition {
    using IUniV4 for IPoolManager;

    IPoolManager internal constant UNI_V4 = IPoolManager(0x000000000004444c5dc75cB358380D2e3dE08A90);
}

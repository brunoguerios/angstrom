// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {ERC20} from "solady/src/tokens/ERC20.sol";

contract MintableMockERC20 is ERC20 {
    string internal _name = "Mock Token";
    string internal _symbol = "MCK";

    function setMeta(string memory newName, string memory newSymbol) external {
        _name = newName;
        _symbol = newSymbol;
    }

    function mint(address to, uint256 amount) external {
        _mint(to, amount);
    }

    function burn(address from, uint256 amount) external {
        _burn(from, amount);
    }

    function name() public view override returns (string memory) {
        return _name;
    }

    function symbol() public view override returns (string memory) {
        return _symbol;
    }
}

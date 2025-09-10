// SPDX-License-Identifier: MIT
pragma solidity 0.8.26;

import {IAngstromAdapter} from "../interfaces/IAngstromAdapter.sol";
import {IPoolManager} from "v4-core/src/interfaces/IPoolManager.sol";
import {PoolKey} from "v4-core/src/types/PoolKey.sol";
import {Currency} from "v4-core/src/types/Currency.sol";
import {BalanceDelta} from "v4-core/src/types/BalanceDelta.sol";
import {TickMath} from "v4-core/src/libraries/TickMath.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";

/// @title AngstromAdapter
/// @notice Adapter contract that handles attestation selection and executes swaps on Angstrom-protected Uniswap v4 pools
/// @author Jet Jadeja <jjadeja@usc.edu>
contract AngstromAdapter is IAngstromAdapter {
    /// @notice The Uniswap v4 PoolManager
    IPoolManager public immutable poolManager;

    /// @notice Temporary storage for swap parameters during unlock callback
    struct SwapCallbackData {
        PoolKey poolKey;
        bool zeroForOne;
        uint128 amountIn;
        uint128 minAmountOut;
        bytes hookData;
        address recipient;
    }

    constructor(IPoolManager _poolManager) {
        poolManager = _poolManager;
    }

    /// @inheritdoc IAngstromAdapter
    function swap(
        address router,
        PoolKey calldata key,
        bool zeroForOne,
        uint128 amountIn,
        uint128 minAmountOut,
        Attestation[] calldata bundle,
        address recipient,
        uint256 deadline
    ) external returns (uint256 amountOut) {
        // check deadline
        require(block.timestamp <= deadline, "DEADLINE_EXPIRED");
        
        // Select the correct attestation for the current block
        bytes memory attestation = _selectAttestation(bundle);
        
        // Pull input tokens from msg.sender
        Currency inputCurrency = zeroForOne ? key.currency0 : key.currency1;
        IERC20(Currency.unwrap(inputCurrency)).transferFrom(msg.sender, address(this), amountIn);
        
        // Package swap parameters and initiate unlock
        SwapCallbackData memory callbackData = SwapCallbackData({
            poolKey: key,
            zeroForOne: zeroForOne,
            amountIn: amountIn,
            minAmountOut: minAmountOut,
            hookData: attestation,
            recipient: recipient
        });
        
        bytes memory encodedData = abi.encode(callbackData);
        poolManager.unlock(encodedData);
        
        // TODO: Return output amount
    }

    /// @notice Callback function called by PoolManager during unlock
    /// @param data Encoded swap parameters
    /// @return bytes Empty bytes to satisfy interface
    function unlockCallback(bytes calldata data) external returns (bytes memory) {
        // Security check - only PoolManager can call this
        require(msg.sender == address(poolManager), "UNAUTHORIZED_CALLBACK");
        
        // Decode the parameters we encoded in swap()
        SwapCallbackData memory params = abi.decode(data, (SwapCallbackData));
        
        // TODO: Execute swap with attestation as hookData
        // TODO: Settle input tokens (pay debt)
        // TODO: Take output tokens (claim credit)
        // TODO: Verify minimum output satisfied
        // TODO: Store output amount for return
        return "";
    }

    /// @notice Selects the correct attestation for the current block
    /// @param bundle Array of attestations to choose from
    /// @return unlockData The attestation data for the current block
    function _selectAttestation(Attestation[] calldata bundle) internal view returns (bytes memory unlockData) {
        uint256 currentBlock = block.number;
        
        for (uint256 i = 0; i < bundle.length; i++) {
            if (bundle[i].blockNumber == currentBlock) {
                return bundle[i].unlockData;
            }
        }
        
        revert("MISSING_ATTESTATION_FOR_CURRENT_BLOCK");
    }

    /// @notice Executes the actual swap on the PoolManager
    /// @param key The pool key
    /// @param zeroForOne The swap direction
    /// @param amountIn The input amount
    /// @param hookData The attestation data to pass to the hook
    /// @return delta The balance delta from the swap
    function _performSwap(
        PoolKey memory key,
        bool zeroForOne,
        uint128 amountIn,
        bytes memory hookData
    ) internal returns (BalanceDelta delta) {
        // Execute the swap on the PoolManager
        delta = poolManager.swap(
            key,
            IPoolManager.SwapParams({
                zeroForOne: zeroForOne,
                amountSpecified: -int256(uint256(amountIn)), // Negative for exact input
                sqrtPriceLimitX96: zeroForOne 
                    ? TickMath.MIN_SQRT_PRICE + 1 
                    : TickMath.MAX_SQRT_PRICE - 1
            }),
            hookData // The attestation that unlocks the pool!
        );
    }

    /// @notice Settles the input token debt with the PoolManager
    /// @param currency The currency to settle
    /// @param amount The amount to settle
    function _settle(Currency currency, uint256 amount) internal {
        // TODO: Implement settlement
        // Transfer tokens to poolManager and call settle
    }

    /// @notice Takes the output tokens from the PoolManager
    /// @param currency The currency to take
    /// @param recipient The recipient address
    /// @param amount The amount to take
    function _take(Currency currency, address recipient, uint256 amount) internal {
        // TODO: Implement take
        // Call poolManager.take to claim tokens
    }
}
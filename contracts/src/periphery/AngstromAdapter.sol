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
        address payer;  // Original caller who pays for the swap
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
        
        // Package swap parameters and initiate unlock
        SwapCallbackData memory callbackData = SwapCallbackData({
            poolKey: key,
            zeroForOne: zeroForOne,
            amountIn: amountIn,
            minAmountOut: minAmountOut,
            hookData: attestation,
            recipient: recipient,
            payer: msg.sender  // store the original caller
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
        
        // Execute swap with attestation as hookData
        BalanceDelta delta = _performSwap(params.poolKey, params.zeroForOne, params.amountIn, params.hookData);
        
        // Settle input tokens (pay the debt)
        // The input currency has a negative delta (we owe tokens)
        Currency inputCurrency = params.zeroForOne ? params.poolKey.currency0 : params.poolKey.currency1;
        int128 inputDelta = params.zeroForOne ? delta.amount0() : delta.amount1();
        require(inputDelta < 0, "INVALID_INPUT_DELTA");
        _settle(inputCurrency, uint256(uint128(-inputDelta)), params.payer);
        
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
    /// @param payer The address to pull tokens from
    function _settle(Currency currency, uint256 amount, address payer) internal {
        // Transfer tokens directly from payer to PoolManager (single transfer!)
        IERC20(Currency.unwrap(currency)).transferFrom(payer, address(poolManager), amount);
        
        // Notify the PoolManager of the settlement
        poolManager.settle();
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
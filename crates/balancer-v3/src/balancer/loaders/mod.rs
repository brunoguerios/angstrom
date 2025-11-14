//! Balancer V3 contract ABIs and data structures

use alloy::{
    primitives::{Address, U256},
    sol
};

// Balancer V3 VaultExplorer contract ABI
sol! {
    #[sol(rpc)]
    #[derive(Debug, PartialEq, Eq)]
    interface IVaultExplorer {
        struct LiquidityManagement {
            bool disableUnbalancedLiquidity;
            bool enableAddLiquidityCustom;
            bool enableRemoveLiquidityCustom;
            bool enableDonation;
        }

        struct PoolConfig {
            LiquidityManagement liquidityManagement;
            uint256 staticSwapFeePercentage;
            uint256 aggregateSwapFeePercentage;
            uint256 aggregateYieldFeePercentage;
            uint24 tokenDecimalDiffs;
            uint32 pauseWindowEndTime;
            bool isPoolRegistered;
            bool isPoolInitialized;
            bool isPoolPaused;
            bool isPoolInRecoveryMode;
        }

        function getPoolConfig(address pool) external view returns (PoolConfig memory);
        function getCurrentLiveBalances(address pool) external view returns (uint256[] memory);
        function getVault() external view returns (address);
    }
}

// Balancer V3 Vault contract ABI (minimal interface for what we need)
sol! {
    #[sol(rpc)]
    #[derive(Debug, PartialEq, Eq)]
    interface IVault {
        struct LiquidityManagement {
            bool disableUnbalancedLiquidity;
            bool enableAddLiquidityCustom;
            bool enableRemoveLiquidityCustom;
            bool enableDonation;
        }

        struct PoolConfig {
            LiquidityManagement liquidityManagement;
            uint256 staticSwapFeePercentage;
            uint256 aggregateSwapFeePercentage;
            uint256 aggregateYieldFeePercentage;
            uint24 tokenDecimalDiffs;
            uint32 pauseWindowEndTime;
            bool isPoolRegistered;
            bool isPoolInitialized;
            bool isPoolPaused;
            bool isPoolInRecoveryMode;
        }

        function getPoolTokenInfo(address pool) external view returns (address[] memory tokens, uint256[] memory scalingFactors);
        function getPoolTokenRates(address pool) external view returns (uint256[] memory decimalScalingFactors, uint256[] memory tokenRates);
        function getPoolConfig(address pool) external view returns (PoolConfig memory);
    }
}

// ReClamm pool contract ABI
sol! {
    #[sol(rpc)]
    #[derive(Debug, PartialEq, Eq)]
    interface IReClamm {
        struct ReClammPoolDynamicData {
            uint256[] balancesLiveScaled18;
            uint256[] tokenRates;
            uint256 staticSwapFeePercentage;
            uint256 totalSupply;
            uint256 lastTimestamp;
            uint256[] lastVirtualBalances;
            uint256 dailyPriceShiftExponent;
            uint256 dailyPriceShiftBase;
            uint256 centerednessMargin;
            uint256 currentPriceRatio;
            uint256 currentFourthRootPriceRatio;
            uint256 startFourthRootPriceRatio;
            uint256 endFourthRootPriceRatio;
            uint32 priceRatioUpdateStartTime;
            uint32 priceRatioUpdateEndTime;
            bool isPoolInitialized;
            bool isPoolPaused;
            bool isPoolInRecoveryMode;
        }

        function getReClammPoolDynamicData() external view returns (ReClammPoolDynamicData memory);
        function isPoolWithinTargetRange() external view returns (bool);
        function computeCurrentVirtualBalances() external view returns (uint256 currentVirtualBalanceA, uint256 currentVirtualBalanceB);
    }
}

/// Aggregated ReClamm pool data loaded from multiple contract calls
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReClammPoolData {
    // From Vault
    pub tokens:             Vec<Address>,
    pub scaling_factors:    Vec<U256>,
    pub token_rates:        Vec<U256>,
    pub swap_fee:           U256,
    pub aggregate_swap_fee: U256,

    // From Pool - State Data
    pub balances_live_scaled18:      Vec<U256>,
    pub total_supply:                U256,
    pub last_timestamp:              U256,
    pub last_virtual_balances:       Vec<U256>,
    pub current_virtual_balances:    Vec<U256>,

    // From Pool - Price Shift Parameters
    pub daily_price_shift_exponent: U256,
    pub daily_price_shift_base:     U256,
    pub centeredness_margin:        U256,

    // From Pool - Current Price State
    pub current_price_ratio:             U256,
    pub current_fourth_root_price_ratio: U256,
    pub start_fourth_root_price_ratio:   U256,
    pub end_fourth_root_price_ratio:     U256,
    pub price_ratio_update_start_time:   u32,
    pub price_ratio_update_end_time:     u32,

    // From Pool - Status Flags
    pub is_pool_initialized:         bool,
    pub is_pool_paused:              bool,
    pub is_pool_in_recovery_mode:    bool,
    pub is_pool_within_target_range: bool,

    // Computed
    pub current_timestamp: U256
}

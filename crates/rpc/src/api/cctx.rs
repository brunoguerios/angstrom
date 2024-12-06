use angstrom_types::sol_bindings::grouped_orders::AllOrders;
use jsonrpsee::{core::RpcResult, proc_macros::rpc};

use crate::types::cctx::{CandleTimeframe, MarketContext, PoolCandle, TickerContext, TokenContext};

#[cfg_attr(not(feature = "client"), rpc(server, namespace = "angstrom"))]
#[cfg_attr(feature = "client", rpc(server, client, namespace = "angstrom"))]
#[async_trait::async_trait]
pub trait DataApi {
    /// Fetches a list of all available markets
    #[method(name = "fetchMarkets")]
    async fn all_markets(&self) -> RpcResult<MarketContext>;

    /// Fetches a list of all available currencies
    #[method(name = "fetchCurrencies")]
    async fn all_currencies(&self) -> RpcResult<Vec<TokenContext>>;

    /// Fetches a list of all available pools with stats
    #[method(name = "fetchTickers")]
    async fn all_pools_with_stats(&self) -> RpcResult<Vec<TickerContext>>;

    /// Fetches a pool with stats
    #[method(name = "fetchTicker")]
    async fn pool_with_stats(&self, pool_ticker: String) -> RpcResult<Vec<TickerContext>>;

    /// Fetches historical candlestick data containing the open, high, low, and
    /// close price, and the volume of a pool
    #[method(name = "fetchOHLCV")]
    async fn pool_candles(
        &self,
        pool_ticker: String,
        timeframe: CandleTimeframe,
        start_block: Option<u64>,
        end_block: Option<u64>
    ) -> RpcResult<PoolCandle>;

    // get the list of most recent executed orders for a particular pool
    #[method(name = "fetchTrades")]
    async fn get_executed_orders(
        &self,
        pool_ticker: String,
        timeframe: CandleTimeframe,
        start_block: Option<u64>,
        end_block: Option<u64>
    ) -> RpcResult<Vec<AllOrders>>;
}

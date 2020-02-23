use xtp::XTPMarketDataStruct;

#[derive(Debug, Clone)]
pub enum QuoteEvent {
    MarketDepth {
        market_data: XTPMarketDataStruct,
        bid1_qty: Vec<i64>,
        max_bid1_count: i32,
        ask1_qty: Vec<i64>,
        max_ask1_count: i32,
    },
}

use super::quote_event::QuoteEvent;
use log::{error, info, warn};
use std::sync::Mutex;
use tokio::sync::mpsc::Sender;
use xtp::{
    OrderBookStruct, QuoteSpi, XTPMarketDataStruct, XTPRspInfoStruct, XTPSpecificTickerStruct,
    XTPTickByTickStruct,
};

type XTPST = XTPSpecificTickerStruct;
type XTPRI = XTPRspInfoStruct;
pub struct QSpi {
    tx: Mutex<Sender<QuoteEvent>>,
}

impl QSpi {
    pub fn new(tx: Sender<QuoteEvent>) -> Self {
        QSpi { tx: Mutex::new(tx) }
    }
}

impl QuoteSpi for QSpi {
    fn on_error(&self, error_info: XTPRI) {
        error!("{:?}", error_info);
    }

    fn on_disconnected(&self, reason: i32) {
        warn!("Disconnected, reason: {}", reason)
    }

    fn on_sub_market_data(&self, ticker: XTPST, error_info: XTPRI, is_last: bool) {
        info!(
            "Sub Market Data: {:?}: {:?} {}",
            ticker, error_info, is_last
        );
    }

    fn on_sub_order_book(&self, ticker: XTPST, error_info: XTPRI, is_last: bool) {
        info!("Sub Orderbook: {:?}: {:?} {}", ticker, error_info, is_last);
    }

    fn on_sub_tick_by_tick(&self, ticker: XTPST, error_info: XTPRI, is_last: bool) {
        info!(
            "Sub Tick By Tick: {:?}: {:?} {}",
            ticker, error_info, is_last
        );
    }

    fn on_depth_market_data(
        &self,
        market_data: XTPMarketDataStruct,
        bid1_qty: &[i64],
        max_bid1_count: i32,
        ask1_qty: &[i64],
        max_ask1_count: i32,
    ) {
        // info!(
        //     "Market Depth: {:?}, {:?}, {}, {:?}, {}",
        //     market_data, bid1_qty, max_bid1_count, ask1_qty, max_ask1_count
        // );
        if let Err(e) = self.tx.lock().unwrap().try_send(QuoteEvent::MarketDepth {
            market_data,
            bid1_qty: bid1_qty.to_owned(),
            max_bid1_count,
            ask1_qty: ask1_qty.to_owned(),
            max_ask1_count,
        }) {
            error!("Buffer full {:?}", e);
        }
    }
    fn on_tick_by_tick(&self, tbt_data: XTPTickByTickStruct) {
        info!("Tick by tick: {:?}", tbt_data);
    }

    fn on_order_book(&self, ob: OrderBookStruct) {
        info!("Orderbook: {:?}", ob);
    }
}

use super::trader_event::TraderEvent;
use log::error;
use tokio::sync::mpsc::Sender;
use xtp::{TraderSpi, XTPRspInfoStruct, XTPSpecificTickerStruct};

type XTPST = XTPSpecificTickerStruct;
type XTPRI = XTPRspInfoStruct;
pub struct TSpi {
    tx: Sender<TraderEvent>,
}

impl TSpi {
    pub fn new(tx: Sender<TraderEvent>) -> Self {
        TSpi { tx }
    }
}

impl TraderSpi for TSpi {
    fn on_error(&self, error_info: XTPRI) {
        error!("{:?}", error_info);
    }
}

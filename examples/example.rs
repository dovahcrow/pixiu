use async_trait::async_trait;
use dotenv::dotenv;
use env_logger::init;
use failure::Fallible;
use futures::stream::StreamExt;
use log::{error, info, trace, warn};
use pixiu::{Exchange, Strategy, XTPExchange};
use std::net::SocketAddrV4;
use std::thread::sleep;
use std::time::Duration;
use structopt::StructOpt;
use tokio::sync::broadcast::Receiver;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of xtp-rs usage.")]
struct Args {
    #[structopt(short, long, default_value = "1")]
    id: i8,
    #[structopt(short, long, env = "XTP_QUOTE_ADDR")]
    quote_addr: SocketAddrV4,
    #[structopt(short, long, env = "XTP_TRADER_ADDR")]
    trader_addr: SocketAddrV4,
    #[structopt(short, long, env = "XTP_USERNAME")]
    username: String,
    #[structopt(short, long, env = "XTP_PASSWORD")]
    password: String,
    #[structopt(short, long, env = "XTP_KEY")]
    key: String,
    #[structopt(long, default_value = "/tmp")]
    path: String,
}

#[tokio::main]
async fn main() -> Fallible<()> {
    let _ = dotenv();
    init();

    let args = Args::from_args();

    let mut exch = XTPExchange::new(
        args.quote_addr,
        args.trader_addr,
        &args.username,
        &args.password,
        &args.key,
    );

    exch.register(MyStrategy::new(1));
    exch.register(MyStrategy::new(2));

    tokio::spawn(exch.run());
    sleep(Duration::from_secs(1000));

    Ok(())
}

struct MyStrategy {
    id: usize,
    rx: Option<Receiver<<XTPExchange as Exchange>::Event>>,
    h: Option<<XTPExchange as Exchange>::Handle>,
}

impl MyStrategy {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            rx: None,
            h: None,
        }
    }
}

#[async_trait]
impl Strategy<XTPExchange> for MyStrategy {
    async fn init(
        &mut self,
        rx: Receiver<<XTPExchange as Exchange>::Event>,
        h: <XTPExchange as Exchange>::Handle,
    ) {
        trace!("MyStrategy {} init", self.id);
        self.rx = Some(rx);
        self.h = Some(h);
    }

    async fn run(self: Box<Self>) {
        info!("MyStrategy {} running", self.id);
        let mut rx = self.rx.unwrap();
        while let Some(msg) = rx.next().await {
            info!("MyStrategy {} Received {:?}", self.id, msg);
        }
    }
}

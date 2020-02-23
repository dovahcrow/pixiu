mod exchanges;

pub use crate::exchanges::xtp::XTPExchange;

use async_trait::async_trait;
use tokio::sync::broadcast;

#[async_trait]
pub trait Exchange: Sized {
    type Event;
    type Handle;

    async fn run(self);

    fn register<S>(&mut self, s: S)
    where
        S: for<'a> Strategy<Self> + Send + Sync + 'static;
}

#[async_trait]
pub trait Strategy<E: Exchange> {
    async fn init(&mut self, rx: broadcast::Receiver<E::Event>, h: E::Handle);
    async fn run(self: Box<Self>);
}

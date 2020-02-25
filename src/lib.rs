mod exchanges;

pub use crate::exchanges::xtp::XTPExchange;

use async_trait::async_trait;
use tokio::sync::broadcast::Receiver;

#[async_trait]
pub trait Exchange: Sized {
    type Event;
    type Handle;

    async fn run(self);

    fn register<S>(&mut self, s: S)
    where
        S: Strategy<Self> + Send + Sync + 'static;
}

#[async_trait]
pub trait Strategy<E: Exchange> {
    async fn run(self: Box<Self>, rx: Receiver<E::Event>, h: E::Handle);
}

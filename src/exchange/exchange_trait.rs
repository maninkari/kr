// use crate::exceptions::StreamSubscriptionError;
// use crate::ExchangeWsTcpStream;
use async_trait::async_trait;

#[async_trait]
pub trait Exchange {
    async fn subscribe_to_orderbook_stream();
}

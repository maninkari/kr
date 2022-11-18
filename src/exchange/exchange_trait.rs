// use crate::exceptions::StreamSubscriptionError;
// use crate::ExchangeWsTcpStream;
use async_trait::async_trait;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

pub type ExchangeWS = WebSocketStream<MaybeTlsStream<TcpStream>>;

#[async_trait]
pub trait Exchange {
    async fn subscribe_to_orderbook_stream(&mut self);
}

use super::Exchange;
use crate::ExchangeWS;
use async_trait::async_trait;
use futures::{SinkExt, StreamExt};
// use futures_channel::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{protocol::Message, Error},
};
use url::Url;

/// Represents the Bitstamp exchange
pub struct Bitstamp {
    pub ws: Option<ExchangeWS>,
    pub endpoint: String,
    pub channel: String,
}

impl Bitstamp {
    pub fn new(_endpoint: String, _channel: String) -> Self {
        Bitstamp {
            ws: None,
            endpoint: _endpoint,
            channel: _channel,
        }
    }
}

#[async_trait]
impl Exchange for Bitstamp {
    async fn subscribe_to_orderbook_stream(&mut self) {
        println!("bitstamp");
        let url = Url::parse(&self.endpoint).unwrap();

        let subscription_data: String = format!(
            r#"{{
            "event": "bts:subscribe",
            "data": {{"channel": "{}"}}
        }}"#,
            &self.channel
        );

        let (mut ws, _) = connect_async(url).await.expect("Failed to connect");
        let bitstamp_subscribe_message = ws.send(Message::Text(subscription_data)).await;

        if bitstamp_subscribe_message.is_err() {
            println!("error");
        } else {
            println!("WebSocket handshake has been successfully completed");
        }

        let (w_tx, mut w_rx) = ws.split();

        // transmitter tx, and receiver rx
        let mut tx: UnboundedSender<Result<Message, Error>>;
        let mut rx: UnboundedReceiver<Result<Message, Error>>;
        (tx, rx) = mpsc::unbounded_channel();

        // Reading and broadcasting messages
        while let Some(result) = w_rx.next().await {
            println!("\n ---- \n{}", result.expect("Failed to fetch message"));
        }
    }
}

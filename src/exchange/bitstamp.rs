use super::Exchange;
use crate::ExchangeWS;
use async_trait::async_trait;
use futures::{SinkExt, StreamExt};
use futures_util::{future, pin_mut};
use tokio::io::AsyncWriteExt;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
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

        let (mut ws_stream, _) = connect_async(url).await.expect("Failed to connect");
        let bitstamp_subscribe_message = ws_stream.send(Message::Text(subscription_data)).await;

        if bitstamp_subscribe_message.is_err() {
            println!("error");
        } else {
            println!("WebSocket handshake has been successfully completed");
        }

        let (write, read) = ws_stream.split();

        let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
        tokio::spawn(Bitstamp::read_stdin(stdin_tx));

        let stdin_to_ws = stdin_rx.map(Ok).forward(write);

        let ws_to_stdout = {
            read.for_each(|message| async {
                let data = message.unwrap().into_data();
                // let stream: Resp = serde_json::from_slice(&data).unwrap();
                // let c: f64 = stream.bids[0].p.parse().expect("Not a number!");

                // println!("\n\nlastUpdateId: {:?}", stream.lastUpdateId);
                // println!("\nbids: {:?}\n", c);
                println!("\n --- bitsamp --- \n");
                tokio::io::stdout().write_all(&data).await.unwrap();
            })
        };

        pin_mut!(stdin_to_ws, ws_to_stdout);
        future::select(stdin_to_ws, ws_to_stdout).await;
    }
}

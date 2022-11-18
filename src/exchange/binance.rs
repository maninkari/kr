use super::Exchange;
use crate::ExchangeWS;
use async_trait::async_trait;
use futures::StreamExt;
// use futures_channel::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{protocol::Message, Error},
};
use url::Url;

pub struct Binance {
    pub ws: Option<ExchangeWS>,
    pub endpoint: String,
    pub pair: String,
    pub freq: String,
}

impl Binance {
    pub fn new(_endpoint: String, _pair: String, _freq: String) -> Self {
        Binance {
            ws: None,
            endpoint: _endpoint,
            pair: _pair,
            freq: _freq,
        }
    }
}

#[async_trait]
impl Exchange for Binance {
    // async fn subscribe_to_orderbook_stream(&mut self) {
    //     println!("binance");

    //     let connect_addr = format!("{}{}{}", self.endpoint, self.pair, self.freq);
    //     let url = Url::parse(&connect_addr).unwrap();
    //     let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    //     let (mut write, read) = ws_stream.split();

    //     let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();

    //     tokio::spawn(Binance::read_stdin(stdin_tx));

    //     let stdin_to_ws = stdin_rx.map(Ok).forward(write);

    //     let ws_to_stdout = {
    //         read.for_each(|message| async {
    //             let data = message.unwrap().into_data();
    //             // let stream: Resp = serde_json::from_slice(&data).unwrap();
    //             // let c: f64 = stream.bids[0].p.parse().expect("Not a number!");

    //             // println!("\n\nlastUpdateId: {:?}", stream.lastUpdateId);
    //             // println!("\nbids: {:?}\n", c);

    //             tokio::io::stdout().write_all(&data).await.unwrap();
    //         })
    //     };

    //     pin_mut!(stdin_to_ws, ws_to_stdout);
    //     future::select(stdin_to_ws, ws_to_stdout).await;
    // }

    async fn subscribe_to_orderbook_stream(&mut self) {
        println!("binance");

        let connect_addr = format!("{}{}{}", self.endpoint, self.pair, self.freq);
        let url = Url::parse(&connect_addr).unwrap();
        let (mut ws, _) = connect_async(url).await.expect("Failed to connect");
        let (w_tx, mut w_rx) = ws.split();

        // transmitter tx, and receiver rx
        let mut tx: UnboundedSender<Result<Message, Error>>;
        let mut rx: UnboundedReceiver<Result<Message, Error>>;
        (tx, rx) = mpsc::unbounded_channel();

        // Reading and broadcasting messages
        while let Some(result) = w_rx.next().await {
            println!("\n oooo \n{}", result.expect("Failed to fetch message"));
        }
    }
}

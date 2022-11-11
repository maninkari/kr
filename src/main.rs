mod exchange;
mod proto;

use crate::exchange::*;
use futures::{SinkExt, StreamExt};
use futures_util::{future, pin_mut};
use protobuf::Message as ProtoMsg;
use serde::Deserialize;
use std::env;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::{
    connect_async, connect_async_tls_with_config, tungstenite::protocol::Message, WebSocketStream,
};
use url::Url;

use proto::messages::Summary;

#[derive(Deserialize, Debug)]
pub struct Duo {
    pub p: String,
    pub q: String,
}

#[derive(Deserialize, Debug)]
pub struct Resp {
    pub lastUpdateId: i64,
    pub bids: Vec<Duo>,
    pub asks: Vec<Duo>,
}

// #[tokio::main]
// async fn main() {
//     println!("main...");

//     // TODO: pasar url como parametro
//     // let connect_addr = env::args().nth(1).unwrap_or_else(|| panic!("this program requires at least one argument"));
//     let connect_addr = env::args()
//         .nth(1)
//         .unwrap_or_else(|| "wss://stream.binance.com:9443/ws/ethbtc@depth20@1000ms".to_string());
//     // .unwrap_or_else(|| "wss://ws.bitstamp.net/ethbtc".to_string());

//     println!("connect_addr: {}", connect_addr);

//     let url = Url::parse(&connect_addr).unwrap();
//     let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
//     println!("WebSocket handshake has been successfully completed");
//     let (write, read) = ws_stream.split();

//     let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
//     tokio::spawn(read_stdin(stdin_tx));

//     let stdin_to_ws = stdin_rx.map(Ok).forward(write);

//     let ws_to_stdout = {
//         read.for_each(|message| async {
//             let data = message.unwrap().into_data();
//             let stream: Resp = serde_json::from_slice(&data).unwrap();
//             let c: f64 = stream.bids[0].p.parse().expect("Not a number!");

//             println!("\n\nlastUpdateId: {:?}", stream.lastUpdateId);
//             println!("\nbids: {:?}\n", c);

//             tokio::io::stdout().write_all(&data).await.unwrap();
//         })
//     };

//     pin_mut!(stdin_to_ws, ws_to_stdout);

//     // awaits for one of the two futures to be ready
//     future::select(stdin_to_ws, ws_to_stdout).await;
// }

#[tokio::main]
async fn main() {
    println!("main...");

    // TODO: pasar url como parametro
    // let connect_addr = env::args()
    //     .nth(1)
    //     .unwrap_or_else(|| panic!("this program requires at least one argument"));
    // connect_to_ws(
    //     "wss://stream.binance.com:9443/ws/".to_owned(),
    //     "ethbtc".to_owned(),
    //     "@depth20@1000ms".to_owned(),
    // )
    // .await;

    connect_to_ws(
        "wss://ws.bitstamp.net/".to_owned(),
        "ethbtc".to_owned(),
        "".to_owned(),
    )
    .await;

    // Binance::subscribe_to_orderbook_stream().await;
    // Bitstamp::subscribe_to_orderbook_stream().await
}

async fn connect_to_ws(url_ws: String, pair: String, freq: String) {
    let connect_addr = format!("{}", url_ws);
    println!("connect_addr: {}", connect_addr);

    let subscription_data: String = format!(
        r#"{{
        "event": "bts:subscribe",
        "data": {{"channel": "{}"}}
    }}"#,
        pair
    );

    println!("subscription_data: {}", subscription_data);

    let url = Url::parse(&connect_addr).unwrap();
    let (mut ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    let bitstamp_subscribe_message = ws_stream.send(Message::Text(subscription_data)).await;

    if bitstamp_subscribe_message.is_err() {
        println!("error");
    } else {
        println!("bien");
    }

    println!("WebSocket handshake has been successfully completed");
    let (write, read) = ws_stream.split();

    let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
    tokio::spawn(read_stdin(stdin_tx));

    let stdin_to_ws = stdin_rx.map(Ok).forward(write);

    let ws_to_stdout = {
        read.for_each(|message| async {
            let data = message.unwrap().into_data();
            let stream: Resp = serde_json::from_slice(&data).unwrap();
            let c: f64 = stream.bids[0].p.parse().expect("Not a number!");

            println!("\n\nlastUpdateId: {:?}", stream.lastUpdateId);
            println!("\nbids: {:?}\n", c);

            tokio::io::stdout().write_all(&data).await.unwrap();
        })
    };

    pin_mut!(stdin_to_ws, ws_to_stdout);
}

async fn read_stdin(tx: futures_channel::mpsc::UnboundedSender<Message>) {
    let mut stdin = tokio::io::stdin();
    loop {
        let mut buf = vec![0; 1024];
        let n = match stdin.read(&mut buf).await {
            Err(_) | Ok(0) => break,
            Ok(n) => n,
        };
        buf.truncate(n);
        tx.unbounded_send(Message::binary(buf)).unwrap();
    }
}

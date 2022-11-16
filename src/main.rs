mod exchange;
mod proto;

use crate::exchange::*;
use proto::messages::Summary;
use protobuf::Message as ProtoMsg;
use serde::Deserialize;
use std::env;

#[tokio::main]
async fn main() {
    println!("main...");

    let mut binance = Binance::new(
        "wss://stream.binance.com:9443/ws/".to_owned(),
        "ethbtc".to_owned(),
        "@depth20".to_owned(),
    );

    let mut bitstamp = Bitstamp::new(
        "wss://ws.bitstamp.net/".to_owned(),
        "order_book_ethbtc".to_owned(),
    );

    binance.subscribe_to_orderbook_stream().await;
    bitstamp.subscribe_to_orderbook_stream().await;
}

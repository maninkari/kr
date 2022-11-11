use super::Exchange;
use async_trait::async_trait;

pub struct Binance;

#[async_trait]
impl Exchange for Binance {
    async fn subscribe_to_orderbook_stream() {
        println!("binance");
    }
}

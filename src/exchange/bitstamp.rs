use super::Exchange;
use async_trait::async_trait;
// use futures::{SinkExt, StreamExt};

/// Represents the Bitstamp exchange
pub struct Bitstamp;

#[async_trait]
impl Exchange for Bitstamp {
    async fn subscribe_to_orderbook_stream() {
        // let currency_pair: String = format!("order_book_{}");
        // let subscription_data: String = format!(
        //     r#"{{
        //     "event": "bts:subscribe",
        //     "data": {{"channel": "{}"}}
        // }}"#,
        //     currency_pair
        // );
        // let bitstamp_subscribe_message = active_stream.send(Message::Text(subscription_data)).await;

        // if bitstamp_subscribe_message.is_err() {
        //     return Err(StreamSubscriptionError::from(
        //         "Could not subscribe to the Bitstamp stream",
        //     ));
        // }
        // Ok(Some(active_stream.next().await.unwrap().unwrap()));

        println!("bitstamp");
    }
}

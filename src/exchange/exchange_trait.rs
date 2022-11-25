// use crate::exceptions::StreamSubscriptionError;
// use crate::ExchangeWsTcpStream;
use async_trait::async_trait;
use tokio::{io::AsyncReadExt, net::TcpStream};
use tokio_tungstenite::{tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream};

pub type ExchangeWS = WebSocketStream<MaybeTlsStream<TcpStream>>;

#[async_trait]
pub trait Exchange {
    async fn subscribe_to_orderbook_stream(&mut self);

    async fn read_stdin(tx: futures_channel::mpsc::UnboundedSender<Message>) {
        let mut stdin = tokio::io::stdin();
        println!("read_stdin Exchange");

        loop {
            println!("\n---\n");
            let mut buf = vec![0; 1024];
            let n = match stdin.read(&mut buf).await {
                Err(_) | Ok(0) => break,
                Ok(n) => n,
            };
            buf.truncate(n);
            tx.unbounded_send(Message::binary(buf)).unwrap();
        }
    }
}

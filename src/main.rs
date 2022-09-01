use std::env;
use futures_util::{future, pin_mut, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    // let connect_addr = env::args().nth(1).unwrap_or_else(|| panic!("this program requires at least one argument"));
    let connect_addr = env::args().nth(1).unwrap_or_else(|| "wss://stream.binance.com:9443/ws/ethbtc@depth20@100ms".to_string());
    println!("connect_addr: {}", connect_addr);

    let url= Url::parse(&connect_addr).unwrap();

    let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
    tokio::spawn(read_stdin(stdin_tx));

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");

    let (write, read) = ws_stream.split();

    let stdin_to_ws = stdin_rx.map(Ok).forward(write);
    
    let ws_to_stdout = {
        read.for_each(|message| async {
            let data = message.unwrap().into_data();
            println!("praaa");
            tokio::io::stdout().write_all(&data).await.unwrap();
        })
    };

    pin_mut!(stdin_to_ws, ws_to_stdout);
    future::select(stdin_to_ws, ws_to_stdout).await;
}

async fn read_stdin(tx: futures_channel::mpsc::UnboundedSender<Message>) {
    let mut stdin = tokio::io::stdin();
    loop {
        println!("loop");
        let mut buf = vec![0; 1024];
        let n = match stdin.read(&mut buf).await {
            Err(_) | Ok(0) => break,
            Ok(n) => n,
        };
        buf.truncate(n);
        println!("praaa buf {:?}", buf);
        tx.unbounded_send(Message::binary(buf)).unwrap();
    }
}
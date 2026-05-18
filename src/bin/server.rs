use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{Sender, channel};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {

    // Receiver khusus untuk client ini
    let mut bcast_rx = bcast_tx.subscribe();

    loop {
        tokio::select! {

            // Menerima pesan dari client
            msg = ws_stream.next() => {
                match msg {
                    Some(Ok(msg)) => {
                        if let Some(text) = msg.as_text() {
                            let full_msg = format!("{addr}: {text}");

                            println!("{full_msg}");

                            // broadcast ke semua client
                            let _ = bcast_tx.send(full_msg);
                        }
                    }

                    Some(Err(e)) => {
                        eprintln!("websocket error: {e}");
                        break;
                    }

                    None => {
                        println!("Client disconnected: {addr}");
                        break;
                    }
                }
            }

            // Menerima broadcast server
            result = bcast_rx.recv() => {
                match result {
                    Ok(msg) => {
                        ws_stream.send(Message::text(msg)).await?;
                    }

                    Err(e) => {
                        eprintln!("broadcast error: {e}");
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (bcast_tx, _) = channel(16);

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("listening on port 8080");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {addr:?}");
        let bcast_tx = bcast_tx.clone();
        tokio::spawn(async move {
            // Wrap the raw TCP stream into a websocket.
            let (_req, ws_stream) = ServerBuilder::new().accept(socket).await?;

            handle_connection(addr, ws_stream, bcast_tx).await
        });
    }
}
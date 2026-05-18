use futures_util::SinkExt;
use futures_util::stream::StreamExt;
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), tokio_websockets::Error> {
    let (mut ws_stream, _) =
        ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:2000"))
            .connect()
            .await?;

    let stdin = tokio::io::stdin();
    let mut stdin = BufReader::new(stdin).lines();


    loop {
        tokio::select! {

            // Input user
            line = stdin.next_line() => {
                match line {
                    Ok(Some(text)) => {
                        ws_stream.send(Message::text(text)).await?;
                    }

                    Ok(None) => {
                        break;
                    }

                    Err(e) => {
                        eprintln!("stdin error: {e}");
                        break;
                    }
                }
            }

            // Pesan dari server
            msg = ws_stream.next() => {
                match msg {
                    Some(Ok(msg)) => {
                        if let Some(text) = msg.as_text() {
                            println!("{text}");
                        }
                    }

                    Some(Err(e)) => {
                        eprintln!("Websocket error: {e}");
                        break;
                    }

                    None => {
                        println!("Server disconnected");
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}
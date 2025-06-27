// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eolib::data::decode_number;
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tauri::State;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::{
        broadcast,
        mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
        Mutex,
    },
};
use tokio_tungstenite::{accept_async, tungstenite::Message, WebSocketStream};

#[derive(Default)]
struct ProxyState {
    shutdown_tx: Option<broadcast::Sender<()>>,
}

#[tauri::command]
async fn start_proxy(
    bind_host: String,
    bind_port: u16,
    target_host: String,
    state: State<'_, Arc<Mutex<ProxyState>>>,
) -> Result<(), String> {
    let addr = format!("{}:{}", bind_host, bind_port);
    let listener = TcpListener::bind(&addr).await.map_err(|e| e.to_string())?;

    let (shutdown_tx, mut shutdown_rx) = broadcast::channel(1);
    state.lock().await.shutdown_tx = Some(shutdown_tx.clone());

    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = shutdown_rx.recv() => {
                    break;
                }
                Ok((stream, _)) = listener.accept() => {
                    let target = target_host.clone();
                    tokio::spawn(async move {
                        if let Err(e) = accept_connection(stream, target).await {
                            println!("Error accepting connection: {}", e);
                        }
                    });
                }
            }
        }
    });

    Ok(())
}

#[tauri::command]
async fn stop_proxy(state: State<'_, Arc<Mutex<ProxyState>>>) -> Result<(), String> {
    if let Some(tx) = &state.lock().await.shutdown_tx {
        let _ = tx.send(());
    }
    Ok(())
}

async fn accept_connection(
    stream: TcpStream,
    target_host: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut eosocket = TcpStream::connect(&target_host).await?;

    let websocket = accept_async(stream).await?;

    let (tx, rx) = unbounded_channel::<Vec<u8>>();
    let mut ws = WS::new(websocket, rx);

    loop {
        tokio::select! {
            Some(message) = ws.socket.next() => match message {
                Ok(Message::Binary(buf)) => {
                    if eosocket.write(&buf).await.is_err() {
                        break;
                    }
                }
                Ok(Message::Close(_)) | Err(_) => break,
                _ => {}
            },
            result = read_eo_packet(&mut eosocket, &tx) => {
                match result {
                    Ok(Some(0)) => break, // server closed
                    Ok(Some(_)) => {}
                    Ok(None) | Err(_) => break,
                }
            }
            Some(buf) = ws.rx.recv() => {
                if ws.socket.send(Message::Binary(buf)).await.is_err() {
                    break;
                }
            }
        }
    }

    Ok(())
}

async fn read_eo_packet(
    eosocket: &mut TcpStream,
    tx: &UnboundedSender<Vec<u8>>,
) -> Result<Option<usize>, Box<dyn std::error::Error + Send + Sync>> {
    let mut length_buf = vec![0; 2];
    eosocket.read_exact(&mut length_buf).await?;

    let length = decode_number(&length_buf);
    let mut buf = vec![0; length as usize];
    let mut total_read = 0;

    while total_read < length as usize {
        let n = eosocket.read(&mut buf[total_read..]).await?;
        if n == 0 {
            // connection closed
            return Ok(None);
        }
        total_read += n;
    }

    // Prepend length_buf to the actual data
    let mut full_buf = length_buf;
    full_buf.extend_from_slice(&buf);

    let _ = tx.send(full_buf);

    Ok(Some(total_read + 2)) // total bytes read including header
}

struct WS {
    socket: WebSocketStream<TcpStream>,
    rx: UnboundedReceiver<Vec<u8>>,
}

impl WS {
    fn new(socket: WebSocketStream<TcpStream>, rx: UnboundedReceiver<Vec<u8>>) -> Self {
        Self { socket, rx }
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .manage(Arc::new(Mutex::new(ProxyState::default())))
        .invoke_handler(tauri::generate_handler![start_proxy, stop_proxy])
        .run(tauri::generate_context!())
        .expect("error while running tauri app");
}

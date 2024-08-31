use std::{fmt, net::SocketAddr, sync::Arc};

use anyhow::Result;
use dashmap::DashMap;
use futures::{stream::SplitStream, SinkExt, StreamExt};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::mpsc,
};
use tokio_util::codec::{Framed, LinesCodec};
use tracing::{info, level_filters::LevelFilter, warn};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

const MAX_MESSAGES: usize = 128;

#[derive(Debug, Default)]
struct State {
    peers: DashMap<SocketAddr, mpsc::Sender<Arc<Message>>>,
}

#[derive(Debug)]
struct Peer {
    username: String,
    stream: SplitStream<Framed<TcpStream, LinesCodec>>,
}

#[derive(Debug)]
enum Message {
    UserJoined(String),
    UserLeft(String),
    Chat { sender: String, content: String },
}

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();
    // console_subscriber::init();

    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await?;
    info!("Starting chat server on {}", addr);
    let state = Arc::new(State::default());

    loop {
        let (stream, addr) = listener.accept().await?;
        info!("Accepted connection from: {}", addr);
        let state_cloned = Arc::clone(&state);
        tokio::spawn(async move {
            if let Err(e) = handle_client(state_cloned, addr, stream).await {
                warn!("Failed to handle client {}: {}", addr, e);
            }
        });
    }
}

async fn handle_client(state: Arc<State>, addr: SocketAddr, stream: TcpStream) -> Result<()> {
    let mut stream = Framed::new(stream, LinesCodec::new());
    stream.send("Enter your username:").await?;

    let username = match stream.next().await {
        Some(Ok(username)) => username,
        Some(Err(e)) => return Err(e.into()),
        _ => {
            warn!("Failed to read username from client: {}", addr);
            return Ok(());
        }
    };

    let mut peer = state.add(addr, username, stream).await;
    let message = Arc::new(Message::user_joined(&peer.username));
    info!("{}", message);

    state.broadcast(addr, message).await;

    while let Some(line) = peer.stream.next().await {
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                warn!("Failed to read line from client {}: {}", addr, e);
                break;
            }
        };

        let message = Arc::new(Message::Chat {
            sender: peer.username.clone(),
            content: line,
        });

        info!("{}", message);
        state.broadcast(addr, message).await;
    }

    // when while loop exit, peer has left the chat or line reading failed
    // remove peer from state
    state.peers.remove(&addr);

    // notify other peers that this peer has left the chat
    let message = Arc::new(Message::UserLeft(peer.username.clone()));
    info!("{}", message);
    state.broadcast(addr, message).await;

    Ok(())
}

impl State {
    async fn broadcast(&self, addr: SocketAddr, message: Arc<Message>) {
        for peer in self.peers.iter() {
            if peer.key() != &addr {
                if let Err(e) = peer.value().send(Arc::clone(&message)).await {
                    warn!("Failed to send message to client {}: {}", peer.key(), e);
                    // if send failed, peer might be gone, remove peer from state
                    self.peers.remove(peer.key());
                }
            }
        }
    }

    async fn add(
        &self,
        addr: SocketAddr,
        username: String,
        stream: Framed<TcpStream, LinesCodec>,
    ) -> Peer {
        let (tx, mut rx) = mpsc::channel(MAX_MESSAGES);
        self.peers.insert(addr, tx);

        let (mut reader, writer) = stream.split();

        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                if let Err(e) = reader.send(message.to_string()).await {
                    warn!("Failed to send message to client {}: {}", addr, e);
                    break;
                }
            }
        });

        Peer {
            username,
            stream: writer,
        }
    }
}

impl Message {
    fn user_joined(username: &str) -> Self {
        Message::UserJoined(username.to_string())
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Message::UserJoined(username) => write!(f, "{} joined the chat", username),
            Message::UserLeft(username) => write!(f, "{} left the chat", username),
            Message::Chat { sender, content } => write!(f, "{}: {}", sender, content),
        }
    }
}

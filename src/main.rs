include!(concat!(env!("OUT_DIR"), "/lib.rs"));

mod config;

use std::sync::Arc;

use anyhow::Context;
use prost::Message;
use tokio::sync::{broadcast::Sender, Mutex};
use webtransport_quinn::Session;

use crate::config::load;

lazy_static::lazy_static! {
    static ref MESSAGES: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    static ref BROADCAST: Arc<Mutex<Sender<()>>> = Arc::new(Mutex::new(Sender::new(1024)));
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env = env_logger::Env::default().default_filter_or("info");
    env_logger::init_from_env(env);

    let config = load()?;

    let addr = "[::]:4443".parse().unwrap();
    log::info!("listening on {}", addr);

    let server = quinn::Endpoint::server(config, addr)?;

    // Accept new connections.
    while let Some(conn) = server.accept().await {
        tokio::spawn(async move {
            let err = run_conn(conn).await;
            if let Err(err) = err {
                log::error!("connection failed: {}", err)
            }
        });
    }

    Ok(())
}

async fn run_conn(conn: quinn::Connecting) -> anyhow::Result<()> {
    log::info!("received new QUIC connection");

    // Wait for the QUIC handshake to complete.
    let conn = conn.await.context("failed to accept connection")?;
    log::info!("established QUIC connection");

    // Perform the WebTransport handshake.
    let request = webtransport_quinn::accept(conn).await?;
    log::info!("received WebTransport request: {}", request.url());

    // Accept the session.
    let session = request.ok().await.context("failed to accept session")?;
    log::info!("accepted session");

    // Run the session
    if let Err(err) = run_session(session).await {
        log::info!("closing session: {}", err);
    }

    Ok(())
}

async fn run_session(session: Session) -> anyhow::Result<()> {
    let mut update = BROADCAST.lock().await.subscribe();

    loop {
        // Wait for a bidirectional stream or datagram.
        tokio::select! {
            res = session.accept_bi() => {
                let (mut send, mut recv) = res?;
                log::info!("accepted stream");

                // Read the message and echo it back.
                let msg = recv.read_to_end(1024).await?;
                log::info!("recv: {}", String::from_utf8_lossy(&msg));
                MESSAGES.lock().await.push(String::from_utf8_lossy(&msg).to_string());
                BROADCAST.lock().await.send(()).unwrap();

                send.write_all(&msg).await?;

                log::info!("send: {}", String::from_utf8_lossy(&msg));
            },
            res = session.read_datagram() => {
                let msg = res?;
                log::info!("accepted datagram");
                log::info!("recv: {}", String::from_utf8_lossy(&msg));

                session.send_datagram(msg.clone()).await?;
                log::info!("send: {}", String::from_utf8_lossy(&msg));
            },
            snd = update.recv() => {
                log::info!("sending update");
                if let Ok(()) = snd {
                    let messages = MESSAGES.lock().await.clone();

                    let req = crate::chat::Messages {
                        messages,
                    };

                    let data = req.encode_to_vec();

                    let mut writer =  session.open_uni().await?;
                    writer.write_all(&data).await?;
                    log::info!("send messages");
                }
            }
        };

        log::info!("echo successful!");
    }
}

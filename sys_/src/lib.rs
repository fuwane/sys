//! FuwaNe System

use futures::SinkExt;
use tokio::io::{ AsyncRead, AsyncWrite };
use tokio_tungstenite::{
    WebSocketStream
};

use songbird::Call;

pub mod types;
pub mod impl_;
pub mod source;
pub mod plugin;


pub struct Client<S: AsyncRead + AsyncWrite + Unpin> {
    pub ws: WebSocketStream<S>,
    pub call: Call
}

impl<S: AsyncRead + AsyncWrite + Unpin> Client<S> {
    pub async fn new(ws: WebSocketStream<S>) -> Self {
        ws.send()
        Self { ws: ws, call: }
    }
    
}
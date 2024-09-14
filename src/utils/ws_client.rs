use std::net::TcpStream;

use tokio_tungstenite::tungstenite::{connect, stream::MaybeTlsStream, WebSocket};

static WS_URL: &str = "ws://localhost:3030/locs";

pub fn init_server_connection() -> WebSocket<MaybeTlsStream<TcpStream>> {
    let (socket, _) = connect(WS_URL).expect("Can't connect");
    socket
}

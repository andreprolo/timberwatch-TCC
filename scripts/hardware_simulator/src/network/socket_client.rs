use crate::consts::SOCKET_URL;
use std::net::TcpStream;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{connect, Message, WebSocket};
use url::Url;

pub fn connect_to_webscoket() -> WebSocket<MaybeTlsStream<TcpStream>> {
    let (socket, _response) = connect(Url::parse(SOCKET_URL).unwrap()).expect("Failed to connect!");
    socket
}

pub fn join_channel(
    socket: &mut WebSocket<MaybeTlsStream<TcpStream>>,
    channel: String,
    payload: json::JsonValue,
) {
    let message = json::array!["3", "3", channel, "phx_join", payload];
    send_message(socket, json::stringify(message));
}

pub fn push(
    socket: &mut WebSocket<MaybeTlsStream<TcpStream>>,
    channel: String,
    topic: String,
    value: json::JsonValue,
) {
    let message = json::array!["3", "5", channel, topic, value];
    send_message(socket, json::stringify(message));
}

fn send_message(socket: &mut WebSocket<MaybeTlsStream<TcpStream>>, message: String) {
    socket.write_message(Message::Text(message)).unwrap();
}

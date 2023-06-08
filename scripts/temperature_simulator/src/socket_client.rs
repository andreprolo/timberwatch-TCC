use std::net::TcpStream;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{connect, Message, WebSocket};
use url::Url;

pub fn connect_to_webscoket() -> WebSocket<MaybeTlsStream<TcpStream>> {
    let (socket, _response) =
        connect(Url::parse(crate::SOCKET_URL).unwrap()).expect("Failed to connect!");
    socket
}

pub fn join_channel(socket: &mut WebSocket<MaybeTlsStream<TcpStream>>, channel: String) {
    let message: String = r#"["3","3",""#.to_owned() + &channel + r#"","phx_join",{}]"#;
    send_message(socket, message);
}

pub fn push_temperature(
    socket: &mut WebSocket<MaybeTlsStream<TcpStream>>,
    channel: String,
    temperature: f64,
) {
    let message: String = r#"["3","5",""#.to_owned()
        + &channel
        + r#"","new_temperature",{"new_temperature": ""#
        + temperature.to_string().as_str()
        + r#""}]"#;
    send_message(socket, message);
}

fn send_message(socket: &mut WebSocket<MaybeTlsStream<TcpStream>>, message: String) {
    socket.write_message(Message::Text(message)).unwrap();
}

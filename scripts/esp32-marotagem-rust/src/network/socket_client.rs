use log::*;
use std::time::Duration;
use esp_idf_svc::ws::client::{EspWebSocketClient, EspWebSocketClientConfig, WebSocketEvent};
use embedded_svc::ws::FrameType;
use esp_idf_svc::errors::EspIOError;

pub fn connect_to_websocket() -> Result<EspWebSocketClient, EspIOError> {
    // let (socket, _response) = connect(Url::parse(SOCKET_URL).unwrap()).expect("Failed to connect!");

    let url = String::from("ws://172.5.0.183:4000/socket/websocket?vsn=2.0.0");
    info!("About to connect via websocket to {}", url);

    let mut client = EspWebSocketClient::new(url, &EspWebSocketClientConfig{
        ..Default::default()
    }, Duration::new(1, 0), callback);

    // client?.send(FrameType::Text(false), r#"["3", "3", "room:monitoring", "phx_join", {"id": "esp32-1", "type": "temperature", "name": "ESP32"}]"#.as_ref());
    // Ok(())

    client
}

fn callback(event: &Result<WebSocketEvent, EspIOError>) {}

pub fn join_channel(
    mut client: &mut EspWebSocketClient,
    channel: &str,
    payload: json::JsonValue,
) {
    let message = json::array!["3", "3", channel, "phx_join", payload];
    client.send(FrameType::Text(false), json::stringify(message).as_ref());
}

pub fn push(
    mut client: &mut EspWebSocketClient,
    channel: &str,
    topic: &str,
    value: json::JsonValue,
) {
    let message = json::array!["3", "5", channel, topic, value];
    info!("Sending via websocket: {}", message);
    client.send(FrameType::Text(false), json::stringify(message).as_ref());
}

// fn send_message(socket: &mut WebSocket<MaybeTlsStream<TcpStream>>, message: String) {
//     socket.write_message(Message::Text(message)).unwrap();
// }

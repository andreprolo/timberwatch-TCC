use log::*;
use std::time::Duration;
use esp_idf_svc::ws::client::{EspWebSocketClient, EspWebSocketClientConfig, WebSocketEvent};
use embedded_svc::ws::FrameType;
use esp_idf_svc::errors::EspIOError;
use std::thread::sleep;
use crate::consts::{TIMBERWATCH_SOCKET_URL};

pub fn connect_to_websocket() -> Result<EspWebSocketClient, EspIOError> {
    sleep(Duration::new(5, 0));

    let url = String::from(TIMBERWATCH_SOCKET_URL);
    info!("About to connect via websocket to {}", url);

    let client = EspWebSocketClient::new(url, &EspWebSocketClientConfig{
        ..Default::default()
    }, Duration::new(1, 0), callback);

    sleep(Duration::new(5, 0));
    client
}

fn callback(_event: &Result<WebSocketEvent, EspIOError>) {}

pub fn join_channel(
    client: &mut EspWebSocketClient,
    channel: &str,
    payload: json::JsonValue,
) {
    let message = json::array!["3", "3", channel, "phx_join", payload];
    let _ = client.send(FrameType::Text(false), json::stringify(message).as_ref());
}

pub fn push(
    client: &mut EspWebSocketClient,
    channel: &str,
    topic: &str,
    value: json::JsonValue,
) {
    let message = json::array!["3", "5", channel, topic, value];
    info!("Sending via websocket: {}", message);
    let _ = client.send(FrameType::Text(false), json::stringify(message).as_ref());
}

// fn send_message(socket: &mut WebSocket<MaybeTlsStream<TcpStream>>, message: String) {
//     socket.write_message(Message::Text(message)).unwrap();
// }

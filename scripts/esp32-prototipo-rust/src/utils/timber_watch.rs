use esp_idf_svc::ws::client::EspWebSocketClient;
use crate::network;
use crate::consts::TIMBERWATCH_MONITORING_TOPIC;

pub fn connect_sensor(id: &str, name: &str, sensor_type: &str) -> EspWebSocketClient {
    let mut client = network::socket_client::connect_to_websocket().unwrap();
    network::socket_client::join_channel(&mut client, TIMBERWATCH_MONITORING_TOPIC,
        json::object! {
            id: id,
            type: sensor_type,
            name: name
        }
    );
    client
}

pub fn send_metric(mut socket: &mut EspWebSocketClient, value: f32) {
    network::socket_client::push(&mut socket, TIMBERWATCH_MONITORING_TOPIC, "new_metric", json::object! {
        new_value: value
    });
}

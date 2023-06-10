use crate::consts::MONITORING_CHANNEL;
use crate::network::socket_client;
use crate::utils;
use rand::Rng;
use std::net::TcpStream;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::WebSocket;

pub fn execute(sensor_id: String, sensor_name: String) {
    println!(
        "{} - {} - Starting vibration simulation...\n",
        sensor_name, sensor_id
    );

    let mut socket = create_and_connect_socket(sensor_id, sensor_name);

    let mut rng = rand::thread_rng();
    let mut vibration_detected = false;

    loop {
        // utils::sleep(rng.gen_range(100..250));
        utils::sleep(100);

        let vibration_level = if vibration_detected {
            if rng.gen_bool(0.1) {
                vibration_detected = false;
            }

            rng.gen_range(0..=1000)
        } else {
            if rng.gen_bool(0.1) {
                vibration_detected = true;
            }

            0
        };

        // println!("Value: {}", vibration_level);

        socket_client::push(
            &mut socket,
            MONITORING_CHANNEL.to_string(),
            "new_metric".to_string(),
            json::object! {
                new_value: vibration_level.to_string()
            },
        );
    }
}

fn create_and_connect_socket(
    sensor_id: String,
    sensor_name: String,
) -> WebSocket<MaybeTlsStream<TcpStream>> {
    let mut socket = socket_client::connect_to_webscoket();
    socket_client::join_channel(
        &mut socket,
        MONITORING_CHANNEL.to_string(),
        json::object! {
            id: sensor_id,
            type: "vibration",
            name: sensor_name
        },
    );

    socket
}

use crate::consts::{MONITORING_CHANNEL, SIMULATION_INTERVAL};
use crate::network::socket_client;
use crate::utils;
use rand::Rng;
use std::net::TcpStream;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::WebSocket;

pub fn execute(sensor_id: String, sensor_name: String, mode: String) {
    println!(
        "{} - {} - Starting generic sensor simulation...\n",
        sensor_name, sensor_id
    );

    let mut socket = create_and_connect_socket(sensor_id, sensor_name);

    let mut rng = rand::thread_rng();

    loop {
        utils::sleep(SIMULATION_INTERVAL);

        let value = rng.gen_range(0.0..100.0);

        if mode == "verbose" {
            println!("Value: {:.2}", value);
        }

        socket_client::push(
            &mut socket,
            MONITORING_CHANNEL.to_string(),
            "new_metric".to_string(),
            json::object! {
                new_value: value
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
            type: "generic",
            name: sensor_name
        },
    );

    socket
}

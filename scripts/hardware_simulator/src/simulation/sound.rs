use crate::consts::{MONITORING_CHANNEL, SIMULATION_INTERVAL};
use crate::network::socket_client;
use crate::utils;
use rand::Rng;
use std::net::TcpStream;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::WebSocket;

pub fn execute(sensor_id: String, sensor_name: String, mode: String) {
    println!(
        "{} - {} - Starting sound simulation...\n",
        sensor_name, sensor_id
    );

    let mut socket = create_and_connect_socket(sensor_id, sensor_name);

    let mut rng = rand::thread_rng();

    let mut sound = rng.gen_range(40.0..75.0);

    loop {
        utils::sleep(SIMULATION_INTERVAL);

        sound += rng.gen_range(0.0..2.0) - 1.0;

        if sound > 80.0 {
            sound = 80.0;
        } else if sound < 40.0 {
            sound = 40.0;
        }

        if mode == "verbose" {
            println!("Sound: {:.2} dB", sound);
        }

        socket_client::push(
            &mut socket,
            MONITORING_CHANNEL.to_string(),
            "new_metric".to_string(),
            json::object! {
                new_value: sound
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
            type: "sound",
            name: sensor_name
        },
    );

    socket
}

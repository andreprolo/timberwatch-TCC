use crate::consts::{MONITORING_CHANNEL, SIMULATION_INTERVAL};
use crate::network::socket_client;
use crate::utils;
use crate::utils::noise;
use rand::Rng;
use std::net::TcpStream;
use std::time::Instant;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::WebSocket;

pub fn execute(sensor_id: String, sensor_name: String, mode: String) {
    println!(
        "{} - {} - Starting temperature simulation...\n",
        sensor_name, sensor_id
    );

    let mut socket = create_and_connect_socket(sensor_id, sensor_name);

    let execution_start = Instant::now();
    let noise = noise::generate_perlin_noise();

    loop {
        utils::sleep(SIMULATION_INTERVAL);

        let temperature =
            noise::generate_temperature(&noise, execution_start.elapsed().as_secs_f64() / 1000.0);

        if mode == "verbose" {
            println!("Temperature: {:.2} °C", temperature);
        }

        socket_client::push(
            &mut socket,
            MONITORING_CHANNEL.to_string(),
            "new_metric".to_string(),
            json::object! {
                new_value: temperature
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
            type: "temperature",
            name: sensor_name
        },
    );

    socket
}

use crate::consts::MONITORING_CHANNEL;
use crate::network::socket_client;
use crate::utils;
use crate::utils::noise;
use rand::Rng;
use std::net::TcpStream;
use std::time::Instant;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::WebSocket;

pub fn execute(sensor_id: String, sensor_name: String) {
    println!(
        "{} - {} - Starting generic sensor simulation...\n",
        sensor_name, sensor_id
    );

    let mut socket = create_and_connect_socket(sensor_id, sensor_name);

    let execution_start = Instant::now();
    let noise = noise::generate_perlin_noise();
    let mut rng = rand::thread_rng();

    loop {
        // utils::sleep(rng.gen_range(100..250));
        utils::sleep(100);

        let value = rng.gen_range(0.0..100.0);
        // println!("Value: {:.2}", value);

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

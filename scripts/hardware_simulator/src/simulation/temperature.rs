use std::net::TcpStream;
use std::time::Instant;

use rand::Rng;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::WebSocket;

use crate::utils;

use crate::consts::MONITORING_CHANNEL;
use crate::network::socket_client;
use crate::utils::noise;

pub fn execute() {
    println!("Starting temperature simulation...\n");

    let mut socket = create_and_connect_socket();

    let execution_start = Instant::now();
    let noise = noise::generate_perlin_noise();
    let mut rng = rand::thread_rng();

    loop {
        utils::sleep(rng.gen_range(100..250));

        let temperature =
            noise::generate_temperature(&noise, execution_start.elapsed().as_secs_f64() / 1000.0);
        println!("Temperature: {:.2}", temperature);

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

fn create_and_connect_socket() -> WebSocket<MaybeTlsStream<TcpStream>> {
    let mut socket = socket_client::connect_to_webscoket();
    socket_client::join_channel(
        &mut socket,
        MONITORING_CHANNEL.to_string(),
        json::object! {
            id: "teste",
            type: "temperature",
            name: "Serra 1"
        },
    );

    socket
}

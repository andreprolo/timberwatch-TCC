use crate::utils;

use crate::consts::MONITORING_CHANNEL;
use crate::network::socket_client;
use crate::utils::noise;

pub fn execute() {
    println!("Starting temperature simulation...\n");
    let mut socket = socket_client::connect_to_webscoket();
    socket_client::join_channel(&mut socket, MONITORING_CHANNEL.to_string());
    let noise = noise::generate_perlin_noise();
    let mut counter = 0.0001;

    loop {
        utils::sleep(100);
        let temperature = noise::generate_temperature(&noise, counter);
        println!("{}", temperature);

        socket_client::push(
            &mut socket,
            MONITORING_CHANNEL.to_string(),
            "new_temperature".to_string(),
            json::object! {
                new_temperature: temperature.to_string()
            },
        );
        counter += 0.0001;
    }
}

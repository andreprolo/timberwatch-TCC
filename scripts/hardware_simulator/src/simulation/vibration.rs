use crate::utils;

use crate::consts::MONITORING_CHANNEL;
use crate::network::socket_client;
use rand::Rng;

pub fn execute() {
    println!("Starting vibration simulation...\n");
    let mut socket = socket_client::connect_to_webscoket();
    socket_client::join_channel(
        &mut socket,
        MONITORING_CHANNEL.to_string(),
        json::object! {
            id: "teste2",
            type: "vibration",
            name: "Serra 1"
        },
    );

    let mut rng = rand::thread_rng();
    let mut vibration_detected = false;

    loop {
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

        println!("{}", vibration_level);

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

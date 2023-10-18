use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use log::*;

use std::time::Duration;
use std::thread::sleep;

use anyhow::{bail, Result};

mod network;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    network::wifi::connect_to_wifi("Aluno Unisep", "alunounisep");
    // network::wifi::connect_to_wifi("Segfy", "12fyseg3");

    sleep(Duration::new(5, 0));
    let mut client = network::socket_client::connect_to_websocket().unwrap();
    sleep(Duration::new(5, 0));

    network::socket_client::join_channel(&mut client, "room:monitoring",
        json::object! {
            id: "esp32-1",
            type: "temperature",
            name: "ESP32" 
        }
    );

    let mut temperature = 30.0;
    loop {
        network::socket_client::push(&mut client, "room:monitoring", "new_metric", json::object! {
            new_value: temperature
        });

        temperature += 0.1;
        sleep(Duration::new(1, 0));
    }
}

use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use embedded_svc::wifi::ClientConfiguration;
use embedded_svc::wifi::Configuration;
use esp_idf_svc::wifi::EspWifi;
use esp_idf_hal::prelude::Peripherals;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::eventloop::EspSystemEventLoop;
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

    // network::wifi::connect_to_wifi("Aluno Unisep", "alunounisep");
    // network::wifi::connect_to_wifi("Segfy", "12fyseg3");
    let ssid = "Aluno Unisep";
    let password = "alunounisep";

    let peripherals = Peripherals::take().unwrap();
    let sys_loop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();

    let mut wifi_driver = EspWifi::new(
        peripherals.modem,
        sys_loop,
        Some(nvs)
    ).unwrap();

    wifi_driver.set_configuration(&Configuration::Client(ClientConfiguration{
        ssid: ssid.into(),
        password: password.into(),
        ..Default::default()
    })).unwrap();

    wifi_driver.start().unwrap();
    wifi_driver.connect().unwrap();

    while !wifi_driver.is_connected().unwrap(){
        let config = wifi_driver.get_configuration().unwrap();
        println!("Waiting for station {:?}", config);
    }
    info!("Should be connected now");
    println!("IP info: {:?}", wifi_driver.sta_netif().get_ip_info().unwrap());

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

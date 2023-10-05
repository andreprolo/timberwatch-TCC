use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use log::*;

use std::time::Duration;
use std::thread::sleep;

use anyhow::{bail, Result};

use esp_idf_hal::prelude::Peripherals;
use embedded_svc::wifi::ClientConfiguration;
use embedded_svc::wifi::Configuration;
use esp_idf_svc::wifi::EspWifi;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::eventloop::EspSystemEventLoop;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sys_loop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();

    let mut wifi_driver = EspWifi::new(
        peripherals.modem,
        sys_loop,
        Some(nvs)
    ).unwrap();

    wifi_driver.set_configuration(&Configuration::Client(ClientConfiguration{
        ssid: "Segfy".into(),
        password: "12fyseg3".into(),
        ..Default::default()
    })).unwrap();

    wifi_driver.start().unwrap();
    wifi_driver.connect().unwrap();

    while !wifi_driver.is_connected().unwrap(){
        let config = wifi_driver.get_configuration().unwrap();
        println!("Waiting for station {:?}", config);
    }
    println!("Should be connected now");

    loop {
        println!("IP info: {:?}", wifi_driver.sta_netif().get_ip_info().unwrap());

        call_server();

        sleep(Duration::new(10,0));
    }
}

fn call_server() -> anyhow::Result<()> {
    use embedded_svc::http::{self, client::*, status, Headers, Status};
    use embedded_svc::io::Read;
    use embedded_svc::utils::io;
    use esp_idf_svc::http::client::*;

    let url = String::from("http://192.168.100.191:4000");

    info!("About to fetch content from {}", url);

    let mut client = Client::wrap(EspHttpConnection::new(&Configuration {
        crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach),

        ..Default::default()
    })?);

    let mut response = client.get(&url)?.submit()?;

    let mut body = [0_u8; 3048];

    let read = io::try_read_full(&mut response, &mut body).map_err(|err| err.0)?;

    info!(
        "Body (truncated to 3K):\n{:?}",
        String::from_utf8_lossy(&body[..read]).into_owned()
    );

    // Complete the response
    while response.read(&mut body)? > 0 {}

    Ok(())
}

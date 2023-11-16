use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use std::time::Duration;
use std::thread::sleep;
use log::error;
use consts::{WIFI_SSID, WIFI_PASSWORD, TIMBERWATCH_SENSOR_ID, TIMBERWATCH_SENSOR_TYPE, TIMBERWATCH_SENSOR_NAME};
use esp_idf_hal::peripherals::Peripherals;

mod network;
mod utils;
mod consts;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let mut _wifi_driver = network::wifi::connect_to_wifi(WIFI_SSID, WIFI_PASSWORD);

    let mut socket = utils::timber_watch::connect_sensor(TIMBERWATCH_SENSOR_ID, TIMBERWATCH_SENSOR_NAME, TIMBERWATCH_SENSOR_TYPE);

    loop {
        match utils::sensor::get_temperature() {
            Some(t) => {
                utils::timber_watch::send_metric(&mut socket, t);
            }
            None => {
                error!("Failed to read sensor");
            }
        }

        sleep(Duration::new(1, 0));
    }
}

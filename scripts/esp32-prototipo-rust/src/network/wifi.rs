use embedded_svc::wifi::ClientConfiguration;
use embedded_svc::wifi::Configuration;
use esp_idf_svc::wifi::EspWifi;
use esp_idf_hal::prelude::Peripherals;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_hal::adc::config::Config;
use esp_idf_hal::adc::Atten11dB;
use esp_idf_hal::adc::*;
use esp_idf_hal::gpio::Gpio32;
use esp_idf_hal::gpio::Gpio33;
use log::*;

pub fn connect_to_wifi<'a>(ssid: &str, password: &str) -> Option<EspWifi<'a>> {
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


    let mut adc1: AdcDriver<ADC1> =
    AdcDriver::new(peripherals.adc1, &Config::new().calibration(true)).ok()?;

    let mut adc_pin32: esp_idf_hal::adc::AdcChannelDriver<'_, Gpio32, Atten11dB<_>> =
    AdcChannelDriver::new(peripherals.pins.gpio32).ok()?;
    let mut adc_pin33: esp_idf_hal::adc::AdcChannelDriver<'_, Gpio33, Atten11dB<_>> =
    AdcChannelDriver::new(peripherals.pins.gpio33).ok()?;

    // loop {
    //     thread::sleep(Duration::from_millis(10));

    let adc_pin32_result = adc1.read(&mut adc_pin32).ok()?;
    let adc_pin33_result = adc1.read(&mut adc_pin33).ok()?;

    println!("Pin 32: {}, Pin 33: {}", adc_pin32_result, adc_pin33_result);
    // }


    Some(wifi_driver)
}

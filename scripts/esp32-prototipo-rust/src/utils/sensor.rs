use esp_idf_hal::adc::config::Config;
use esp_idf_hal::adc::Atten11dB;
use esp_idf_hal::adc::*;
use esp_idf_hal::gpio::Gpio32;
use esp_idf_hal::gpio::Gpio33;
use esp_idf_hal::peripherals::Peripherals;

pub fn get_temperature() -> Option<f32> {
    // let peripherals = Peripherals::take().unwrap();
    //
    // let mut adc1: Result<AdcDriver<ADC1>, EspError> =
    //     AdcDriver::new(peripherals.adc1, &Config::new().calibration(true));
    //
    // let mut adc_pin32: Result<esp_idf_hal::adc::AdcChannelDriver<'_, Gpio32, Atten11dB<_>>, EspError> =
    //     AdcChannelDriver::new(peripherals.pins.gpio32);
    //
    //
    // match adc1 {
    //     Some(adc) => {
    //         match adc_pin32 {
    //             Some(adc_pin) => {
    //                 let adc_pin32_result = adc1.read(&mut adc_pin32);
    //                 match adc_pin32_result {
    //                     Some(result) => println!("Pin 32 read: {}", adc_pin32_result),
    //                     None => println!("None value in adc_pin32_result"),
    //                 }
    //             },
    //             None => println!("None value in adc_pin32.")
    //         }
    //     },
    //     None => println!("None value in adc1.")
    // }

    let peripherals_option = Peripherals::take();

    match peripherals_option {
        Some(peripherals) => {
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

        },
        None => println!("Peripherals are none."),
    }

    Some(28.3)
}

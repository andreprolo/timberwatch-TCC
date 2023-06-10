use std::process::exit;

use crate::utils;

pub fn welcome_message() {
    println!("\tWelcome to Hardware Simulator!\n");
}

pub fn choose_simulation_type() -> String {
    println!("Select a simulation type:");
    println!("\t1: Single sensor");
    println!("\t2: Multiple sensors");
    println!();
    let option = utils::io::input("Option: ");

    match option.as_str() {
        "1" => "single_sensor",
        "2" => "multiple_sensors",
        _ => "invalid_simulation_type",
    }
    .to_string()
}

pub fn choose_sensor_type() -> String {
    println!("Select a sensor type:");
    println!("\t1: Temperature");
    println!("\t2: Vibration");
    println!("\t3: Sound [in progress...]");
    println!("\t4: Electric [in progress...]");
    println!("\t5: Generic (Random Values)");
    println!();
    let option = utils::io::input("Option: ");

    match option.as_str() {
        "1" => "temperature",
        "2" => "vibration",
        "3" => "sound",
        "4" => "electric",
        _ => "generic",
    }
    .to_string()
}

pub fn get_sensor_name() -> String {
    utils::io::input("Sensor name: ")
}

pub fn get_sensor_id() -> String {
    utils::io::input("Sensor unique identifier (id): ")
}

pub fn get_sensors_amount() -> i32 {
    utils::io::input("Sensors amount: ").parse().unwrap()
}

pub fn proceed_to_simultaion() {
    let proceed = utils::io::input_yes_or_no("Proceed to simulation? [Y/n] ", true);

    if !proceed {
        exit(0);
    }
}

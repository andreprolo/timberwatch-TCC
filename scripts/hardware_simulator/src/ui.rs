use std::process::exit;

use crate::utils;

pub fn welcome_message() {
    println!("\tWelcome to Hardware Simulator!\n");
}

pub fn choose_sensor_type() -> String {
    println!("Select a sensor type:");
    println!("\t1: Temperature");
    println!("\t2: Vibration");
    println!("\t3: Sound");
    println!("\t4: Electric");
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

pub fn proceed_to_simultaion() {
    let proceed = utils::io::input_yes_or_no("Proceed to simulation? [Y/n] ", true);

    if !proceed {
        exit(0);
    }
}

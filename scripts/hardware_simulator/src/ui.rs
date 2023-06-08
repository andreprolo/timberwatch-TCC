use std::process::exit;

use crate::utils;

pub fn welcome_message() {
    println!("\tWelcome to Hardware Simulator!\n");
}

pub fn choose_option() -> String {
    println!("Select a metric to start simulation:");
    println!("\t1: Temperature");
    println!("\t2: Vibration");
    println!("\t3: Sound");
    println!("\t4: Electric");
    println!();
    utils::io::input("Option: ")
}

pub fn proceed_to_simultaion() {
    let proceed = utils::io::input_yes_or_no("Proceed to simulation? [Y/n] ", true);

    if !proceed {
        exit(0);
    }
}

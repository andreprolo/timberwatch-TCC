use std::io;
use std::io::Write;

pub fn input_yes_or_no(message: &str, default_yes: bool) -> bool {
    let ready: String = input(message);
    println!();

    if default_yes {
        ready.trim().to_lowercase() != "n"
    } else {
        ready.trim().to_lowercase() == "y"
    }
}

pub fn input(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error getting input");

    return input.trim().to_string();
}

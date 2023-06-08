use std::{thread, time};

mod generators;
mod socket_client;

pub const SOCKET_URL: &str = "ws://localhost:4000/socket/websocket?vsn=2.0.0";
pub const MONITORING_CHANNEL: &str = "room:monitoring";

fn main() {
    let mut socket = socket_client::connect_to_webscoket();
    socket_client::join_channel(&mut socket, MONITORING_CHANNEL.to_string());
    let noise = generators::generate_perlin_noise();
    let mut counter = 0.0001;

    loop {
        sleep(100);
        let temperature = generators::generate_temperature(&noise, counter);
        println!("{}", temperature);

        socket_client::push_temperature(&mut socket, MONITORING_CHANNEL.to_string(), temperature);
        counter += 0.0001;
    }
}

fn sleep(millis: u64) {
    thread::sleep(time::Duration::from_millis(millis));
}

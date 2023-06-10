use rand::seq::SliceRandom;
use std::thread;
mod consts;
mod network;
mod simulation;
mod ui;
mod utils;

fn main() {
    ui::welcome_message();
    let simulation_type = ui::choose_simulation_type();

    match simulation_type.as_str() {
        "single_sensor" => {
            let sensor_type = ui::choose_sensor_type();

            let sensor_name = ui::get_sensor_name();
            let sensor_id = ui::get_sensor_id();

            ui::proceed_to_simultaion();

            simulation::run(sensor_type, sensor_id, sensor_name, "verbose".to_string());
        }
        "multiple_sensors" => {
            let sensors_ammount = ui::get_sensors_amount();

            let mut sensors = vec![];

            for i in 0..sensors_ammount {
                let sensor_task = thread::spawn(move || {
                    let sensor_name = format!("Sensor {}", i);
                    let sensor_id = format!("sensor-{}", i);
                    let sensor_type = ["temperature", "vibration", "energy", "sound", "generic"]
                        .choose(&mut rand::thread_rng())
                        .unwrap()
                        .to_string();

                    simulation::run(sensor_type, sensor_id, sensor_name, "silent".to_string());
                });
                sensors.push(sensor_task);
                utils::sleep(10);
            }

            for sensor in sensors {
                sensor.join().unwrap();
            }
        }
        _ => panic!("invalid simulation type"),
    }
}

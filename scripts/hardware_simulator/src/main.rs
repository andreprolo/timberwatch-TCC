use std::thread;

use rand::seq::SliceRandom;
use tokio::task;

mod consts;
mod network;
mod simulation;
mod ui;
mod utils;

#[tokio::main]
async fn main() {
    ui::welcome_message();
    let simulation_type = ui::choose_simulation_type();

    match simulation_type.as_str() {
        "single_sensor" => {
            let sensor_type = ui::choose_sensor_type();

            let sensor_name = ui::get_sensor_name();
            let sensor_id = ui::get_sensor_id();

            ui::proceed_to_simultaion();

            simulation::run(sensor_type, sensor_id, sensor_name);
        }
        "multiple_sensors" => {
            let sensors_ammount = ui::get_sensors_amount();

            let mut sensors = vec![];

            for i in 0..sensors_ammount {
                let sensor_task = thread::spawn(move || {
                    let sensor_name = format!("Sensor {}", i);
                    let sensor_id = format!("sensor-{}", i);
                    let sensor_type = ["temperature", "vibration", "generic"]
                        .choose(&mut rand::thread_rng())
                        .unwrap()
                        .to_string();

                    simulation::run(sensor_type, sensor_id, sensor_name);
                });
                sensors.push(sensor_task);
                utils::sleep(10);
            }

            for sensor in sensors {
                sensor.join().unwrap();
            }

            // let sensors_ammount = ui::get_sensors_amount();
            //
            // for i in 0..sensors_ammount {
            //     task::spawn(async move {
            //         let sensor_name = format!("Sensor {}", i);
            //         let sensor_id = format!("sensor-{}", i);
            //         let sensor_type = ["temperature", "vibration", "generic"]
            //             .choose(&mut rand::thread_rng())
            //             .unwrap()
            //             .to_string();
            //
            //         simulation::run(sensor_type, sensor_id, sensor_name);
            //     });
            //     utils::sleep(10);
            // }
            // loop {}
        }
        _ => panic!("invalid simulation type"),
    }
}

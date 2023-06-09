mod consts;
mod network;
mod simulation;
mod ui;
mod utils;

fn main() {
    // simulation::vibration::execute();
    // simulation::temperature::execute();
    ui::welcome_message();
    let sensor_type = ui::choose_sensor_type();

    ui::proceed_to_simultaion();

    match sensor_type.as_str() {
        "temperature" => simulation::temperature::execute(),
        "vibration" => simulation::vibration::execute(),
        _ => panic!("invalid sensor type"),
    };
}

pub mod energy;
pub mod generic;
pub mod sound;
pub mod temperature;
pub mod vibration;

pub fn run(sensor_type: String, sensor_id: String, sensor_name: String, mode: String) {
    match sensor_type.as_str() {
        "temperature" => temperature::execute(sensor_id, sensor_name, mode),
        "vibration" => vibration::execute(sensor_id, sensor_name, mode),
        "energy" => energy::execute(sensor_id, sensor_name, mode),
        "sound" => sound::execute(sensor_id, sensor_name, mode),
        "generic" => generic::execute(sensor_id, sensor_name, mode),
        _ => panic!("invalid sensor type"),
    };
}

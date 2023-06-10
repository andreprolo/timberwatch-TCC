pub mod generic;
pub mod temperature;
pub mod vibration;

pub fn run(sensor_type: String, sensor_id: String, sensor_name: String) {
    match sensor_type.as_str() {
        "temperature" => temperature::execute(sensor_id, sensor_name),
        "vibration" => vibration::execute(sensor_id, sensor_name),
        "generic" => generic::execute(sensor_id, sensor_name),
        _ => panic!("invalid sensor type"),
    };
}

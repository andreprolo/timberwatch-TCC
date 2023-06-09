use noise::{NoiseFn, Perlin};
use rand::prelude::*;

pub fn generate_temperature(perlin: &Perlin, counter: f64) -> f64 {
    perlin.get([0.0, counter]) * 100.0 + 30.0
}

pub fn generate_vibration(perlin: &Perlin, counter: f64) -> f64 {
    remap(perlin.get([0.0, counter]), -1.0, 1.0, 0.0, 1000.0)
}

pub fn generate_perlin_noise() -> Perlin {
    let mut rng = rand::thread_rng();
    let seed: u32 = rng.gen();
    Perlin::new(seed)
}

// Remap a value from one range to another
fn remap(value: f64, in_min: f64, in_max: f64, out_min: f64, out_max: f64) -> f64 {
    (value - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

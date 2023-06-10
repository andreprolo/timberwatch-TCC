use noise::{NoiseFn, Perlin};
use rand::prelude::*;

pub fn generate_temperature(perlin: &Perlin, counter: f64) -> f64 {
    perlin.get([0.0, counter]) * 100.0 + 30.0
}

pub fn generate_energy_variation(perlin: &Perlin, counter: f64) -> f64 {
    perlin.get([0.0, counter]) * 10.0
}

pub fn generate_perlin_noise() -> Perlin {
    let mut rng = rand::thread_rng();
    let seed: u32 = rng.gen();
    Perlin::new(seed)
}

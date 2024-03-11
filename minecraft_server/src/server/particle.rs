use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Particle {
    id: i32,
    long_distance: bool,
    x: f64,
    y: f64,
    z: f64,
    offset_x: f32,
    offset_y: f32,
    offset_z: f32,
    max_speed: f32,
    particle_count: i32,
    data: ParticleData,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ParticleData {}

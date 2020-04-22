pub mod tuple;

use tuple::Tuple;

pub const EPSILON: f64 = 0.00001;

pub fn near_eq(a: f64, b: f64) -> bool {
    if f64::abs(a - b) < EPSILON {
        true
    } else {
        false
    }
}

#[derive(Copy, Clone)]
pub struct Environment {
    pub gravity: Tuple,
    pub wind: Tuple,
}

#[derive(Copy, Clone)]
pub struct Projectile {
    pub position: Tuple,
    pub velocity: Tuple,
}

pub fn tick(environment: Environment, projectile: Projectile) -> Projectile {
    let position = projectile.position + projectile.velocity;
    let velocity = projectile.velocity + environment.gravity + environment.wind;

    Projectile { position, velocity }
}
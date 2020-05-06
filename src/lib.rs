pub mod camera;
pub mod canvas;
pub mod color;
pub mod computations;
pub mod intersection;
pub mod light;
pub mod material;
pub mod matrix;
pub mod pattern;
pub mod plane;
pub mod ray;
pub mod shape;
pub mod sphere;
pub mod transformation;
pub mod tuple;
pub mod world;

use color::Color;
use tuple::Tuple;

pub const BLACK: Color = Color { red: 0., green: 0., blue: 0. };
pub const WHITE: Color = Color { red: 1., green: 1., blue: 1. };
pub const EPSILON: f64 = 0.00001;
pub const ORIGIN: Tuple = Tuple { x: 0., y: 0., z: 0., w: 1. };

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
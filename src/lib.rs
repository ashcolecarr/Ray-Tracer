pub mod bound;
pub mod camera;
pub mod canvas;
pub mod color;
pub mod computations;
pub mod cone;
pub mod csg;
pub mod cube;
pub mod cylinder;
pub mod group;
pub mod intersection;
pub mod light;
pub mod material;
pub mod matrix;
pub mod obj_file;
pub mod pattern;
pub mod plane;
pub mod ray;
pub mod shape;
pub mod smooth_triangle;
pub mod sphere;
pub mod transformation;
pub mod triangle;
pub mod tuple;
pub mod world;

use color::Color;
use cylinder::Cylinder;
use group::Group;
use matrix::Matrix;
use shape::{Shape, CommonShape};
use sphere::Sphere;
use transformation::*;
use tuple::Tuple;
use lazy_static::lazy_static;
use std::f64::consts::PI;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::RwLock;

pub const BLACK: Color = Color { red: 0., green: 0., blue: 0. };
pub const DEFAULT_RECURSION: i32 = 5;
pub const EPSILON: f64 = 0.00001;
pub const ORIGIN: Tuple = Tuple { x: 0., y: 0., z: 0., w: 1. };
pub const WHITE: Color = Color { red: 1., green: 1., blue: 1. };

static ID_COUNT: AtomicI32 = AtomicI32::new(1);
lazy_static! {
    static ref PARENT_REFERENCES: RwLock<Vec<Shape>> = RwLock::new(Vec::new());
}

static MATRIX_ID: AtomicI32 = AtomicI32::new(1);
lazy_static! {
    static ref CACHED_INVERSES: RwLock<Vec<Matrix>> = RwLock::new(Vec::new());
}

pub fn near_eq(a: f64, b: f64) -> bool {
    if f64::abs(a - b) < EPSILON {
        true
    } else {
        false
    }
}

pub fn generate_object_id() -> i32 {
    ID_COUNT.fetch_add(1, Ordering::Relaxed)
}

pub fn generate_matrix_id() -> i32 {
    MATRIX_ID.fetch_add(1, Ordering::Relaxed)
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

fn hexagon_corner() -> Shape {
    let mut corner = Shape::Sphere(Sphere::new());
    corner.set_transform(translate(0., 0., -1.) * scale(0.25, 0.25, 0.25));

    corner
}

fn hexagon_edge() -> Shape {
    let mut edge = Shape::Cylinder(Cylinder::new());
    edge.set_minimum(0.);
    edge.set_maximum(1.);
    edge.set_transform(translate(0., 0., -1.) * rotate(-(PI / 6.), Axis::Y) *
        rotate(-(PI / 2.), Axis::Z) * scale(0.25, 1., 0.25));

    edge
}

fn hexagon_side() -> Shape {
    let mut side = Shape::Group(Group::new());
    side.add_child(&mut hexagon_corner());
    side.add_child(&mut hexagon_edge());

    side
}

pub fn hexagon() -> Shape {
    let mut hex = Shape::Group(Group::new());

    for n in 0..=5 {
        let mut side = hexagon_side();
        side.set_transform(rotate((n as f64) * (PI / 3.), Axis::Y));
        hex.add_child(&mut side);
    }

    hex
}

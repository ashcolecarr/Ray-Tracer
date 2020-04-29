use super::near_eq;
use super::sphere::Sphere;
use super::tuple::Tuple;

#[derive(Debug)]
pub struct Computations {
    pub t: f64,
    pub object: Sphere,
    pub point: Tuple,
    pub eye_vector: Tuple,
    pub normal_vector: Tuple,
    pub inside: bool,
}

impl PartialEq for Computations {
    fn eq(&self, other: &Self) -> bool {
        near_eq(self.t, other.t) && self.object == other.object &&
            self.point == other.point && self.eye_vector == other.eye_vector &&
            self.normal_vector == other.normal_vector && self.inside == other.inside
    }    
}
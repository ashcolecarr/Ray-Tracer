use super::near_eq;
use super::shape::Shape;
use super::tuple::Tuple;

#[derive(Debug, Clone)]
pub struct Computations {
    pub t: f64,
    pub object: Shape,
    pub point: Tuple,
    pub eye_vector: Tuple,
    pub normal_vector: Tuple,
    pub inside: bool,
    pub over_point: Tuple,
    pub reflect_vector: Tuple,
    pub n1: f64,
    pub n2: f64,
    pub under_point: Tuple,
}

impl PartialEq for Computations {
    fn eq(&self, other: &Self) -> bool {
        near_eq(self.t, other.t) && self.object == other.object &&
            self.point == other.point && self.eye_vector == other.eye_vector &&
            self.normal_vector == other.normal_vector && self.inside == other.inside &&
            self.over_point == other.over_point && self.reflect_vector == other.reflect_vector &&
            near_eq(self.n1, other.n1) && near_eq(self.n2, other.n2) &&
            self.under_point == other.under_point
    }    
}
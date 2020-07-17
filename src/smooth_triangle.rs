use super::bound::Bound;
use super::EPSILON;
use super::generate_object_id;
use super::intersection::Intersection;
use super::material::Material;
use super::matrix::Matrix;
use super::ray::Ray;
use super::shape::Shape;
use super::tuple::Tuple;

#[derive(Debug, Clone)]
pub struct SmoothTriangle {
    id: i32,
    pub transform: Matrix,
    pub material: Material,
    pub casts_shadow: bool,
    pub point1: Tuple,
    pub point2: Tuple,
    pub point3: Tuple,
    pub normal_vector1: Tuple,
    pub normal_vector2: Tuple,
    pub normal_vector3: Tuple,
    pub edge_vector1: Tuple,
    pub edge_vector2: Tuple,
    pub normal_vector: Tuple,
    pub parent: Option<i32>,
}

impl PartialEq for SmoothTriangle {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.transform == other.transform &&
            self.material == other.material && self.casts_shadow == other.casts_shadow &&
            self.point1 == other.point1 && self.point2 == other.point2 &&
            self.point3 == other.point3 && self.normal_vector1 == other.normal_vector1 &&
            self.normal_vector2 == other.normal_vector2 && self.normal_vector3 == other.normal_vector3 && 
            self.edge_vector1 == other.edge_vector1 && self.edge_vector2 == other.edge_vector2 && 
            self.normal_vector == other.normal_vector && self.parent == other.parent
    }
}

impl SmoothTriangle {
    pub fn new(point1: Tuple, point2: Tuple, point3: Tuple,
        normal_vector1: Tuple, normal_vector2: Tuple, normal_vector3: Tuple) -> Self {

        let edge_vector1 = point2 - point1;
        let edge_vector2 = point3 - point1;
        let normal_vector = (edge_vector2.cross(edge_vector1)).normalize();

        Self {
            id: generate_object_id(),
            transform: Matrix::identity(4),
            material: Default::default(),
            casts_shadow: true,
            point1,
            point2,
            point3,
            normal_vector1,
            normal_vector2,
            normal_vector3,
            edge_vector1: edge_vector1,
            edge_vector2: edge_vector2,
            normal_vector: normal_vector,
            parent: None,
        }
    }

    pub fn get_id(&self) -> &i32 {
        &self.id
    }

    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let dir_cross_e2 = ray.direction.cross(self.edge_vector2);
        let determinant = self.edge_vector1.dot(dir_cross_e2);

        if determinant.abs() < EPSILON {
            return vec![];
        }

        let f = 1.0 / determinant;

        let p1_to_origin = ray.origin - self.point1;
        let u = f * p1_to_origin.dot(dir_cross_e2);
        if u < 0. || u > 1. {
            return vec![];
        }

        let origin_cross_e1 = p1_to_origin.cross(self.edge_vector1);
        let v = f * ray.direction.dot(origin_cross_e1);
        if v < 0. || (u + v) > 1. {
            return vec![];
        }

        let t = f * self.edge_vector2.dot(origin_cross_e1);

        vec![Intersection::intersection_with_uv(t, Shape::SmoothTriangle(self.clone()), u, v)]
    }

    pub fn normal_at(&self, _world_point: Tuple, hit: &Intersection) -> Tuple {
        let hit_u = hit.u.unwrap();
        let hit_v = hit.v.unwrap();

        self.normal_vector2 * hit_u + self.normal_vector3 *
        hit_v + self.normal_vector1 * (1. - hit_u - hit_v)
    }

    pub fn bounds_of(&self) -> Bound {
        Bound::bounding_box_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::intersections;
    use super::super::near_eq;
    use super::super::ORIGIN;
    use super::super::ray::Ray;
    use super::super::{CommonShape, Shape};
    use super::super::tuple::Tuple;

    fn make_test_smooth_triangle() -> SmoothTriangle {
        let point1 = Tuple::point(0., 1., 0.);
        let point2 = Tuple::point(-1., 0., 0.);
        let point3 = Tuple::point(1., 0., 0.);
        let normal1 = Tuple::vector(0., 1., 0.);
        let normal2 = Tuple::vector(-1., 0., 0.);
        let normal3 = Tuple::vector(1., 0., 0.);

        SmoothTriangle::new(point1, point2, point3, normal1, normal2, normal3)
    }

    #[test]
    fn constructing_smooth_triangle() {
        let expected_point1 = Tuple::point(0., 1., 0.);
        let expected_point2 = Tuple::point(-1., 0., 0.);
        let expected_point3 = Tuple::point(1., 0., 0.);
        let expected_normal1 = Tuple::vector(0., 1., 0.);
        let expected_normal2 = Tuple::vector(-1., 0., 0.);
        let expected_normal3 = Tuple::vector(1., 0., 0.);

        let actual = make_test_smooth_triangle();

        assert_eq!(expected_point1, actual.point1);
        assert_eq!(expected_point2, actual.point2);
        assert_eq!(expected_point3, actual.point3);
        assert_eq!(expected_normal1, actual.normal_vector1);
        assert_eq!(expected_normal2, actual.normal_vector2);
        assert_eq!(expected_normal3, actual.normal_vector3);
    }

    #[test]
    fn intersection_with_smooth_triangle_stores_uv() {
        let ray = Ray::new(Tuple::point(-0.2, 0.3, -2.), Tuple::vector(0., 0., 1.));
        let triangle = make_test_smooth_triangle();

        let expected_u = 0.45;
        let expected_v = 0.25;

        let actual = triangle.intersect(ray);

        match actual[0].u {
            Some(u) => assert!(near_eq(expected_u, u)),
            None => assert!(false),
        };
        match actual[0].v {
            Some(v) => assert!(near_eq(expected_v, v)),
            None => assert!(false),
        };
    }

    #[test]
    fn smooth_triangle_uses_uv_to_interpolate_normal() {
        let triangle = Shape::SmoothTriangle(make_test_smooth_triangle());
        let intersection = Intersection::intersection_with_uv(1., triangle.clone(), 0.45, 0.25);

        let expected = Tuple::vector(-0.5547, 0.83205, 0.);

        let actual = triangle.normal_at(ORIGIN, &intersection);

        assert_eq!(expected, actual);
    }

    #[test]
    fn preparing_normal_on_smooth_triangle() {
        let triangle = Shape::SmoothTriangle(make_test_smooth_triangle());
        let intersection = Intersection::intersection_with_uv(1., triangle, 0.45, 0.25);
        let ray = Ray::new(Tuple::point(-0.2, 0.3, -2.), Tuple::vector(0., 0., 1.));
        let intersections = intersections!(intersection.clone());
        let computations = intersection.prepare_computations(ray, &intersections);

        let expected = Tuple::vector(-0.5547, 0.83205, 0.);

        let actual = computations.normal_vector;

        assert_eq!(expected, actual);
    }
}
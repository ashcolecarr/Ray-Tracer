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
pub struct Triangle {
    id: i32,
    pub transform: Matrix,
    pub material: Material,
    pub casts_shadow: bool,
    pub point1: Tuple,
    pub point2: Tuple,
    pub point3: Tuple,
    pub edge_vector1: Tuple,
    pub edge_vector2: Tuple,
    pub normal_vector: Tuple,
    pub parent: Option<i32>,
}

impl PartialEq for Triangle {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.transform == other.transform &&
            self.material == other.material && self.casts_shadow == other.casts_shadow &&
            self.point1 == other.point1 && self.point2 == other.point2 &&
            self.point3 == other.point3 && self.edge_vector1 == other.edge_vector1 &&
            self.edge_vector2 == other.edge_vector2 && self.normal_vector == other.normal_vector &&
            self.parent == other.parent
    }
}

impl Triangle {
    pub fn new(point1: Tuple, point2: Tuple, point3: Tuple) -> Self {
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

        vec![Intersection::new(t, Shape::Triangle(self.clone()))]
    }

    pub fn normal_at(&self, _world_point: Tuple) -> Tuple {
        self.normal_vector
    }

    pub fn bounds_of(&self) -> Bound {
        Bound::bounding_box_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::ray::Ray;
    use super::super::tuple::Tuple;

    #[test]
    fn constructing_triangle() {
        let point1 = Tuple::point(0., 1., 0.);
        let point2 = Tuple::point(-1., 0., 0.);
        let point3 = Tuple::point(1., 0., 0.);

        let expected_point1 = point1;
        let expected_point2 = point2;
        let expected_point3 = point3;
        let expected_edge_vector1 = Tuple::vector(-1., -1., 0.);
        let expected_edge_vector2 = Tuple::vector(1., -1., 0.);
        let expected_normal_vector = Tuple::vector(0., 0., -1.);

        let actual = Triangle::new(point1, point2, point3);

        assert_eq!(expected_point1, actual.point1);
        assert_eq!(expected_point2, actual.point2);
        assert_eq!(expected_point3, actual.point3);
        assert_eq!(expected_edge_vector1, actual.edge_vector1);
        assert_eq!(expected_edge_vector2, actual.edge_vector2);
        assert_eq!(expected_normal_vector, actual.normal_vector);
    }

    #[test]
    fn finding_normal_on_triangle() {
        let triangle = Triangle::new(Tuple::point(0., 1., 0.), Tuple::point(-1., 0., 0.), Tuple::point(1., 0., 0.));

        let expected = triangle.normal_vector;

        let actual1 = triangle.normal_at(Tuple::point(0., 0.5, 0.));
        let actual2 = triangle.normal_at(Tuple::point(-0.5, 0.75, 0.));
        let actual3 = triangle.normal_at(Tuple::point(0.5, 0.25, 0.));

        assert_eq!(expected, actual1);
        assert_eq!(expected, actual2);
        assert_eq!(expected, actual3);
    }

    #[test]
    fn intersecting_ray_parallel_to_triangle() {
        let triangle = Triangle::new(Tuple::point(0., 1., 0.), Tuple::point(-1., 0., 0.), Tuple::point(1., 0., 0.));
        let ray = Ray::new(Tuple::point(0., -1., -2.), Tuple::vector(0., 1., 0.));

        let actual = triangle.intersect(ray);

        assert!(actual.is_empty());
    }

    #[test]
    fn ray_misses_p1_to_p2_edge() {
        let triangle = Triangle::new(Tuple::point(0., 1., 0.), Tuple::point(-1., 0., 0.), Tuple::point(1., 0., 0.));
        let ray = Ray::new(Tuple::point(-1., 1., -2.), Tuple::vector(0., 0., 1.));

        let actual = triangle.intersect(ray);

        assert!(actual.is_empty());
    }

    #[test]
    fn ray_misses_p2_to_p3_edge() {
        let triangle = Triangle::new(Tuple::point(0., 1., 0.), Tuple::point(-1., 0., 0.), Tuple::point(1., 0., 0.));
        let ray = Ray::new(Tuple::point(0., -1., -2.), Tuple::vector(0., 0., 1.));

        let actual = triangle.intersect(ray);

        assert!(actual.is_empty());
    }

    #[test]
    fn ray_misses_p1_to_p3_edge() {
        let triangle = Triangle::new(Tuple::point(0., 1., 0.), Tuple::point(-1., 0., 0.), Tuple::point(1., 0., 0.));
        let ray = Ray::new(Tuple::point(1., 1., -2.), Tuple::vector(0., 0., 1.));

        let actual = triangle.intersect(ray);

        assert!(actual.is_empty());
    }

    #[test]
    fn ray_strikes_triangle() {
        let triangle = Triangle::new(Tuple::point(0., 1., 0.), Tuple::point(-1., 0., 0.), Tuple::point(1., 0., 0.));
        let ray = Ray::new(Tuple::point(0., 0.5, -2.), Tuple::vector(0., 0., 1.));

        let expected_count = 1;
        let expected_t = 2.;

        let actual = triangle.intersect(ray);

        assert_eq!(expected_count, actual.len());
        assert_eq!(expected_t, actual[0].t);
    }
}
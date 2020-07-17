use super::bound::Bound;
use super::EPSILON;
use super::generate_object_id;
use super::intersection::Intersection;
use super::material::Material;
use super::matrix::Matrix;
use super::near_eq;
use super::ray::Ray;
use super::shape::Shape;
use super::tuple::Tuple;
use std::f64::INFINITY;
use std::mem::swap;

#[derive(Debug, Clone)]
pub struct Cone {
    id: i32,
    pub transform: Matrix,
    pub material: Material,
    pub casts_shadow: bool,
    pub minimum: f64,
    pub maximum: f64,
    pub closed: bool,
    pub parent: Option<i32>,
}

impl PartialEq for Cone {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.transform == other.transform &&
            self.material == other.material && self.casts_shadow == other.casts_shadow &&
            near_eq(self.minimum, other.minimum) && near_eq(self.maximum, other.maximum) &&
            self.closed == other.closed && self.parent == other.parent
    }
}

impl Cone {
    pub fn new() -> Self {
        Self {
            id: generate_object_id(),
            transform: Matrix::identity(4),
            material: Default::default(),
            casts_shadow: true,
            minimum: -INFINITY,
            maximum: INFINITY,
            closed: false,
            parent: None,
        }
    }

    pub fn get_id(&self) -> &i32 {
        &self.id
    }

    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let mut intersections: Vec<Intersection> = vec![];

        let a = ray.direction.x.powi(2) - ray.direction.y.powi(2) + ray.direction.z.powi(2);
        let b = 2. * ray.origin.x * ray.direction.x - 2. * ray.origin.y * ray.direction.y + 2. * ray.origin.z * ray.direction.z;
        let c = ray.origin.x.powi(2) - ray.origin.y.powi(2) + ray.origin.z.powi(2);

        if near_eq(a, 0.) && !near_eq(b, 0.) {
            let t = -c / (2. * b);

            intersections.push(Intersection::new(t, Shape::Cone(self.clone())));
        } 
        
        if !near_eq(a, 0.) { 
            let disc = b.powi(2) - 4. * a * c;

            if disc > 0. || near_eq(disc, 0.) {
                let mut t0 = (-b - disc.sqrt()) / (2. * a);
                let mut t1 = (-b + disc.sqrt()) / (2. * a);

                if t0 > t1 {
                    swap(&mut t0, &mut t1);
                }
        
                let y0 = ray.origin.y + t0 * ray.direction.y;
                if self.minimum < y0 && y0 < self.maximum {
                    intersections.push(Intersection::new(t0, Shape::Cone(self.clone())));
                }
        
                let y1 = ray.origin.y + t1 * ray.direction.y;
                if self.minimum < y1 && y1 < self.maximum {
                    intersections.push(Intersection::new(t1, Shape::Cone(self.clone())));
                }
            }
        }

        self.intersect_caps(ray, &mut intersections);

        intersections
    }

    fn intersect_caps(&self, ray: Ray, intersections: &mut Vec<Intersection>) {
        if !self.closed || near_eq(ray.direction.y, 0.) {
            return;
        }

        let t_lower = (self.minimum - ray.origin.y) / ray.direction.y;
        if Cone::check_cap(ray, t_lower, self.minimum) {
            intersections.push(Intersection::new(t_lower, Shape::Cone(self.clone())));
        }

        let t_upper = (self.maximum - ray.origin.y) / ray.direction.y;
        if Cone::check_cap(ray, t_upper, self.maximum) {
            intersections.push(Intersection::new(t_upper, Shape::Cone(self.clone())));
        }
    } 
    
    fn check_cap(ray: Ray, t: f64, y: f64) -> bool {
        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;
        
        let result = x.powi(2) + z.powi(2); 
        
        result < y.powi(2) || near_eq(result, y.powi(2))
    }

    pub fn normal_at(&self, world_point: Tuple, _hit: &Intersection) -> Tuple {
        let distance = world_point.x.powi(2) + world_point.z.powi(2);

        if distance < self.maximum.powi(2) && (world_point.y > self.maximum - EPSILON ||
            near_eq(world_point.y, self.maximum - EPSILON)) {

            Tuple::vector(0., 1., 0.)
        } else if distance < self.minimum.powi(2) && (world_point.y < self.minimum + EPSILON ||
            near_eq(world_point.y, self.minimum + EPSILON)) {

            Tuple::vector(0., -1., 0.)
        } else {
            let y = distance.sqrt();

            Tuple::vector(world_point.x, if world_point.y > 0. { -y } else { y }, world_point.z)
        }
    }

    pub fn bounds_of(&self) -> Bound {
        let a = self.minimum.abs();
        let b = self.maximum.abs();
        let limit = f64::max(a, b);
        
        Bound::bounding_box_init(Tuple::point(-limit, self.minimum, -limit),
            Tuple::point(limit, self.maximum, limit))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::near_eq;
    use super::super::ORIGIN;
    use super::super::ray::Ray;
    use super::super::tuple::Tuple;

    #[test]
    fn intersecting_cone_with_ray() {
        let cone = Cone::new();
        let rays: Vec<Ray> = vec![
            Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.).normalize()),
            Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(1., 1., 1.).normalize()),
            Ray::new(Tuple::point(1., 1., -5.), Tuple::vector(-0.5, -1., 1.).normalize()),
        ];

        let expected_count = 2;
        let expecteds: Vec<(f64, f64)> = vec![(5., 5.), (8.66025, 8.66025), (4.55006, 49.44994)];

        for source in expecteds.iter().zip(rays) {
            let (expected, ray) = source;
            let actual = cone.intersect(ray);

            assert_eq!(expected_count, actual.len());
            assert!(near_eq(expected.0, actual[0].t));
            assert!(near_eq(expected.1, actual[1].t));
        }
    }

    #[test]
    fn intersecting_cone_with_ray_parallel_to_one_of_its_halves() {
        let cone = Cone::new();
        let ray = Ray::new(Tuple::point(0., 0., -1.), Tuple::vector(0., 1., 1.).normalize());
        
        let expected_count = 1;
        let expected = 0.35355;

        let actual = cone.intersect(ray);

        assert_eq!(expected_count, actual.len());
        assert!(near_eq(expected, actual[0].t));
    }

    #[test]
    fn intersecting_cone_end_caps() {
        let mut cone = Cone::new();
        cone.minimum = -0.5;
        cone.maximum = 0.5;
        cone.closed = true;
        let rays: Vec<Ray> = vec![
            Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 1., 0.).normalize()),
            Ray::new(Tuple::point(0., 0., -0.25), Tuple::vector(0., 1., 1.).normalize()),
            Ray::new(Tuple::point(0., 0., -0.25), Tuple::vector(0., 1., 0.).normalize()),
        ];

        let expected_counts = vec![0, 2, 4]; 

        for source in expected_counts.iter().zip(rays) {
            let (expected, ray) = source;
            let actual = cone.intersect(ray);

            assert_eq!(*expected, actual.len());
        }
    }

    #[test]
    fn computing_normal_vector_on_cone() {
        let cone = Cone::new();
        let intersection = Intersection::new(1., Shape::Cone(cone.clone()));

        let points = vec![
            ORIGIN,
            Tuple::point(1., 1., 1.),
            Tuple::point(-1., -1., 0.), 
        ];

        let expecteds: Vec<Tuple> = vec![
            Tuple::vector(0., 0., 0.),
            Tuple::vector(1., -2_f64.sqrt(), 1.),
            Tuple::vector(-1., 1., 0.), 
        ];

        for source in expecteds.iter().zip(points) {
            let (expected, point) = source;
            let actual = cone.normal_at(point, &intersection);
            
            assert_eq!(*expected, actual);
        }
    }

    #[test]
    fn unbounded_cone_has_bounding_box() {
        let shape = Cone::new();
        let bounding_box = shape.bounds_of();

        let expected_minimum = Tuple::point(-INFINITY, -INFINITY, -INFINITY);
        let expected_maximum = Tuple::point(INFINITY, INFINITY, INFINITY);

        let actual_minimum = bounding_box.minimum;
        let actual_maximum = bounding_box.maximum;

        assert_eq!(expected_minimum.x, actual_minimum.x);
        assert_eq!(expected_minimum.y, actual_minimum.y);
        assert_eq!(expected_minimum.z, actual_minimum.z);
        assert_eq!(expected_maximum.x, actual_maximum.x);
        assert_eq!(expected_maximum.y, actual_maximum.y);
        assert_eq!(expected_maximum.z, actual_maximum.z);
    }

    #[test]
    fn bounded_cylinder_has_bounding_box() {
        let mut shape = Cone::new();
        shape.minimum = -5.;
        shape.maximum = 3.;
        let bounding_box = shape.bounds_of();

        let expected_minimum = Tuple::point(-5., -5., -5.);
        let expected_maximum = Tuple::point(5., 3., 5.);

        let actual_minimum = bounding_box.minimum;
        let actual_maximum = bounding_box.maximum;

        assert_eq!(expected_minimum, actual_minimum);
        assert_eq!(expected_maximum, actual_maximum);
    }
}
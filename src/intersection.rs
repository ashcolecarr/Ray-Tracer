use super::computations::Computations;
use super::EPSILON;
use super::near_eq;
use super::ray::Ray;
use super::shape::{Shape, Actionable};

#[derive(Debug, Clone)]
pub struct Intersection {
    pub t: f64,
    pub object: Shape,
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        near_eq(self.t, other.t) && self.object == other.object
    }
}

impl Intersection {
    pub fn new(t: f64, object: Shape) -> Self {
        Self { t, object }
    }

    pub fn hit(intersections: Vec<Self>) -> Option<Self> {
        // This is assuming that the list of intersections is already sorted.
        match intersections.into_iter().filter(|i| i.t > 0.).next() {
            Some(i) => Some(i),
            None => None
        }
    }

    pub fn prepare_computations(&self, ray: Ray, intersections: Vec<Intersection>) -> Computations {
        let point = ray.position(self.t);
        let mut normal_vector = self.object.normal_at(point);
        let eye_vector = -ray.direction;

        let inside = if normal_vector.dot(eye_vector) < 0. {
            normal_vector = -normal_vector;
            true
        } else {
            false
        };

        let over_point = point + normal_vector * EPSILON;
        let under_point = point - normal_vector * EPSILON;
        let reflect_vector = ray.direction.reflect(normal_vector);

        let mut n1 = 0.;
        let mut n2 = 0.;
        let mut containers: Vec<Shape> = Vec::new();
        for intersection in intersections {
            if intersection == *self {
                n1 = if containers.is_empty() {
                    1.
                } else {
                    containers.last().unwrap().get_material().refractive_index
                };
            }

            if containers.contains(&intersection.object) {
                let index = &containers.iter().position(|con| *con == intersection.object).unwrap();
                containers.remove(*index);
            } else {
                containers.push(intersection.clone().object);
            }

            if intersection == *self {
                n2 = if containers.is_empty() {
                    1.
                } else {
                    containers.last().unwrap().get_material().refractive_index
                }
            }
        }

        Computations {
            t: self.t,
            object: self.object.clone(),
            point,
            eye_vector,
            normal_vector,
            inside, 
            over_point,
            reflect_vector,
            n1,
            n2,
            under_point,
        }
    }

    pub fn schlick(computations: Computations) -> f64 {
        let mut cos = computations.eye_vector.dot(computations.normal_vector);

        if computations.n1 > computations.n2 {
            let n = computations.n1 / computations.n2;
            let sin2_t = n.powi(2) * (1. - cos.powi(2));

            if sin2_t > 1. {
                return 1.;
            }

            let cos_t = (1. - sin2_t).sqrt();
            cos = cos_t;
        }

        let r0 = ((computations.n1 - computations.n2) / (computations.n1 + computations.n2)).powi(2);

        r0 + (1. - r0) * (1. - cos).powi(5)
    }
}

#[macro_export]
macro_rules! intersections {
    ( $( $x:expr ),* ) => {
        {
            let mut intersections = Vec::new();
            $(
                intersections.push($x);
            )*
            intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
            intersections
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::computations::Computations;
    use super::super::EPSILON;
    use super::super::material::Material;
    use super::super::near_eq;
    use super::super::ORIGIN;
    use super::super::plane::Plane;
    use super::super::ray::Ray;
    use super::super::shape::Shape;
    use super::super::sphere::Sphere;
    use super::super::transformation::*;
    use super::super::tuple::Tuple;

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let sphere = Shape::Sphere(Sphere::new());

        let expected_t = 3.5;
        let expected_object = sphere.clone();

        let actual = Intersection::new(3.5, sphere);

        assert_eq!(expected_t, actual.t);
        assert_eq!(expected_object, actual.object);
    }

    #[test]
    fn aggregating_intersections() {
        let sphere = Shape::Sphere(Sphere::new());
        let intersection1 = Intersection::new(1., sphere.clone());
        let intersection2 = Intersection::new(2., sphere.clone());

        let expected_count = 2;
        let expected_t1 = 1.;
        let expected_t2 = 2.;

        let actual = intersections!(intersection1, intersection2);

        assert_eq!(expected_count, actual.len());
        assert!(near_eq(expected_t1, actual[0].t));
        assert!(near_eq(expected_t2, actual[1].t));
    }

    #[test]
    fn hit_when_all_intersections_have_positive_t() {
        let sphere = Shape::Sphere(Sphere::new());
        let intersection1 = Intersection::new(1., sphere.clone());
        let intersection2 = Intersection::new(2., sphere.clone());
        let intersections = intersections!(intersection2, intersection1.clone());
        
        let expected = intersection1;

        let actual = Intersection::hit(intersections).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn hit_when_some_intersections_have_negative_t() {
        let sphere = Shape::Sphere(Sphere::new());
        let intersection1 = Intersection::new(-1., sphere.clone());
        let intersection2 = Intersection::new(1., sphere.clone());
        let intersections = intersections!(intersection2.clone(), intersection1);
        
        let expected = intersection2.clone();

        let actual = Intersection::hit(intersections).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn hit_when_all_intersections_have_negative_t() {
        let sphere = Shape::Sphere(Sphere::new());
        let intersection1 = Intersection::new(-2., sphere.clone());
        let intersection2 = Intersection::new(-1., sphere.clone());
        let intersections = intersections!(intersection2, intersection1);
        
        let actual = Intersection::hit(intersections);

        assert!(actual.is_none());
    }
    
    #[test]
    fn hit_is_always_lowest_nonnegative_intersection() {
        let sphere = Shape::Sphere(Sphere::new());
        let intersection1 = Intersection::new(5., sphere.clone());
        let intersection2 = Intersection::new(7., sphere.clone());
        let intersection3 = Intersection::new(-3., sphere.clone());
        let intersection4 = Intersection::new(2., sphere.clone());
        let intersections = intersections!(intersection1, intersection2, intersection3, intersection4.clone());
        
        let expected = intersection4.clone();

        let actual = Intersection::hit(intersections).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn precomputing_state_of_intersection() {
        let ray = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let shape = Shape::Sphere(Sphere::new());
        let intersection = Intersection { t: 4., object: shape.clone() };

        let expected = Computations {
            t: 4.,
            object: shape,
            point: Tuple::point(0., 0., -1.),
            eye_vector: Tuple::vector(0., 0., -1.),
            normal_vector: Tuple::vector(0., 0., -1.),
            inside: false,
            over_point: Tuple::point(0., 0., -1.) + Tuple::vector(0., 0., -1.) * EPSILON,
            reflect_vector: ray.direction.reflect(Tuple::vector(0., 0., -1.)),
            n1: 1.,
            n2: 1.,
            under_point: Tuple::point(0., 0., -1.) - Tuple::vector(0., 0., -1.) * EPSILON,
        };

        let actual = intersection.prepare_computations(ray, vec![intersection.clone()]);

        assert_eq!(expected, actual);
    }

    #[test]
    fn hit_when_intersection_occurs_on_outside() {
        let ray = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let shape = Shape::Sphere(Sphere::new());
        let intersection = Intersection { t: 4., object: shape.clone() };

        let actual = intersection.prepare_computations(ray, vec![intersection.clone()]);

        assert!(!actual.inside);
    }

    #[test]
    fn hit_when_intersection_occurs_on_inside() {
        let ray = Ray::new(ORIGIN, Tuple::vector(0., 0., 1.));
        let shape = Shape::Sphere(Sphere::new());
        let intersection = Intersection { t: 1., object: shape.clone() };

        let expected = Computations {
            t: 1.,
            object: shape,
            point: Tuple::point(0., 0., 1.),
            eye_vector: Tuple::vector(0., 0., -1.),
            normal_vector: Tuple::vector(0., 0., -1.),
            inside: true,
            over_point: Tuple::point(0., 0., 1.) + (Tuple::vector(0., 0., -1.) * EPSILON),
            reflect_vector: ray.direction.reflect(Tuple::vector(0., 0., -1.)),
            n1: 1.,
            n2: 1.,
            under_point: Tuple::point(0., 0., 1.) - (Tuple::vector(0., 0., -1.) * EPSILON),
        };

        let actual = intersection.prepare_computations(ray, vec![intersection.clone()]);

        assert!(actual.inside);
        assert_eq!(expected, actual);
    }

    #[test]
    fn hit_shold_offset_point() {
        let ray = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let mut shape = Shape::Sphere(Sphere::new());
        shape.set_transform(translate(0., 0., 1.));
        let intersection = Intersection::new(5., shape);
        let actual = intersection.prepare_computations(ray, vec![intersection.clone()]);

        assert!(actual.over_point.z < -EPSILON / 2.);
        assert!(actual.point.z > actual.over_point.z);
    }

    #[test]
    fn precomputing_reflection_vector() {
        let shape = Shape::Plane(Plane::new());
        let ray = Ray::new(Tuple::point(0., 1., -1.), Tuple::vector(0., -2_f64.sqrt() / 2., 2_f64.sqrt() / 2.));
        let intersection = Intersection::new(2_f64.sqrt(), shape);
        let computations = intersection.prepare_computations(ray, vec![intersection.clone()]);

        let expected = Tuple::vector(0., 2_f64.sqrt() / 2., 2_f64.sqrt() / 2.);

        let actual = computations.reflect_vector;

        assert_eq!(expected, actual);
    }

    #[test]
    fn finding_n1_and_n2_at_various_intersections() {
        let mut sphere1 = Shape::Sphere(Sphere::glass_sphere());
        sphere1.set_transform(scale(2., 2., 2.));

        let mut sphere2_material: Material = Default::default();
        sphere2_material.refractive_index = 2.;
        let mut sphere2 = Shape::Sphere(Sphere::glass_sphere());
        sphere2.set_transform(translate(0., 0., -0.25));
        sphere2.set_material(sphere2_material);

        let mut sphere3_material: Material = Default::default();
        sphere3_material.refractive_index = 2.5;
        let mut sphere3 = Shape::Sphere(Sphere::glass_sphere());
        sphere3.set_transform(scale(2., 2., 2.));
        sphere3.set_material(sphere3_material);

        let ray = Ray::new(Tuple::point(0., 0., -4.), Tuple::vector(0., 0., -1.));
        let intersections = intersections!(Intersection::new(2., sphere1.clone()), 
            Intersection::new(2.75, sphere2.clone()), Intersection::new(3.25, sphere3.clone()), 
            Intersection::new(4.75, sphere2), Intersection::new(5.25, sphere3), 
            Intersection::new(6., sphere1));
        
        let expected: Vec<(f64, f64)> = vec![(1., 1.5), (1.5, 2.), (2., 2.5),
            (2.5, 2.5), (2.5, 1.5), (1.5, 1.)];
        
        for index in 0..6 {
            let actual = intersections[index].prepare_computations(ray, intersections.clone());
            assert!(near_eq(expected[index].0, actual.n1));
            assert!(near_eq(expected[index].1, actual.n2));
        }
    }

    #[test]
    fn under_point_is_offset_below_surface() {
        let ray = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let mut shape = Shape::Sphere(Sphere::glass_sphere());
        shape.set_transform(translate(0., 0., 1.));
        let intersection = Intersection::new(5., shape);
        let intersections = intersections!(intersection.clone());
        let computations = intersection.prepare_computations(ray, intersections);

        assert!(computations.under_point.z > EPSILON / 2.);
        assert!(computations.point.z < computations.under_point.z);
    }

    #[test]
    fn schlick_approximation_under_total_internal_reflection() {
        let shape = Shape::Sphere(Sphere::glass_sphere());
        let ray = Ray::new(Tuple::point(0., 0., 2_f64.sqrt()), Tuple::vector(0., 1., 0.));
        let intersections = intersections!(Intersection::new(-2_f64.sqrt() / 2., shape.clone()),
            Intersection::new(2_f64.sqrt() / 2., shape));   
        let computations = intersections[1].prepare_computations(ray, intersections.clone());

        let expected = 1.;

        let actual = Intersection::schlick(computations);

        assert!(near_eq(expected, actual));        
    }

    #[test]
    fn schlick_approximation_with_perpendicular_viewing_angle() {
        let shape = Shape::Sphere(Sphere::glass_sphere());
        let ray = Ray::new(ORIGIN, Tuple::vector(0., 1., 0.));
        let intersections = intersections!(Intersection::new(-1., shape.clone()),
            Intersection::new(1., shape));   
        let computations = intersections[1].prepare_computations(ray, intersections.clone());

        let expected = 0.04;

        let actual = Intersection::schlick(computations);

        assert!(near_eq(expected, actual)); 
    }

    #[test]
    fn schlick_approximation_with_small_angle_and_n2_greater_than_n1() {
        let shape = Shape::Sphere(Sphere::glass_sphere());
        let ray = Ray::new(Tuple::point(0., 0.99, -2.), Tuple::vector(0., 0., 1.));
        let intersections = intersections!(Intersection::new(1.8589, shape));
        let computations = intersections[0].prepare_computations(ray, intersections.clone());

        let expected = 0.48873;

        let actual = Intersection::schlick(computations);

        assert!(near_eq(expected, actual)); 
    }
}
use super::computations::Computations;
use super::near_eq;
use super::ray::Ray;
use super::sphere::Sphere;

#[derive(Debug, Clone)]
pub struct Intersection {
    pub t: f64,
    pub object: Sphere,
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        near_eq(self.t, other.t) && self.object == other.object
    }
}

impl Intersection {
    pub fn new(t: f64, object: Sphere) -> Self {
        Self { t, object }
    }

    pub fn hit(intersections: Vec<Self>) -> Option<Self> {
        // This is assuming that the list of intersections is already sorted.
        match intersections.into_iter().filter(|i| i.t > 0.).next() {
            Some(i) => Some(i),
            None => None
        }
    }

    pub fn prepare_computations(&self, ray: Ray) -> Computations {
        let point = ray.position(self.t);
        let mut normal_vector = self.object.normal_at(point);
        let eye_vector = -ray.direction;

        let inside = if normal_vector.dot(eye_vector) < 0. {
            normal_vector = -normal_vector;
            true
        } else {
            false
        };

        Computations {
            t: self.t,
            object: self.object.clone(),
            point,
            eye_vector,
            normal_vector,
            inside, 
        }
    }
}

#[macro_export]
macro_rules! intersections {
    ( $( $x:expr ), * ) => {
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
    use super::super::near_eq;
    use super::super::ORIGIN;
    use super::super::ray::Ray;
    use super::super::sphere::Sphere;
    use super::super::tuple::Tuple;

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let sphere = Sphere::new();

        let expected_t = 3.5;
        let expected_object = sphere.clone();

        let actual = Intersection::new(3.5, sphere);

        assert_eq!(expected_t, actual.t);
        assert_eq!(expected_object, actual.object);
    }

    #[test]
    fn aggregating_intersections() {
        let sphere = Sphere::new();
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
        let sphere = Sphere::new();
        let intersection1 = Intersection::new(1., sphere.clone());
        let intersection2 = Intersection::new(2., sphere.clone());
        let intersections = intersections!(intersection2, intersection1.clone());
        
        let expected = intersection1;

        let actual = Intersection::hit(intersections).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn hit_when_some_intersections_have_negative_t() {
        let sphere = Sphere::new();
        let intersection1 = Intersection::new(-1., sphere.clone());
        let intersection2 = Intersection::new(1., sphere.clone());
        let intersections = intersections!(intersection2.clone(), intersection1);
        
        let expected = intersection2.clone();

        let actual = Intersection::hit(intersections).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn hit_when_all_intersections_have_negative_t() {
        let sphere = Sphere::new();
        let intersection1 = Intersection::new(-2., sphere.clone());
        let intersection2 = Intersection::new(-1., sphere.clone());
        let intersections = intersections!(intersection2, intersection1);
        
        let actual = Intersection::hit(intersections);

        assert!(actual.is_none());
    }
    
    #[test]
    fn hit_is_always_lowest_nonnegative_intersection() {
        let sphere = Sphere::new();
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
        let shape = Sphere::new();
        let intersection = Intersection { t: 4., object: shape.clone() };

        let expected = Computations {
            t: 4.,
            object: shape,
            point: Tuple::point(0., 0., -1.),
            eye_vector: Tuple::vector(0., 0., -1.),
            normal_vector: Tuple::vector(0., 0., -1.),
            inside: false,
        };

        let actual = intersection.prepare_computations(ray);

        assert_eq!(expected, actual);
    }

    #[test]
    fn hit_when_intersection_occurs_on_outside() {
        let ray = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let shape = Sphere::new();
        let intersection = Intersection { t: 4., object: shape.clone() };

        let actual = intersection.prepare_computations(ray);

        assert!(!actual.inside);
    }

    #[test]
    fn hit_when_intersection_occurs_on_inside() {
        let ray = Ray::new(ORIGIN, Tuple::vector(0., 0., 1.));
        let shape = Sphere::new();
        let intersection = Intersection { t: 1., object: shape.clone() };

        let expected = Computations {
            t: 1.,
            object: shape,
            point: Tuple::point(0., 0., 1.),
            eye_vector: Tuple::vector(0., 0., -1.),
            normal_vector: Tuple::vector(0., 0., -1.),
            inside: true,
        };

        let actual = intersection.prepare_computations(ray);

        assert!(actual.inside);
        assert_eq!(expected, actual);
    }
}
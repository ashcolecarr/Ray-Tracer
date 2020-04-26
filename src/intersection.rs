use super::intersections;
use super::sphere::Sphere;

#[derive(Debug, Clone)]
pub struct Intersection {
    pub t: f64,
    pub object: Sphere,
}

impl Intersection {
    pub fn new(t: f64, object: Sphere) -> Self {
        Self { t, object }
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
    use super::super::near_eq;
    use super::super::sphere::Sphere;

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
}
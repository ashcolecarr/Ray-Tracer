use super::intersection::Intersection;
use super::matrix::Matrix;
use super::ray::Ray;
use super::shape::{Shape, Actionable};
use std::sync::atomic::{AtomicI32, Ordering};

#[derive(Debug, Clone)]
pub struct Group {
    id: i32,
    pub transform: Matrix,
    pub shapes: Vec<Shape>,
    pub parent: Box<Option<Shape>>,
}

impl PartialEq for Group {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.transform == other.transform 
    }
}

impl Group {
    pub fn new() -> Self {
        static ID_COUNT: AtomicI32 = AtomicI32::new(1);

        Self {
            id: ID_COUNT.fetch_add(1, Ordering::Relaxed),
            transform: Matrix::identity(4),
            shapes: vec![],
            parent: Box::new(None),
        }
    }

    pub fn add_child(&mut self, shape: &mut Shape) {
        shape.set_parent(Shape::Group(self.clone()));
        self.shapes.push(shape.clone());
    }

    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let mut intersections: Vec<Intersection> = self.shapes.iter().fold(Vec::new(), |mut ints, o| {
            ints.append(&mut o.intersect(ray));
            ints
        }); 

        intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

        intersections
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::matrix::Matrix;
    use super::super::ORIGIN;
    use super::super::ray::Ray;
    use super::super::shape::*;
    use super::super::sphere::Sphere;
    use super::super::transformation::*;
    use super::super::tuple::Tuple;

    #[test]
    fn creating_new_group() {
        let expected = Matrix::identity(4);

        let actual = Group::new();

        assert_eq!(expected, actual.transform);
        assert!(actual.shapes.is_empty());
    }

    #[test]
    fn adding_child_to_group() {
        let mut shape = Shape::TestShape(TestShape::new());

        let mut actual = Group::new();
        actual.add_child(&mut shape);

        let expected = shape.clone();
        let expected_shape = if let Shape::Group(group) = shape.get_parent().unwrap() { group } else { panic!("") };

        assert!(!actual.shapes.is_empty());
        assert_eq!(expected, actual.shapes[0]);
        assert_eq!(expected_shape, actual);
    }

    #[test]
    fn intersecting_ray_with_empty_group() {
        let group = Group::new();
        let ray = Ray::new(ORIGIN, Tuple::vector(0., 0., 1.));

        let actual = group.intersect(ray);

        assert!(actual.is_empty());
    }

    #[test]
    fn intersecting_ray_with_nonempty_group() {
        let mut group = Group::new();
        let sphere1 = Shape::Sphere(Sphere::new());
        let mut sphere2 = Shape::Sphere(Sphere::new());
        sphere2.set_transform(translate(0., 0., -3.));
        let mut sphere3 = Shape::Sphere(Sphere::new());
        sphere3.set_transform(translate(5., 0., 0.));
        group.shapes.push(sphere1.clone());
        group.shapes.push(sphere2.clone());
        group.shapes.push(sphere3.clone());
        let ray = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));

        let expected_count = 4;
        let expected_object1 = sphere1;
        let expected_object2 = sphere2;

        let actual = group.intersect(ray);

        assert_eq!(expected_count, actual.len());
        assert_eq!(expected_object2, actual[0].object);
        assert_eq!(expected_object2, actual[1].object);
        assert_eq!(expected_object1, actual[2].object);
        assert_eq!(expected_object1, actual[3].object);
    }

    #[test]
    fn intersecting_transformed_group() {
        let mut group = Shape::Group(Group::new());
        group.set_transform(scale(2., 2., 2.));
        let mut sphere = Shape::Sphere(Sphere::new());
        sphere.set_transform(translate(5., 0., 0.));
        group.add_child(&mut sphere);
        let ray = Ray::new(Tuple::point(10., 0., -10.), Tuple::vector(0., 0., 1.));

        let expected_count = 2;

        let actual = group.intersect(ray);

        assert_eq!(expected_count, actual.len());
    }
}
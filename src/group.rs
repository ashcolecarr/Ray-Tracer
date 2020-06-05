use super::generate_object_id;
use super::intersection::Intersection;
use super::material::Material;
use super::matrix::Matrix;
use super::PARENT_REFERENCES;
use super::ray::Ray;
use super::shape::{Shape, CommonShape};
use super::tuple::Tuple;

#[derive(Debug, Clone)]
pub struct Group {
    id: i32,
    pub transform: Matrix,
    pub material: Material,
    pub casts_shadow: bool,
    pub shapes: Vec<Shape>,
    pub parent: Option<i32>,
}

impl PartialEq for Group {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.transform == other.transform && 
            self.material == other.material && self.casts_shadow == other.casts_shadow &&
            self.parent == other.parent
    }
}

impl Group {
    pub fn new() -> Self {
        Self {
            id: generate_object_id(),
            transform: Matrix::identity(4),
            material: Default::default(),
            casts_shadow: true,
            shapes: vec![],
            parent: None,
        }
    }

    pub fn get_id(&self) -> &i32 {
        &self.id
    }

    pub fn add_child(&mut self, shape: &mut Shape) {
        shape.set_parent(self.id);
        self.shapes.push(shape.clone());

        Group::update_group_reference(self.clone());
    }

    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let mut intersections: Vec<Intersection> = self.shapes.iter().fold(Vec::new(), |mut ints, o| {
            ints.append(&mut o.intersect(ray));
            ints
        }); 

        intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

        intersections
    }

    pub fn normal_at(&self, _world_point: Tuple) -> Tuple {
        panic!("Normal at cannot be calculated on a group.")
    }

    pub fn update_group_reference(group: Self) {
        let read_reference = PARENT_REFERENCES.read().unwrap();
        let index = read_reference.iter().position(|pr| pr.get_id() == *group.get_id()); 
        drop(read_reference);

        let mut write_reference = PARENT_REFERENCES.write().unwrap();
        match index {
            Some(i) => {
                write_reference.remove(i);
                write_reference.push(Shape::Group(group));
            },
            None => write_reference.push(Shape::Group(group)),
        };
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

        let expected_shape = shape.clone();
        let parent_references = PARENT_REFERENCES.read().unwrap();
        let shape = parent_references.iter().find(|pr| pr.get_id() == shape.get_parent().unwrap()).unwrap();
        let expected_parent = if let Shape::Group(group) = shape { group } else { panic!("") };

        assert!(!actual.shapes.is_empty());
        assert_eq!(expected_shape, actual.shapes[0]);
        assert_eq!(*expected_parent, actual);
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
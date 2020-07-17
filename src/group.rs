use super::bound::Bound;
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
        if self.bounds_of().intersects(ray) {
            let mut intersections: Vec<Intersection> = self.shapes.iter().fold(Vec::new(), |mut ints, o| {
                ints.append(&mut o.intersect(ray));
                ints
            }); 

            intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
    
            return intersections;
        }

        vec![]
    }

    pub fn normal_at(&self, _world_point: Tuple, _hit: &Intersection) -> Tuple {
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

    pub fn bounds_of(&self) -> Bound {
        let mut group_box = Bound::bounding_box_empty();

        for shape in &self.shapes {
            let shape_box = shape.parent_space_bounds_of();
            group_box.add_box(shape_box);
        }

        group_box
    }
    
    pub fn partition_children(&mut self) -> (Vec<Shape>, Vec<Shape>) {
        let (left_box, right_box) = self.bounds_of().split_bounds();
        let mut left: Vec<Shape> = vec![];
        let mut right: Vec<Shape> = vec![];

        self.shapes.retain(|shape| {
            let remove = {
                if left_box.box_contains_box(shape.parent_space_bounds_of()) {
                    left.push(shape.clone());

                    true
                } else if right_box.box_contains_box(shape.parent_space_bounds_of()) {
                    right.push(shape.clone());

                    true
                } else {
                    false
                }
            };

            !remove
        });

        Group::update_group_reference(self.clone());

        (left, right)
    }

    pub fn make_subgroup(&mut self, shapes: &mut Vec<Shape>) {
        let mut subgroup = Shape::Group(Group::new());
        for mut shape in shapes {
            subgroup.add_child(&mut shape);
        }

        self.add_child(&mut subgroup);
    }

    pub fn divide(&mut self, threshold: usize) {
        if threshold <= self.shapes.len() {
            let (mut left, mut right) = self.partition_children();
            if !left.is_empty() {
                self.make_subgroup(&mut left);
            }

            if !right.is_empty() {
                self.make_subgroup(&mut right);
            }
        }

        for child in &mut self.shapes {
            child.divide(threshold);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::cylinder::Cylinder;
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
        sphere2.set_transform(&translate(0., 0., -3.));
        let mut sphere3 = Shape::Sphere(Sphere::new());
        sphere3.set_transform(&translate(5., 0., 0.));
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
        group.set_transform(&scale(2., 2., 2.));
        let mut sphere = Shape::Sphere(Sphere::new());
        sphere.set_transform(&translate(5., 0., 0.));
        group.add_child(&mut sphere);
        let ray = Ray::new(Tuple::point(10., 0., -10.), Tuple::vector(0., 0., 1.));

        let expected_count = 2;

        let actual = group.intersect(ray);

        assert_eq!(expected_count, actual.len());
    }

    #[test]
    fn group_has_bounding_box_that_contains_its_children() {
        let mut sphere = Shape::Sphere(Sphere::new());
        sphere.set_transform(&(translate(2., 5., -3.) * scale(2., 2., 2.)));
        let mut cylinder = Shape::Cylinder(Cylinder::new());
        cylinder.set_minimum(-2.);
        cylinder.set_maximum(2.);
        cylinder.set_transform(&(translate(-4., -1., 4.) * scale(0.5, 1., 0.5)));
        let mut shape = Shape::Group(Group::new());
        shape.add_child(&mut sphere);
        shape.add_child(&mut cylinder);

        let expected_minimum = Tuple::point(-4.5, -3., -5.);
        let expected_maximum = Tuple::point(4., 7., 4.5);

        let actual = shape.bounds_of();

        assert_eq!(expected_minimum, actual.minimum);
        assert_eq!(expected_maximum, actual.maximum);
    }

    #[test]
    fn intersecting_ray_group_does_not_test_children_if_box_is_missed() {
        let mut child = Shape::TestShape(TestShape::new());
        let mut shape = Shape::Group(Group::new());
        shape.add_child(&mut child);
        let ray = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 1., 0.));

        let expected = unsafe {
            SAVED_RAY
        };

        let _intersection = shape.intersect(ray);

        let actual = unsafe {
            SAVED_RAY
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn intersecting_ray_group_tests_children_if_box_is_hit() {
        let mut child = Shape::TestShape(TestShape::new());
        let mut shape = Shape::Group(Group::new());
        shape.add_child(&mut child);
        let ray = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));

        let expected = ray;

        let _intersection = shape.intersect(ray);

        let actual = unsafe {
            SAVED_RAY
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn partitioning_groups_children() {
        let mut sphere1 = Shape::Sphere(Sphere::new());
        sphere1.set_transform(&translate(-2., 0., 0.));
        let mut sphere2 = Shape::Sphere(Sphere::new());
        sphere2.set_transform(&translate(2., 0., 0.));
        let mut sphere3 = Shape::Sphere(Sphere::new());
        let mut group = Shape::Group(Group::new());
        group.add_child(&mut sphere1);
        group.add_child(&mut sphere2);
        group.add_child(&mut sphere3);

        let expected_left = vec![sphere1.clone()];
        let expected_right = vec![sphere2.clone()];
        let expected_group = vec![sphere3.clone()];
        let expected_count = 1;

        let (actual_left, actual_right) = group.partition_children();
        let actual_group = if let Shape::Group(group) = group { group } else { panic!("") };

        assert_eq!(expected_group[0], actual_group.shapes[0]);
        assert_eq!(expected_count, actual_group.shapes.len());
        assert_eq!(expected_left[0], actual_left[0]);
        assert_eq!(expected_right[0], actual_right[0]);
    }

    #[test]
    fn creating_sub_group_from_list_of_children() {
        let sphere1 = Shape::Sphere(Sphere::new());
        let sphere2 = Shape::Sphere(Sphere::new());
        let mut group = Shape::Group(Group::new());

        let expected_group = vec![sphere1.clone(), sphere2.clone()];

        group.make_subgroup(&mut vec![sphere1, sphere2]);

        let actual_group = if let Shape::Group(group) = group { group } else { panic!("") };

        assert_eq!(expected_group[0].get_id(), actual_group.shapes[0].get_shapes()[0].get_id());
        assert_eq!(expected_group[1].get_id(), actual_group.shapes[0].get_shapes()[1].get_id());
    }

    #[test]
    fn subdividing_groups_partitions_its_children() {
        let mut sphere1 = Shape::Sphere(Sphere::new());
        sphere1.set_transform(&translate(-2., -2., 0.));
        let mut sphere2 = Shape::Sphere(Sphere::new());
        sphere2.set_transform(&translate(-2., 2., 0.));
        let mut sphere3 = Shape::Sphere(Sphere::new());
        sphere3.set_transform(&scale(4., 4., 4.));
        let mut group = Shape::Group(Group::new());
        group.add_child(&mut sphere1);
        group.add_child(&mut sphere2);
        group.add_child(&mut sphere3);

        let expected_group = vec![sphere3.clone()];
        let expected_subgroup_count = 2;
        let expected_subgroup1 = vec![sphere1.clone()];
        let expected_subgroup2 = vec![sphere2.clone()];

        group.divide(1);

        let actual_group = group.clone();

        assert_eq!(expected_group[0].get_id(), actual_group.get_shapes()[0].get_id());
        assert_eq!(expected_group[0].get_transform(), actual_group.get_shapes()[0].get_transform());
        assert_eq!(expected_subgroup_count, actual_group.get_shapes()[1].get_shapes().len());
        assert_eq!(expected_subgroup1[0].get_id(), actual_group.get_shapes()[1].get_shapes()[0].get_shapes()[0].get_id());
        assert_eq!(expected_subgroup1[0].get_transform(), actual_group.get_shapes()[1].get_shapes()[0].get_shapes()[0].get_transform());
        assert_eq!(expected_subgroup2[0].get_id(), actual_group.get_shapes()[1].get_shapes()[1].get_shapes()[0].get_id());
        assert_eq!(expected_subgroup2[0].get_transform(), actual_group.get_shapes()[1].get_shapes()[1].get_shapes()[0].get_transform());
    }

    #[test]
    fn subdividing_group_with_too_few_children() {
        let mut sphere1 = Shape::Sphere(Sphere::new());
        sphere1.set_transform(&translate(-2., 0., 0.));
        let mut sphere2 = Shape::Sphere(Sphere::new());
        sphere2.set_transform(&translate(2., 1., 0.));
        let mut sphere3 = Shape::Sphere(Sphere::new());
        sphere3.set_transform(&translate(2., -1., 0.));
        let mut subgroup = Shape::Group(Group::new());
        subgroup.add_child(&mut sphere1);
        subgroup.add_child(&mut sphere2);
        subgroup.add_child(&mut sphere3);
        let mut sphere4 = Shape::Sphere(Sphere::new());
        let mut group = Shape::Group(Group::new());
        group.add_child(&mut subgroup);
        group.add_child(&mut sphere4);

        let expected_subgroup = subgroup.clone();
        let expected_sphere4 = sphere4.clone();
        let expected_subgroup_count = 2;
        let expected_subgroup1 = vec![sphere1.clone()];
        let expected_subgroup2 = vec![sphere2.clone(), sphere3.clone()];

        group.divide(3);

        let actual_group = group.clone();

        assert_eq!(expected_subgroup.get_id(), actual_group.get_shapes()[0].get_id());
        assert_eq!(expected_sphere4.get_id(), actual_group.get_shapes()[1].get_id());
        assert_eq!(expected_subgroup_count, actual_group.get_shapes()[0].get_shapes().len());
        assert_eq!(expected_subgroup1[0].get_id(), actual_group.get_shapes()[0].get_shapes()[0].get_shapes()[0].get_id());
        assert_eq!(expected_subgroup1[0].get_transform(), actual_group.get_shapes()[0].get_shapes()[0].get_shapes()[0].get_transform());
        assert_eq!(expected_subgroup2[0].get_id(), actual_group.get_shapes()[0].get_shapes()[1].get_shapes()[0].get_id());
        assert_eq!(expected_subgroup2[0].get_transform(), actual_group.get_shapes()[0].get_shapes()[1].get_shapes()[0].get_transform());
        assert_eq!(expected_subgroup2[1].get_id(), actual_group.get_shapes()[0].get_shapes()[1].get_shapes()[1].get_id());
        assert_eq!(expected_subgroup2[1].get_transform(), actual_group.get_shapes()[0].get_shapes()[1].get_shapes()[1].get_transform());
    }
}
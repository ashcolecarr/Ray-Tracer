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
pub struct CSG {
    id: i32,
    pub transform: Matrix,
    pub material: Material,
    pub casts_shadow: bool,
    //pub shapes: Vec<Shape>,
    pub operation: String,
    pub left: Box<Shape>,
    pub right: Box<Shape>,
    pub parent: Option<i32>,
}

impl PartialEq for CSG {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.transform == other.transform && 
            self.material == other.material && self.casts_shadow == other.casts_shadow &&
            self.operation == other.operation && self.parent == other.parent
    }
}

impl CSG {
    pub fn new(operation: String, left: &mut Shape, right: &mut Shape) -> Self {
        let csg_id = generate_object_id();
        left.set_parent(csg_id);
        right.set_parent(csg_id);
        
        let new_csg = Self {
            id: csg_id,
            transform: Matrix::identity(4),
            material: Default::default(),
            casts_shadow: true,
            operation,
            left: Box::new(left.clone()),
            right: Box::new(right.clone()),
            //shapes: vec![],
            parent: None,
        };

        CSG::update_csg_reference(new_csg.clone());

        new_csg
    }

    pub fn get_id(&self) -> &i32 {
        &self.id
    }

    //pub fn add_child(&mut self, shape: &mut Shape) {
    //    shape.set_parent(self.id);
    //    self.shapes.push(shape.clone());

    //    Group::update_group_reference(self.clone());
    //}

    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let left_intersections = self.left.intersect(ray);
        let right_intersections = self.right.intersect(ray);

        let mut intersections = [left_intersections, right_intersections].concat();
        intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

        self.filter_intersections(intersections)
    }

    pub fn normal_at(&self, _world_point: Tuple, _hit: Intersection) -> Tuple {
        panic!("Normal at cannot be calculated on a csg.")
    }

    pub fn update_csg_reference(csg: Self) {
        let read_reference = PARENT_REFERENCES.read().unwrap();
        let index = read_reference.iter().position(|pr| pr.get_id() == *csg.get_id()); 
        drop(read_reference);

        let mut write_reference = PARENT_REFERENCES.write().unwrap();
        match index {
            Some(i) => {
                write_reference.remove(i);
                write_reference.push(Shape::CSG(csg));
            },
            None => write_reference.push(Shape::CSG(csg)),
        };
    }

    pub fn intersection_allowed(operation: String, left_hit: bool,
        inside_left_hit: bool, inside_right_hit: bool) -> bool {
        
        match operation.as_str() {
            "union" => (left_hit && !inside_right_hit) || (!left_hit && !inside_left_hit),
            "intersection" => (left_hit && inside_right_hit) || (!left_hit && inside_left_hit),
            "difference" => (left_hit && !inside_right_hit) || (!left_hit && inside_left_hit),
            _ => false,
        }
    }

    pub fn filter_intersections(&self, intersections: Vec<Intersection>) -> Vec<Intersection> {
        let mut inside_left_hit = false;
        let mut inside_right_hit = false;
        let mut result: Vec<Intersection> = vec![];

        for intersection in intersections {
            let left_hit = self.left.includes(intersection.object.clone());

            if CSG::intersection_allowed(self.operation.clone(), left_hit, inside_left_hit, inside_right_hit) {
                result.push(intersection);
            }

            if left_hit {
                inside_left_hit = !inside_left_hit;
            } else {
                inside_right_hit = !inside_right_hit;
            }
        }

        result
    }

    pub fn bounds_of(&self) -> Bound {
        let mut group_box = Bound::bounding_box_empty();

        //for shape in &self.shapes {
        //    let shape_box = shape.parent_space_bounds_of();
        //    group_box.add_box(shape_box);
        //}

        group_box
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::cube::Cube;
    use super::super::intersections;
    use super::super::shape::{CommonShape, Shape};
    use super::super::sphere::Sphere;
    use super::super::translate;

    #[test]
    fn csg_is_created_with_operation_and_two_shapes() {
        let mut shape1 = Shape::Sphere(Sphere::new());
        let mut shape2 = Shape::Cube(Cube::new());

        let actual = CSG::new(String::from("union"), &mut shape1, &mut shape2);

        let expected = CSG {
            id: shape1.get_parent().unwrap(),
            transform: Matrix::identity(4),
            material: Default::default(),
            casts_shadow: true,
            operation: String::from("union"),
            left: Box::new(shape1.clone()),
            right: Box::new(shape2.clone()),
            parent: None,
        };


        assert_eq!(expected.operation, actual.operation);
        assert_eq!(expected.left.get_id(), actual.left.get_id());
        assert_eq!(expected.right.get_id(), actual.right.get_id());
        assert_eq!(shape1.get_parent().unwrap(), *actual.get_id());
        assert_eq!(shape2.get_parent().unwrap(), *actual.get_id());
    }

    #[test]
    fn evaluating_rule_for_csg_operation() {
        struct Operation {
            operation: String,
            left_hit: bool,
            inside_left_hit: bool,
            inside_right_hit: bool,
            result: bool,
        };

        let operation_truth_table = vec![
            Operation { operation: String::from("union"), left_hit: true, inside_left_hit: true, inside_right_hit: true, result: false },
            Operation { operation: String::from("union"), left_hit: true, inside_left_hit: true, inside_right_hit: false, result: true, },
            Operation { operation: String::from("union"), left_hit: true, inside_left_hit: false, inside_right_hit: true, result: false, },
            Operation { operation: String::from("union"), left_hit: true, inside_left_hit: false, inside_right_hit: false, result: true, },
            Operation { operation: String::from("union"), left_hit: false, inside_left_hit: true, inside_right_hit: true, result: false, },
            Operation { operation: String::from("union"), left_hit: false, inside_left_hit: true, inside_right_hit: false, result: false, },
            Operation { operation: String::from("union"), left_hit: false, inside_left_hit: false, inside_right_hit: true, result: true, },
            Operation { operation: String::from("union"), left_hit: false, inside_left_hit: false, inside_right_hit: false, result: true, },
            Operation { operation: String::from("intersection"), left_hit: true, inside_left_hit: true, inside_right_hit: true, result: true, },
            Operation { operation: String::from("intersection"), left_hit: true, inside_left_hit: true, inside_right_hit: false, result: false, },
            Operation { operation: String::from("intersection"), left_hit: true, inside_left_hit: false, inside_right_hit: true, result: true, },
            Operation { operation: String::from("intersection"), left_hit: true, inside_left_hit: false, inside_right_hit: false, result: false, },
            Operation { operation: String::from("intersection"), left_hit: false, inside_left_hit: true, inside_right_hit: true, result: true, },
            Operation { operation: String::from("intersection"), left_hit: false, inside_left_hit: true, inside_right_hit: false, result: true, },
            Operation { operation: String::from("intersection"), left_hit: false, inside_left_hit: false, inside_right_hit: true, result: false, },
            Operation { operation: String::from("intersection"), left_hit: false, inside_left_hit: false, inside_right_hit: false, result: false, },
            Operation { operation: String::from("difference"), left_hit: true, inside_left_hit: true, inside_right_hit: true, result: false, },
            Operation { operation: String::from("difference"), left_hit: true, inside_left_hit: true, inside_right_hit: false, result: true, },
            Operation { operation: String::from("difference"), left_hit: true, inside_left_hit: false, inside_right_hit: true, result: false, },
            Operation { operation: String::from("difference"), left_hit: true, inside_left_hit: false, inside_right_hit: false, result: true, },
            Operation { operation: String::from("difference"), left_hit: false, inside_left_hit: true, inside_right_hit: true, result: true, },
            Operation { operation: String::from("difference"), left_hit: false, inside_left_hit: true, inside_right_hit: false, result: true, },
            Operation { operation: String::from("difference"), left_hit: false, inside_left_hit: false, inside_right_hit: true, result: false, },
            Operation { operation: String::from("difference"), left_hit: false, inside_left_hit: false, inside_right_hit: false, result: false, },
        ];

        for operation in operation_truth_table {
            let expected = operation.result;

            let actual = CSG::intersection_allowed(operation.operation, operation.left_hit, 
                operation.inside_left_hit, operation.inside_right_hit);
            
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn filtering_list_of_intersections() {
        let mut shape1 = Shape::Sphere(Sphere::new());
        let mut shape2 = Shape::Cube(Cube::new());
        let intersections = intersections!(Intersection::new(1., shape1.clone()),
            Intersection::new(2., shape2.clone()), Intersection::new(3., shape1.clone()),
            Intersection::new(4., shape2.clone()));

        let expecteds = vec![("union", 0, 3), ("intersection", 1, 2), ("difference", 0, 1)];

        for expected in expecteds {
            let csg_shape = CSG::new(String::from(expected.0), &mut shape1, &mut shape2);
            let actual = csg_shape.filter_intersections(intersections.clone());

            assert_eq!(2, actual.len());
            assert_eq!(intersections[expected.1], actual[0]);
            assert_eq!(intersections[expected.2], actual[1]);
        }
    }

    #[test]
    fn ray_misses_csg_object() {
        let mut shape1 = Shape::Sphere(Sphere::new());
        let mut shape2 = Shape::Cube(Cube::new());
        let csg = CSG::new(String::from("union"), &mut shape1, &mut shape2);
        let ray = Ray::new(Tuple::point(0., 2., -5.), Tuple::vector(0., 0., 1.));

        let actual = csg.intersect(ray);

        assert!(actual.is_empty());
    }

    #[test]
    fn ray_hits_csg_object() {
        let mut shape1 = Shape::Sphere(Sphere::new());
        let mut shape2 = Shape::Cube(Cube::new());
        shape2.set_transform(translate(0., 0., 0.5));
        let csg = CSG::new(String::from("union"), &mut shape1, &mut shape2);
        let ray = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));

        let expected_t1 = 4.;
        let expected_id1 = shape1.get_id();
        let expected_t2 = 6.5;
        let expected_id2 = shape2.get_id();

        let actual = csg.intersect(ray);

        assert_eq!(2, actual.len());
        assert_eq!(expected_t1, actual[0].t);
        assert_eq!(expected_id1, actual[0].object.get_id());
        assert_eq!(expected_t2, actual[1].t);
        assert_eq!(expected_id2, actual[1].object.get_id());
    }
}
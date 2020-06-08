use super::bound::Bound;
use super::cone::Cone;
use super::cube::Cube;
use super::cylinder::Cylinder;
use super::generate_object_id;
use super::group::Group;
use super::intersection::Intersection;
use super::material::Material;
use super::matrix::Matrix;
use super::PARENT_REFERENCES;
use super::plane::Plane;
use super::ray::Ray;
use super::sphere::Sphere;
use super::tuple::Tuple;

// For testing only.
static mut SAVED_RAY: Ray = Ray {
    origin: Tuple { x: 0., y: 0., z: 0., w: 0. },
    direction: Tuple { x: 0., y: 0., z: 0., w: 0. },
};

#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Sphere (Sphere),
    Plane (Plane),
    Cube (Cube),
    Cylinder (Cylinder),
    Cone (Cone),
    Group (Group),
    TestShape (TestShape),
}

pub trait CommonShape {
    fn intersect(&self, ray: Ray) -> Vec<Intersection>;
    fn normal_at(&self, world_point: Tuple) -> Tuple;
    fn get_id(&self) -> i32;
    fn get_transform(&self) -> Matrix;
    fn set_transform(&mut self, transform: Matrix);
    fn get_material(&self) -> Material;
    fn set_material(&mut self, material: Material);
    fn get_casts_shadow(&self) -> bool;
    fn set_casts_shadow(&mut self, casts_shadow: bool);
    fn get_minimum(&self) -> f64;
    fn set_minimum(&mut self, minimum: f64);
    fn get_maximum(&self) -> f64;
    fn set_maximum(&mut self, maximum: f64);
    fn get_closed(&self) -> bool;
    fn set_closed(&mut self, closed: bool);
    fn get_parent(&self) -> Option<i32>;
    fn set_parent(&mut self, parent: i32);
    fn add_child(&mut self, shape: &mut Shape);
    fn world_to_object(&self, point: Tuple) -> Tuple;
    fn normal_to_world(&self, normal: Tuple) -> Tuple;
    fn bounds_of(&self) -> Bound;
}

impl CommonShape for Shape {
    fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let inverse = self.get_transform().inverse().unwrap();
        let local_ray = ray.transform(inverse);

        match self {
            Shape::Sphere(sphere) => sphere.intersect(local_ray),
            Shape::Plane(plane) => plane.intersect(local_ray),
            Shape::Cube(cube) => cube.intersect(local_ray),
            Shape::Cylinder(cylinder) => cylinder.intersect(local_ray),
            Shape::Cone(cone) => cone.intersect(local_ray),
            Shape::Group(group) => group.intersect(local_ray),
            Shape::TestShape(test_shape) => test_shape.intersect(local_ray),
        }
    }
    
    fn normal_at(&self, world_point: Tuple) -> Tuple {
        let local_point = self.world_to_object(world_point);
        let local_normal = match self {
            Shape::Sphere(sphere) => sphere.normal_at(local_point),
            Shape::Plane(plane) => plane.normal_at(local_point),
            Shape::Cube(cube) => cube.normal_at(local_point),
            Shape::Cylinder(cylinder) => cylinder.normal_at(local_point),
            Shape::Cone(cone) => cone.normal_at(local_point),
            Shape::Group(group) => group.normal_at(local_point),
            Shape::TestShape(test_shape) => test_shape.normal_at(local_point),
        };

        self.normal_to_world(local_normal)
    }

    fn get_id(&self) -> i32 {
        match self.clone() {
            Shape::Sphere(sphere) => *sphere.get_id(),
            Shape::Plane(plane) => *plane.get_id(),
            Shape::Cube(cube) => *cube.get_id(),
            Shape::Cylinder(cylinder) => *cylinder.get_id(),
            Shape::Cone(cone) => *cone.get_id(),
            Shape::Group(group) => *group.get_id(),
            Shape::TestShape(test_shape) => *test_shape.get_id(),
        }
    }

    fn get_transform(&self) -> Matrix {
        match self.clone() {
            Shape::Sphere(sphere) => sphere.transform,
            Shape::Plane(plane) => plane.transform,
            Shape::Cube(cube) => cube.transform,
            Shape::Cylinder(cylinder) => cylinder.transform,
            Shape::Cone(cone) => cone.transform,
            Shape::Group(group) => group.transform,
            Shape::TestShape(test_shape) => test_shape.transform,
        }
    }

    fn set_transform(&mut self, transform: Matrix) {
        match self {
            Shape::Sphere(sphere) => sphere.transform = transform,
            Shape::Plane(plane) => plane.transform = transform,
            Shape::Cube(cube) => cube.transform = transform,
            Shape::Cylinder(cylinder) => cylinder.transform = transform,
            Shape::Cone(cone) => cone.transform = transform,
            Shape::Group(group) => { 
                group.transform = transform;

                Group::update_group_reference(group.clone());
            },
            Shape::TestShape(test_shape) => test_shape.transform = transform,
        }
    }

    fn get_material(&self) -> Material {
        match self.get_parent() {
            Some(parent) => {
                let read_reference = PARENT_REFERENCES.read().unwrap();
                read_reference.iter().find(|pr| pr.get_id() == parent).unwrap().get_material()
            },
            None => {
                match self.clone() {
                    Shape::Sphere(sphere) => sphere.material,
                    Shape::Plane(plane) => plane.material,
                    Shape::Cube(cube) => cube.material,
                    Shape::Cylinder(cylinder) => cylinder.material,
                    Shape::Cone(cone) => cone.material,
                    Shape::Group(group) => group.material,
                    Shape::TestShape(test_shape) => test_shape.material,
                }
            }
        }
    }

    fn set_material(&mut self, material: Material) {
        match self {
            Shape::Sphere(sphere) => sphere.material = material,
            Shape::Plane(plane) => plane.material = material,
            Shape::Cube(cube) => cube.material = material,
            Shape::Cylinder(cylinder) => cylinder.material = material,
            Shape::Cone(cone) => cone.material = material,
            Shape::Group(group) => { 
                group.material = material.clone();

                Group::update_group_reference(group.clone());
            },
            Shape::TestShape(test_shape) => test_shape.material = material,
        }
    }

    fn get_casts_shadow(&self) -> bool {
        match self.clone() {
            Shape::Sphere(sphere) => sphere.casts_shadow,
            Shape::Plane(plane) => plane.casts_shadow,
            Shape::Cube(cube) => cube.casts_shadow,
            Shape::Cylinder(cylinder) => cylinder.casts_shadow,
            Shape::Cone(cone) => cone.casts_shadow,
            Shape::Group(group) => group.casts_shadow,
            Shape::TestShape(test_shape) => test_shape.casts_shadow,
        }
    }

    fn set_casts_shadow(&mut self, casts_shadow: bool) {
        match self {
            Shape::Sphere(sphere) => sphere.casts_shadow = casts_shadow,
            Shape::Plane(plane) => plane.casts_shadow = casts_shadow,
            Shape::Cube(cube) => cube.casts_shadow = casts_shadow,
            Shape::Cylinder(cylinder) => cylinder.casts_shadow = casts_shadow,
            Shape::Cone(cone) => cone.casts_shadow = casts_shadow,
            Shape::Group(group) => { 
                group.casts_shadow = casts_shadow;

                Group::update_group_reference(group.clone());
            },
            Shape::TestShape(test_shape) => test_shape.casts_shadow = casts_shadow,
        }
    }

    fn get_minimum(&self) -> f64 {
        match self {
            Shape::Cylinder(cylinder) => cylinder.minimum,
            Shape::Cone(cone) => cone.minimum,
            _ => panic!("Minimum property is not available for this shape."),
        }
    }

    fn set_minimum(&mut self, minimum: f64) {
        match self {
            Shape::Cylinder(cylinder) => cylinder.minimum = minimum,
            Shape::Cone(cone) => cone.minimum = minimum,
            _ => panic!("Minimum property is not available for this shape."),
        }
    }
    fn get_maximum(&self) -> f64 {
        match self {
            Shape::Cylinder(cylinder) => cylinder.maximum,
            Shape::Cone(cone) => cone.maximum,
            _ => panic!("Maximum property is not available for this shape."),
        }
    }

    fn set_maximum(&mut self, maximum: f64) {
        match self {
            Shape::Cylinder(cylinder) => cylinder.maximum = maximum,
            Shape::Cone(cone) => cone.maximum = maximum,
            _ => panic!("Maximum property is not available for this shape."),
        }
    }

    fn get_closed(&self) -> bool {
        match self {
            Shape::Cylinder(cylinder) => cylinder.closed,
            Shape::Cone(cone) => cone.closed,
            _ => panic!("Closed property is not available for this shape."),
        }
    }

    fn set_closed(&mut self, closed: bool) {
        match self {
            Shape::Cylinder(cylinder) => cylinder.closed = closed,
            Shape::Cone(cone) => cone.closed = closed,
            _ => panic!("Closed property is not available for this shape."),
        }
    }
    
    fn get_parent(&self) -> Option<i32> {
        match self.clone() {
            Shape::Sphere(sphere) => sphere.parent,
            Shape::Plane(plane) => plane.parent,
            Shape::Cube(cube) => cube.parent,
            Shape::Cylinder(cylinder) => cylinder.parent,
            Shape::Cone(cone) => cone.parent,
            Shape::Group(group) => group.parent,
            Shape::TestShape(test_shape) => test_shape.parent,
        }
    }

    fn set_parent(&mut self, parent: i32) {
        match self {
            Shape::Sphere(sphere) => sphere.parent = Some(parent),
            Shape::Plane(plane) => plane.parent = Some(parent),
            Shape::Cube(cube) => cube.parent = Some(parent),
            Shape::Cylinder(cylinder) => cylinder.parent = Some(parent),
            Shape::Cone(cone) => cone.parent = Some(parent),
            Shape::Group(group) => {
                group.parent = Some(parent);

                Group::update_group_reference(group.clone());
            },
            Shape::TestShape(test_shape) => test_shape.parent = Some(parent),
        }
    }
    
    fn add_child(&mut self, shape: &mut Shape) {
        match self {
            Shape::Group(group) => group.add_child(shape),
            _ => panic!("Only groups can contain children."),
        }
    }

    fn world_to_object(&self, point: Tuple) -> Tuple {
        let parent = self.get_parent();
        self.get_transform().inverse().unwrap() * match parent {
            Some(parent) => {
                let parent_references = PARENT_REFERENCES.read().unwrap();
                let parent_shape = parent_references.iter().find(|pr| pr.get_id() == parent).unwrap();
                parent_shape.world_to_object(point)
            },
            None => point,
        }
    }

    fn normal_to_world(&self, normal: Tuple) -> Tuple {
        let mut new_normal = self.get_transform().inverse().unwrap().transpose() * normal;
        new_normal.w = 0.;
        new_normal = new_normal.normalize();

        let parent = self.get_parent();
        match parent {
            Some(parent) => {
                let parent_references = PARENT_REFERENCES.read().unwrap();
                let parent_shape = parent_references.iter().find(|pr| pr.get_id() == parent).unwrap();
                parent_shape.normal_to_world(new_normal)
            },
            None => new_normal,
        }
    }

    fn bounds_of(&self) -> Bound {
        match self {
            Shape::Sphere(sphere) => sphere.bounds_of(),
            Shape::Plane(plane) => plane.bounds_of(),
            Shape::Cube(cube) => cube.bounds_of(),
            Shape::Cylinder(cylinder) => cylinder.bounds_of(),
            Shape::Cone(cone) => cone.bounds_of(),
            //Shape::Triangle(triangle) => triangle.bounds_of(),
            //Shape::Group(group) => group.bounds_of(),
            Shape::TestShape(test_shape) => test_shape.bounds_of(),
            _ => Bound::bounding_box_empty(),
        }
    }
}

/// For testing purposes only--not meant to be used directly.
#[derive(Debug, Clone)]
pub struct TestShape {
    id: i32,
    pub transform: Matrix,
    pub material: Material,
    pub casts_shadow: bool,
    pub parent: Option<i32>,
}

impl PartialEq for TestShape {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.transform == other.transform && 
            self.material == other.material && self.casts_shadow == other.casts_shadow &&
            self.parent == other.parent
    }
}

impl TestShape {
    pub fn new() -> Self {
        Self {
            id: generate_object_id(),
            transform: Matrix::identity(4),
            material: Default::default(),
            casts_shadow: true,
            parent: None,
        }
    }    

    pub fn get_id(&self) -> &i32 {
        &self.id
    }

    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        unsafe {
            SAVED_RAY = ray;
        }

        vec![]
    }

    pub fn normal_at(&self, world_point: Tuple) -> Tuple {
        Tuple::vector(world_point.x, world_point.y, world_point.z)
    }

    pub fn bounds_of(&self) -> Bound {
        Bound::bounding_box_init(Tuple::point(-1., -1., -1.),
            Tuple::point(1., 1., 1.))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::material::Material;
    use super::super::matrix::Matrix;
    use super::super::transformation::*;
    use super::super::tuple::Tuple;
    use std::f64::consts::PI;

    #[test]
    fn default_transformation() {
        let shape = Shape::TestShape(TestShape::new());

        let expected = Matrix::identity(4);

        let actual = shape.get_transform();

        assert_eq!(expected, actual);
    }

    #[test]
    fn assigning_transformation() {
        let mut shape = Shape::TestShape(TestShape::new());
        let transform =  translate(2., 3., 4.);
        shape.set_transform(transform.clone());
        
        let expected = transform;

        let actual = shape.get_transform();

        assert_eq!(expected, actual);
    }

    #[test]
    fn default_material() {
        let shape = Shape::TestShape(TestShape::new());

        let expected: Material = Default::default();

        let actual = shape.get_material();

        assert_eq!(expected, actual);
    }

    #[test]
    fn sphere_may_be_assigned_material() {
        let mut shape = Shape::TestShape(TestShape::new());
        let mut material: Material = Default::default();
        material.ambient = 1.;
        shape.set_material(material.clone());
        
        let expected = material.clone();

        let actual = shape.get_material();

        assert_eq!(expected, actual);
    }

    #[test]
    fn intersecting_scaled_shape_with_ray() {
        let ray = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let mut shape = Shape::TestShape(TestShape::new());
        shape.set_transform(scale(2., 2., 2.));
        let _intersections = shape.intersect(ray);

        let expected = Ray::new(Tuple::point(0., 0., -2.5), Tuple::vector(0., 0., 0.5));

        let actual = unsafe {
            SAVED_RAY
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn intersecting_translated_shape_with_ray() {
        let ray = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let mut shape = Shape::TestShape(TestShape::new());
        shape.set_transform(translate(5., 0., 0.));
        let _intersections = shape.intersect(ray);

        let expected = Ray::new(Tuple::point(-5., 0., -5.), Tuple::vector(0., 0., 1.));

        let actual = unsafe {
            SAVED_RAY
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn computing_normal_on_translated_shape() {
        let mut shape = Shape::TestShape(TestShape::new());
        shape.set_transform(translate(0., 1., 0.));

        let expected = Tuple::vector(0., 0.70711, -0.70711);

        let actual = shape.normal_at(Tuple::point(0., 1.70711, -0.70711));

        assert_eq!(expected, actual);
    }

    #[test]
    fn computing_normal_on_transformed_sphere() {
        let mut shape = Shape::TestShape(TestShape::new());
        let transform = scale(1., 0.5, 1.) * rotate(PI / 5., Axis::Z);
        shape.set_transform(transform);

        let expected = Tuple::vector(0., 0.97014, -0.24254);

        let actual = shape.normal_at(Tuple::point(0., 2_f64.sqrt() / 2., -2_f64.sqrt() / 2.));

        assert_eq!(expected, actual);
    }

    #[test]
    fn default_shadow_casting() {
        let shape = Shape::TestShape(TestShape::new());

        let expected = true;

        let actual = shape.get_casts_shadow();

        assert_eq!(expected, actual);
    }

    #[test]
    fn assigning_shadow_casting() {
        let mut shape = Shape::TestShape(TestShape::new());
        shape.set_casts_shadow(false);

        let expected = false;

        let actual = shape.get_casts_shadow();

        assert_eq!(expected, actual);
    }

    #[test]
    fn shape_has_parent_attribute() {
        let actual = TestShape::new();

        assert!(actual.parent.is_none());
    }

    #[test]
    fn converting_point_from_world_space_to_object_space() {
        let mut group1 = Shape::Group(Group::new());
        group1.set_transform(rotate(PI / 2., Axis::Y));
        let mut group2 = Shape::Group(Group::new());
        group2.set_transform(scale(2., 2., 2.));
        group1.add_child(&mut group2);
        let mut shape = Shape::Sphere(Sphere::new());
        shape.set_transform(translate(5., 0., 0.));
        group2.add_child(&mut shape);

        let expected = Tuple::point(0., 0., -1.);

        let actual = shape.world_to_object(Tuple::point(-2., 0., -10.));

        assert_eq!(expected, actual);
    }

    #[test]
    fn converting_normal_from_object_to_world_space() {
        let mut group1 = Shape::Group(Group::new());
        group1.set_transform(rotate(PI / 2., Axis::Y));
        let mut group2 = Shape::Group(Group::new());
        group2.set_transform(scale(1., 2., 3.));
        group1.add_child(&mut group2);
        let mut shape = Shape::Sphere(Sphere::new());
        shape.set_transform(translate(5., 0., 0.));
        group2.add_child(&mut shape);

        let expected = Tuple::vector(0.28571, 0.42857, -0.85714);

        let actual = shape.normal_to_world(Tuple::vector(3_f64.sqrt() / 3., 3_f64.sqrt() / 3., 3_f64.sqrt() / 3.));

        assert_eq!(expected, actual);
    }

    #[test]
    fn finding_normal_on_child_object() {
        let mut group1 = Shape::Group(Group::new());
        group1.set_transform(rotate(PI / 2., Axis::Y));
        let mut group2 = Shape::Group(Group::new());
        group2.set_transform(scale(1., 2., 3.));
        group1.add_child(&mut group2);
        let mut shape = Shape::Sphere(Sphere::new());
        shape.set_transform(translate(5., 0., 0.));
        group2.add_child(&mut shape);

        let expected = Tuple::vector(0.2857, 0.42854, -0.85716);

        let actual = shape.normal_at(Tuple::point(1.7321, 1.1547, -5.5774));

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_shape_has_arbitrary_bounds() {
        let shape = TestShape::new();
        let bounding_box = shape.bounds_of();

        let expected_minimum = Tuple::point(-1., -1., -1.);
        let expected_maximum = Tuple::point(1., 1., 1.);

        let actual_minimum = bounding_box.minimum;
        let actual_maximum = bounding_box.maximum;

        assert_eq!(expected_minimum, actual_minimum);
        assert_eq!(expected_maximum, actual_maximum);
    }
}
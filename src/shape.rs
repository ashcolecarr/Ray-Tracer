use super::cone::Cone;
use super::cube::Cube;
use super::cylinder::Cylinder;
use super::group::Group;
use super::intersection::Intersection;
use super::material::Material;
use super::matrix::Matrix;
use super::plane::Plane;
use super::ray::Ray;
use super::sphere::Sphere;
use super::tuple::Tuple;
use core::fmt::Debug;
use core::ptr::null_mut;
use core::sync::atomic::AtomicPtr;
use std::sync::atomic::{AtomicI32, Ordering};

// For testing only.
static mut SAVED_RAY: Ray = Ray {
    origin: Tuple { x: 0., y: 0., z: 0., w: 0. },
    direction: Tuple { x: 0., y: 0., z: 0., w: 0. },
};

#[derive(Debug)]
pub struct Shape {
    id: i32,
    pub transform: Matrix,
    pub material: Material,
    pub casts_shadow: bool,
    pub parent: AtomicPtr<Group>,
}

impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.transform == other.transform &&
            self.material == other.material && self.casts_shadow == other.casts_shadow
    }
}

impl Shape {
    pub fn new() -> Self {
        static ID_COUNT: AtomicI32 = AtomicI32::new(1);

        Self {
            id: ID_COUNT.fetch_add(1, Ordering::Relaxed),
            transform: Matrix::identity(4),
            material: Default::default(),
            casts_shadow: true,
            parent: AtomicPtr::new(null_mut()),
        }
    }
}

pub trait ShapeCommon: Debug {
    fn get_shape(&self) -> &Shape;
    fn get_shape_mut(&self) -> &mut Shape;
    fn local_intersect(&self, ray: Ray) -> Vec<Intersection>;
    fn local_normal_at(&self, world_point: Tuple) -> Tuple;
    fn add_child(&mut self, shape: &mut Shape) {}

    fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let inverse = self.get_transform().inverse().unwrap();
        let local_ray = ray.transform(inverse);

        self.local_intersect(ray)
    }
    
    fn normal_at(&self, world_point: Tuple) -> Tuple {
        let local_point = self.world_to_object(world_point);
        let local_normal = self.local_normal_at(world_point);

        self.normal_to_world(local_normal)
    }

    fn world_to_object(&self, point: Tuple) -> Tuple {
        let parent = self.get_parent();
        self.get_transform().inverse().unwrap() * match parent {
            Some(parent) => self.get_parent().world_to_object(point),
            None => point,
        }
    }

    fn normal_to_world(&self, normal: Tuple) -> Tuple {
        let mut new_normal = self.get_transform().inverse().unwrap().transpose() * normal;
        new_normal.w = 0.;
        new_normal = new_normal.normalize();

        let parent = self.get_parent();
        match parent {
            Some(parent) => self.get_parent().normal_to_world(new_normal),
            None => new_normal,
        }
    }

    fn get_id(&self) -> i32 {
        self.get_shape().id
    }

    fn get_transform(&self) -> Matrix {
        self.get_shape().transform
    }

    fn set_transform(&mut self, transform: Matrix) {
        self.get_shape_mut().transform = transform;
    }

    fn get_material(&self) -> Material {
        self.get_shape().material
    }

    fn set_material(&mut self, material: Material) {
        self.get_shape_mut().material = material;
    }

    fn get_casts_shadow(&self) -> bool {
        self.get_shape().casts_shadow
    }

    fn set_casts_shadow(&mut self, casts_shadow: bool) {
        self.get_shape_mut().casts_shadow = casts_shadow;
    }

    fn get_parent(&self) -> Option<&Group> {
        unsafe {
            self.get_shape().parent.load(Ordering::Relaxed).as_ref()
        }
    }

    fn set_parent(&mut self, group: &mut Group) {
        self.get_shape_mut().parent = AtomicPtr::new(group);
    }
}

/// For testing purposes only--not meant to be used directly.
#[derive(Debug)]
pub struct TestShape {
    pub shape: Shape,
}

impl PartialEq for TestShape {
    fn eq(&self, other: &Self) -> bool {
        self.shape == other.shape
    }
}

impl TestShape {
    pub fn new() -> Self {
        Self {
            shape: Shape::new(),
        }
    }    
}

impl ShapeCommon for TestShape {
    fn get_shape(&self) -> &Shape {
        &self.shape
    }

    fn get_shape_mut(&self) -> &mut Shape {
        &mut self.shape
    }

    fn local_intersect(&self, ray: Ray) -> Vec<Intersection> {
        unsafe {
            SAVED_RAY = ray;
        }

        vec![]
    }

    fn local_normal_at(&self, world_point: Tuple) -> Tuple {
        Tuple::vector(world_point.x, world_point.y, world_point.z)
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
}
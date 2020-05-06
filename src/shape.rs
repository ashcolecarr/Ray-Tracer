use super::intersection::Intersection;
use super::material::Material;
use super::matrix::Matrix;
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
    TestShape (TestShape),
}

pub trait Actionable {
    fn intersect(&self, ray: Ray) -> Vec<Intersection>;
    fn normal_at(&self, world_point: Tuple) -> Tuple;
    fn get_transform(&self) -> Matrix;
    fn set_transform(&mut self, transform: Matrix);
    fn get_material(&self) -> Material;
    fn set_material(&mut self, material: Material);
}

impl Actionable for Shape {
    fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let inverse = self.get_transform().inverse().unwrap();
        let local_ray = ray.transform(inverse);

        match self {
            Shape::Sphere(sphere) => sphere.intersect(local_ray),
            Shape::Plane(plane) => plane.intersect(local_ray),
            Shape::TestShape(test_shape) => test_shape.intersect(local_ray),
        }
    }
    
    fn normal_at(&self, world_point: Tuple) -> Tuple {
        let inverse = self.get_transform().inverse().unwrap();
        let local_point = inverse.clone() * world_point;
        let local_normal = match self {
            Shape::Sphere(sphere) => sphere.normal_at(local_point),
            Shape::Plane(plane) => plane.normal_at(local_point),
            Shape::TestShape(test_shape) => test_shape.normal_at(local_point),
        };

        let mut world_normal = inverse.transpose() * local_normal;
        world_normal.w = 0.;

        world_normal.normalize()
    }

    fn get_transform(&self) -> Matrix {
        match self.clone() {
            Shape::Sphere(sphere) => sphere.transform,
            Shape::Plane(plane) => plane.transform,
            Shape::TestShape(test_shape) => test_shape.transform,
        }
    }

    fn set_transform(&mut self, transform: Matrix) {
        match self {
            Shape::Sphere(sphere) => sphere.transform = transform,
            Shape::Plane(plane) => plane.transform = transform,
            Shape::TestShape(test_shape) => test_shape.transform = transform,
        }
    }

    fn get_material(&self) -> Material {
        match self.clone() {
            Shape::Sphere(sphere) => sphere.material,
            Shape::Plane(plane) => plane.material,
            Shape::TestShape(test_shape) => test_shape.material,
        }
    }

    fn set_material(&mut self, material: Material) {
        match self {
            Shape::Sphere(sphere) => sphere.material = material,
            Shape::Plane(plane) => plane.material = material,
            Shape::TestShape(test_shape) => test_shape.material = material,
        }
    }
}

/// For testing purposes only--not meant to be used directly.
#[derive(Debug, Clone)]
pub struct TestShape {
    pub transform: Matrix,
    pub material: Material,
}

impl PartialEq for TestShape {
    fn eq(&self, other: &Self) -> bool {
        self.transform == other.transform &&
            self.material == other.material
    }
}

impl TestShape {
    pub fn new() -> Self {
        Self {
            transform: Matrix::identity(4),
            material: Default::default(),
        }
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
}
use super::BLACK;
use super::color::Color;
use super::computations::Computations;
use super::intersection::Intersection;
use super::light::Light;
use super::material::Material;
use super::near_eq;
use super::ray::Ray;
use super::shape::{Shape, CommonShape};
use super::sphere::Sphere;
use super::transformation::*;
use super::tuple::Tuple;
use super::WHITE;

pub struct World {
    pub objects: Vec<Shape>,
    pub lights: Vec<Light>,
}

impl Default for World {
    fn default() -> Self {
        let light = Light::point_light(Tuple::point(-10., 10., -10.), WHITE);

        let mut material: Material = Default::default();
        material.color = Color::new(0.8, 1., 0.6);
        material.diffuse = 0.7;
        material.specular = 0.2;
        let mut sphere1 = Shape::Sphere(Sphere::new());
        sphere1.set_material(&material);

        let mut sphere2 = Shape::Sphere(Sphere::new());
        sphere2.set_transform(&scale(0.5, 0.5, 0.5));

        let lights = vec![light];
        let objects = vec![sphere1, sphere2];

        Self { objects, lights }
    }
}

impl World {
    pub fn new() -> Self {
        Self { objects: vec![], lights: vec![] }
    }

    pub fn intersect_world(&self, ray: Ray) -> Vec<Intersection> {
        let mut intersections: Vec<Intersection> = self.objects.iter().fold(Vec::new(), |mut ints, o| {
            ints.append(&mut o.intersect(ray));
            ints
        }); 

        intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

        intersections
    }

    pub fn shade_hit(&self, computations: &Computations, remaining: i32) -> Color {
        let mut surface = BLACK;
        let shadowed = self.is_shadowed(computations.over_point);
        let material = computations.object.get_material();

        for (light, shadow) in self.lights.iter().zip(shadowed) {
            surface += material.lighting(computations.object.clone(),
                *light, computations.over_point, computations.eye_vector, 
                computations.normal_vector, shadow);
        }
        
        let reflected = self.reflected_color(computations, remaining);
        let refracted = self.refracted_color(computations, remaining);
        
        let material = computations.object.get_material();
        if material.reflective > 0. && material.transparency > 0. {
            let reflectance = Intersection::schlick(computations);

            surface + reflected * reflectance + refracted * (1. - reflectance)
        } else {
            surface + reflected + refracted
        }
    }

    pub fn color_at(&self, ray: Ray, remaining: i32) -> Color {
        let intersections = self.intersect_world(ray);
        let hit = Intersection::hit(&intersections);
        
        if hit.is_none() {
            BLACK
        } else {
            self.shade_hit(&hit.unwrap().prepare_computations(ray, &intersections), remaining)
        }
    }

    pub fn is_shadowed(&self, point: Tuple) -> Vec<bool> {
        self.lights.iter().map(|light| {
            let vector = light.position - point;
            let distance = vector.magnitude();
            let direction = vector.normalize();

            let ray = Ray::new(point, direction);
            let intersections = self.intersect_world(ray);

            let hit = Intersection::hit(&intersections);
            match hit {
                Some(hit) => hit.t < distance && hit.object.get_casts_shadow(),
                None => false,
            }
        }).collect::<Vec<bool>>()
    }

    pub fn reflected_color(&self, computations: &Computations, remaining: i32) -> Color {
        if near_eq(computations.object.get_material().reflective, 0.) || remaining <= 0 {
            return BLACK;
        }

        let reflect_ray = Ray::new(computations.over_point, computations.reflect_vector);
        let color = self.color_at(reflect_ray, remaining - 1);

        color * computations.object.get_material().reflective
    }

    pub fn refracted_color(&self, computations: &Computations, remaining: i32) -> Color {
        if near_eq(computations.object.get_material().transparency, 0.) || remaining <= 0 {
            return BLACK;
        }

        let n_ratio = computations.n1 / computations.n2;
        let cos_i = computations.eye_vector.dot(computations.normal_vector);
        let sin2_t = n_ratio.powi(2) * (1. - cos_i.powi(2));
        if sin2_t > 1. {
            return BLACK;
        }

        let cos_t = (1. - sin2_t).sqrt();
        let direction = computations.normal_vector * (n_ratio * cos_i - cos_t) - computations.eye_vector * n_ratio;
        let refract_ray = Ray::new(computations.under_point, direction);

        self.color_at(refract_ray, remaining - 1) * computations.object.get_material().transparency
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::color::Color;
    use super::super::DEFAULT_RECURSION;
    use super::super::intersections;
    use super::super::material::Material;
    use super::super::ORIGIN;
    use super::super::pattern::*;
    use super::super::plane::Plane;
    use super::super::ray::Ray;
    use super::super::shape::{Shape, CommonShape};
    use super::super::tuple::Tuple;

    #[test]
    fn creating_world() {
        let actual = World::new();

        assert!(actual.objects.is_empty());
        assert!(actual.lights.is_empty());
    }

    #[test]
    fn default_world() {
        let expected_light = Light::point_light(Tuple::point(-10., 10., -10.), WHITE);

        let mut material: Material = Default::default();
        material.color = Color::new(0.8, 1., 0.6);
        material.diffuse = 0.7;
        material.specular = 0.2;
        let mut expected_sphere1 = Shape::Sphere(Sphere::new());
        expected_sphere1.set_material(&material);

        let mut expected_sphere2 = Shape::Sphere(Sphere::new());
        expected_sphere2.set_transform(&scale(0.5, 0.5, 0.5));

        let actual: World = Default::default();

        assert_eq!(expected_light, actual.lights[0]);
        assert_eq!(expected_sphere1.get_material().color, actual.objects[0].get_material().color);
        assert_eq!(expected_sphere1.get_material().diffuse, actual.objects[0].get_material().diffuse);
        assert_eq!(expected_sphere1.get_material().specular, actual.objects[0].get_material().specular);
        assert_eq!(expected_sphere2.get_transform(), actual.objects[1].get_transform());
    }

    #[test]
    fn intersect_world_with_ray() {
        let world: World = Default::default();
        let ray = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));

        let expected_count = 4;
        let expected_t1 = 4.;
        let expected_t2 = 4.5;
        let expected_t3 = 5.5;
        let expected_t4 = 6.;

        let actual = world.intersect_world(ray);

        assert_eq!(expected_count, actual.len());
        assert_eq!(expected_t1, actual[0].t);
        assert_eq!(expected_t2, actual[1].t);
        assert_eq!(expected_t3, actual[2].t);
        assert_eq!(expected_t4, actual[3].t);
    }

    #[test]
    fn shading_intersection() {
        let world: World = Default::default();
        let ray = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let shape = world.objects[0].clone();
        let intersection = Intersection::new(4., shape);
        let computations = intersection.prepare_computations(ray, &vec![intersection.clone()]);

        let expected = Color::new(0.38066, 0.47583, 0.2855);

        let actual = world.shade_hit(&computations, DEFAULT_RECURSION);

        assert_eq!(expected, actual);
    }

    #[test]
    fn shading_intersection_from_inside() {
        let mut world: World = Default::default();
        world.lights[0] = Light::point_light(Tuple::point(0., 0.25, 0.), WHITE);
        let ray = Ray::new(ORIGIN, Tuple::vector(0., 0., 1.));
        let shape = world.objects[1].clone();
        let intersection = Intersection::new(0.5, shape);
        let computations = intersection.prepare_computations(ray, &vec![intersection.clone()]);

        let expected = Color::new(0.90498, 0.90498, 0.90498);

        let actual = world.shade_hit(&computations, DEFAULT_RECURSION);

        assert_eq!(expected, actual);
    }

    #[test]
    fn color_when_ray_misses() {
        let world: World = Default::default();
        let ray = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 1., 0.));

        let expected = BLACK;

        let actual = world.color_at(ray, DEFAULT_RECURSION);

        assert_eq!(expected, actual);
    }

    #[test]
    fn color_when_ray_hits() {
        let world: World = Default::default();
        let ray = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));

        let expected = Color::new(0.38066, 0.47583, 0.2855);

        let actual = world.color_at(ray, DEFAULT_RECURSION);

        assert_eq!(expected, actual);
    }

    #[test]
    fn color_with_intersection_behind_ray() {
        let mut world: World = Default::default();
        let mut material: Material = Default::default();
        material.ambient = 1.;
        world.objects[0].set_material(&material);
        world.objects[1].set_material(&material);
        let ray = Ray::new(Tuple::point(0., 0., 0.75), Tuple::vector(0., 0., -1.));

        let expected = world.objects[1].get_material().color;

        let actual = world.color_at(ray, DEFAULT_RECURSION);

        assert_eq!(expected, actual);
    }

    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let world: World = Default::default();
        let point = Tuple::point(0., 10., 0.);

        assert!(!world.is_shadowed(point)[0]);
    }

    #[test]
    fn shadow_when_object_is_between_point_and_light() {
        let world: World = Default::default();
        let point = Tuple::point(10., -10., 10.);

        assert!(world.is_shadowed(point)[0]);
    }

    #[test]
    fn there_is_no_shadow_when_object_is_behind_light() {
        let world: World = Default::default();
        let point = Tuple::point(-20., 20., -20.);

        assert!(!world.is_shadowed(point)[0]);
    }

    #[test]
    fn there_is_no_shadow_when_object_is_behind_point() {
        let world: World = Default::default();
        let point = Tuple::point(-2., 2., -2.);

        assert!(!world.is_shadowed(point)[0]);
    }

    #[test]
    fn shade_hit_is_given_intersection_in_shadow() {
        let mut world = World::new();
        world.lights.push(Light::point_light(Tuple::point(0., 0., -10.), WHITE));
        let sphere1 = Shape::Sphere(Sphere::new());
        world.objects.push(sphere1);
        let mut sphere2 = Shape::Sphere(Sphere::new());
        sphere2.set_transform(&translate(0., 0., 10.));
        world.objects.push(sphere2.clone());
        let ray = Ray::new(Tuple::point(0., 0., 5.), Tuple::vector(0., 0., 1.));
        let intersection = Intersection::new(4., sphere2);
        let computations = intersection.prepare_computations(ray, &vec![intersection.clone()]);

        let expected = Color::new(0.1, 0.1, 0.1);

        let actual = world.shade_hit(&computations, DEFAULT_RECURSION);

        assert_eq!(expected, actual);
    }

    #[test]
    fn reflected_color_for_nonreflective_material() {
        let world: World = Default::default();
        let ray = Ray::new(ORIGIN, Tuple::vector(0., 0., 1.));
        let mut material: Material = Default::default();
        material.ambient = 1.;
        let mut shape = world.objects[1].clone();
        shape.set_material(&material);
        let intersection = Intersection::new(1., shape);
        let computations = intersection.prepare_computations(ray, &vec![intersection.clone()]);

        let expected = BLACK;

        let actual = world.reflected_color(&computations, DEFAULT_RECURSION);

        assert_eq!(expected, actual);
    }

    #[test]
    fn reflected_color_for_reflective_material() {
        let mut world: World = Default::default();
        let mut material: Material = Default::default();
        material.reflective = 0.5;
        let mut shape = Shape::Plane(Plane::new());
        shape.set_material(&material);
        shape.set_transform(&translate(0., -1., 0.));
        world.objects.push(shape.clone());
        let ray = Ray::new(Tuple::point(0., 0., -3.), Tuple::vector(0., -2_f64.sqrt() / 2., 2_f64.sqrt() / 2.));
        let intersection = Intersection::new(2_f64.sqrt(), shape);
        let computations = intersection.prepare_computations(ray, &vec![intersection.clone()]);

        let expected = Color::new(0.19033, 0.23791, 0.14274);

        let actual = world.reflected_color(&computations, DEFAULT_RECURSION);

        assert_eq!(expected, actual);
    }

    #[test]
    fn shade_hit_with_reflective_material() {
        let mut world: World = Default::default();
        let mut material: Material = Default::default();
        material.reflective = 0.5;
        let mut shape = Shape::Plane(Plane::new());
        shape.set_material(&material);
        shape.set_transform(&translate(0., -1., 0.));
        world.objects.push(shape.clone());
        let ray = Ray::new(Tuple::point(0., 0., -3.), Tuple::vector(0., -2_f64.sqrt() / 2., 2_f64.sqrt() / 2.));
        let intersection = Intersection::new(2_f64.sqrt(), shape);
        let computations = intersection.prepare_computations(ray, &vec![intersection.clone()]);

        let expected = Color::new(0.87675, 0.92434, 0.82917);

        let actual = world.shade_hit(&computations, DEFAULT_RECURSION);

        assert_eq!(expected, actual);
    }

    #[test]
    fn color_at_with_mutually_reflective_surfaces() {
        let mut world = World::new();
        world.lights.push(Light::point_light(ORIGIN, WHITE));
        let mut lower_material: Material = Default::default();
        lower_material.reflective = 1.;
        let mut lower = Shape::Plane(Plane::new());
        lower.set_material(&lower_material);
        lower.set_transform(&translate(0., -1., 0.));
        world.objects.push(lower);

        let mut upper_material: Material = Default::default();
        upper_material.reflective = 1.;
        let mut upper = Shape::Plane(Plane::new());
        upper.set_material(&upper_material);
        upper.set_transform(&translate(0., 1., 0.));
        world.objects.push(upper);

        let ray = Ray::new(ORIGIN, Tuple::vector(0., 1., 0.));
        let _color = world.color_at(ray, DEFAULT_RECURSION);

        // If the test got here, then the the infinite recursion did not happen.
        assert!(true);
    }

    #[test]
    fn reflected_color_at_maximum_recursive_depth() {
        let mut world: World = Default::default();
        let mut material: Material = Default::default();
        material.reflective = 0.5;
        let mut shape = Shape::Plane(Plane::new());
        shape.set_material(&material);
        shape.set_transform(&translate(0., -1., 0.));
        world.objects.push(shape.clone());
        let ray = Ray::new(Tuple::point(0., 0., -3.), Tuple::vector(0., -2_f64.sqrt() / 2., 2_f64.sqrt() / 2.));
        let intersection = Intersection::new(2_f64.sqrt(), shape);
        let computations = intersection.prepare_computations(ray, &vec![intersection.clone()]);

        let expected = BLACK;

        let actual = world.reflected_color(&computations, 0);

        assert_eq!(expected, actual);
    }

    #[test]
    fn refracted_color_with_opaque_surface() {
        let world: World = Default::default();
        let shape = world.objects[0].clone();
        let ray = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let intersections = intersections!(Intersection::new(4., shape.clone()), 
            Intersection::new(6., shape));
        let computations = intersections[0].prepare_computations(ray, &intersections);

        let expected = BLACK;

        let actual = world.refracted_color(&computations, 5);

        assert_eq!(expected, actual);
    }

    #[test]
    fn refracted_color_at_maximum_recursive_depth() {
        let world: World = Default::default();
        let mut material: Material = Default::default();
        material.transparency = 1.;
        material.refractive_index = 1.5;
        let mut shape = world.objects[0].clone();
        shape.set_material(&material);
        let ray = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let intersections = intersections!(Intersection::new(4., shape.clone()), 
            Intersection::new(6., shape));
        let computations = intersections[0].prepare_computations(ray, &intersections);

        let expected = BLACK;

        let actual = world.refracted_color(&computations, 0);

        assert_eq!(expected, actual);
    }

    #[test]
    fn refracted_color_under_total_internal_reflection() {
        let world: World = Default::default();
        let mut material: Material = Default::default();
        material.transparency = 1.;
        material.refractive_index = 1.5;
        let mut shape = world.objects[0].clone();
        shape.set_material(&material);
        let ray = Ray::new(Tuple::point(0., 0., 2_f64.sqrt() / 2.), Tuple::vector(0., 1., 0.));
        let intersections = intersections!(Intersection::new(-2_f64.sqrt() / 2., shape.clone()),
            Intersection::new(2_f64.sqrt() / 2., shape));
        let computations = intersections[1].prepare_computations(ray, &intersections);

        let expected = BLACK;

        let actual = world.refracted_color(&computations, DEFAULT_RECURSION);

        assert_eq!(expected, actual);
    }

    #[test]
    fn refracted_color_with_refracted_ray() {
        let mut world: World = Default::default();
        let mut shape1_material: Material = Default::default();
        shape1_material.ambient = 1.;
        shape1_material.pattern = Some(Pattern::Test(TestPattern::new()));
        world.objects[0].set_material(&shape1_material);

        let mut shape2_material: Material = Default::default();
        shape2_material.transparency = 1.;
        shape2_material.refractive_index = 1.5;
        world.objects[1].set_material(&shape2_material);

        let ray = Ray::new(Tuple::point(0., 0., 0.1), Tuple::vector(0., 1., 0.));
        let intersections = intersections!(Intersection::new(-0.9899, world.objects[0].clone()),
            Intersection::new(-0.4899, world.objects[1].clone()), Intersection::new(0.4899, world.objects[1].clone()),
            Intersection::new(0.9899, world.objects[0].clone()));
        let computations = intersections[2].prepare_computations(ray, &intersections);

        let expected = Color::new(0., 0.99887, 0.04722);

        let actual = world.refracted_color(&computations, DEFAULT_RECURSION);

        assert_eq!(expected, actual);
    }

    #[test]
    fn shade_hit_with_transparent_material() {
        let mut world: World = Default::default();
        let mut floor_material: Material = Default::default();
        floor_material.transparency = 0.5;
        floor_material.refractive_index = 1.5;
        let mut floor = Shape::Plane(Plane::new());
        floor.set_transform(&translate(0., -1., 0.));
        floor.set_material(&floor_material);
        world.objects.push(floor.clone());

        let mut ball_material: Material = Default::default();
        ball_material.color = Color::new(1., 0., 0.);
        ball_material.ambient = 0.5;
        let mut ball = Shape::Sphere(Sphere::new());
        ball.set_material(&ball_material);
        ball.set_transform(&translate(0., -3.5, -0.5));
        world.objects.push(ball);

        let ray = Ray::new(Tuple::point(0., 0., -3.), Tuple::vector(0., -2_f64.sqrt() / 2., 2_f64.sqrt() / 2.));
        let intersections = intersections!(Intersection::new(2_f64.sqrt(), floor));
        let computations = intersections[0].prepare_computations(ray, &intersections);

        let expected = Color::new(0.93642, 0.68642, 0.68642);

        let actual = world.shade_hit(&computations, DEFAULT_RECURSION);

        assert_eq!(expected, actual);
    }

    #[test]
    fn shade_hit_with_reflective_transparent_material() {
        let mut world: World = Default::default();
        let mut floor_material: Material = Default::default();
        floor_material.reflective = 0.5;
        floor_material.transparency = 0.5;
        floor_material.refractive_index = 1.5;
        let mut floor = Shape::Plane(Plane::new());
        floor.set_transform(&translate(0., -1., 0.));
        floor.set_material(&floor_material);
        world.objects.push(floor.clone());

        let mut ball_material: Material = Default::default();
        ball_material.color = Color::new(1., 0., 0.);
        ball_material.ambient = 0.5;
        let mut ball = Shape::Sphere(Sphere::new());
        ball.set_material(&ball_material);
        ball.set_transform(&translate(0., -3.5, -0.5));
        world.objects.push(ball);

        let ray = Ray::new(Tuple::point(0., 0., -3.), Tuple::vector(0., -2_f64.sqrt() / 2., 2_f64.sqrt() / 2.));
        let intersections = intersections!(Intersection::new(2_f64.sqrt(), floor));
        let computations = intersections[0].prepare_computations(ray, &intersections);

        let expected = Color::new(0.93391, 0.69643, 0.69243);

        let actual = world.shade_hit(&computations, DEFAULT_RECURSION);

        assert_eq!(expected, actual);
    }

    #[test]
    fn there_is_no_shadow_when_object_is_set_not_to_cast_shadow() {
        let mut world = World::new();
        world.lights.push(Light::point_light(Tuple::point(0., 10., 0.), WHITE));
        let mut sphere = Shape::Sphere(Sphere::new());
        sphere.set_transform(&translate(0., 3., 0.));
        sphere.set_casts_shadow(false);
        world.objects.push(sphere);
        let plane = Shape::Plane(Plane::new());
        world.objects.push(plane);

        let point = ORIGIN;
        
        assert!(!world.is_shadowed(point)[0]);
    }

    //#[test]
    //fn is_shadow_tests_for_occlusion_between_two_points() {
    //    let world = World::default();
    //    let points = vec![Tuple::point(-10., -10., 10.), Tuple::point(10., 10., 10.), 
    //        Tuple::point(-20., -20., -20.), Tuple::point(-5., -5., -5.)];

    //    let expecteds = vec![false, true, false, false];

    //    for source in points.iter().zip(expecteds) {
    //        let (point, expected) = source;

    //        let actual = world.is_shadowed(point);
    //    }
    //}
}
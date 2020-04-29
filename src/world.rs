use super::BLACK;
use super::color::Color;
use super::computations::Computations;
use super::intersection::Intersection;
use super::light::Light;
use super::ray::Ray;
use super::sphere::Sphere;
use super::transformation::*;
use super::tuple::Tuple;

pub struct World {
    pub objects: Vec<Sphere>,
    pub lights: Vec<Light>,
}

impl Default for World {
    fn default() -> Self {
        let light = Light::point_light(Tuple::point(-10., 10., -10.), Color::new(1., 1., 1.));

        let mut sphere1 = Sphere::new();
        sphere1.material.color = Color::new(0.8, 1., 0.6);
        sphere1.material.diffuse = 0.7;
        sphere1.material.specular = 0.2;

        let mut sphere2 = Sphere::new();
        sphere2.transform = scale(0.5, 0.5, 0.5);

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

    pub fn shade_hit(&self, computations: Computations) -> Color {
        let mut color = BLACK;

        for light in &self.lights {
            color += computations.object.material.lighting(*light, 
                computations.point, computations.eye_vector, 
                computations.normal_vector)
        }

        color
    }

    pub fn color_at(&self, ray: Ray) -> Color {
        let intersections = self.intersect_world(ray);
        let hit = Intersection::hit(intersections);

        if hit.is_none() {
            BLACK
        } else {
            self.shade_hit(hit.unwrap().prepare_computations(ray))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::color::Color;
    use super::super::ORIGIN;
    use super::super::ray::Ray;
    use super::super::tuple::Tuple;

    #[test]
    fn creating_world() {
        let actual = World::new();

        assert!(actual.objects.is_empty());
        assert!(actual.lights.is_empty());
    }

    #[test]
    fn default_world() {
        let expected_light = Light::point_light(Tuple::point(-10., 10., -10.), Color::new(1., 1., 1.));

        let mut expected_sphere1 = Sphere::new();
        expected_sphere1.material.color = Color::new(0.8, 1., 0.6);
        expected_sphere1.material.diffuse = 0.7;
        expected_sphere1.material.specular = 0.2;

        let mut expected_sphere2 = Sphere::new();
        expected_sphere2.transform = scale(0.5, 0.5, 0.5);

        let actual: World = Default::default();

        assert_eq!(expected_light, actual.lights[0]);
        assert_eq!(expected_sphere1.material.color, actual.objects[0].material.color);
        assert_eq!(expected_sphere1.material.diffuse, actual.objects[0].material.diffuse);
        assert_eq!(expected_sphere1.material.specular, actual.objects[0].material.specular);
        assert_eq!(expected_sphere2.transform, actual.objects[1].transform);
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
        let computations = intersection.prepare_computations(ray);

        let expected = Color::new(0.38066, 0.47583, 0.2855);

        let actual = world.shade_hit(computations);

        assert_eq!(expected, actual);
    }

    #[test]
    fn shading_intersection_from_inside() {
        let mut world: World = Default::default();
        world.lights[0] = Light::point_light(Tuple::point(0., 0.25, 0.), Color::new(1., 1., 1.));
        let ray = Ray::new(ORIGIN, Tuple::vector(0., 0., 1.));
        let shape = world.objects[1].clone();
        let intersection = Intersection::new(0.5, shape);
        let computations = intersection.prepare_computations(ray);

        let expected = Color::new(0.90498, 0.90498, 0.90498);

        let actual = world.shade_hit(computations);

        assert_eq!(expected, actual);
    }

    #[test]
    fn color_when_ray_misses() {
        let world: World = Default::default();
        let ray = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 1., 0.));

        let expected = BLACK;

        let actual = world.color_at(ray);

        assert_eq!(expected, actual);
    }

    #[test]
    fn color_when_ray_hits() {
        let world: World = Default::default();
        let ray = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));

        let expected = Color::new(0.38066, 0.47583, 0.2855);

        let actual = world.color_at(ray);

        assert_eq!(expected, actual);
    }

    #[test]
    fn color_with_intersection_behind_ray() {
        let mut world: World = Default::default();
        world.objects[0].material.ambient = 1.;
        world.objects[1].material.ambient = 1.;
        let ray = Ray::new(Tuple::point(0., 0., 0.75), Tuple::vector(0., 0., -1.));

        let expected = world.objects[1].material.color;

        let actual = world.color_at(ray);

        assert_eq!(expected, actual);
    }
}
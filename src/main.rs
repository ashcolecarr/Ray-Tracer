use ray_tracer::BLACK;
use ray_tracer::camera::Camera;
use ray_tracer::canvas::Canvas;
use ray_tracer::color::Color;
use ray_tracer::Environment;
use ray_tracer::intersection::Intersection;
use ray_tracer::light::Light;
use ray_tracer::material::Material;
use ray_tracer::near_eq;
use ray_tracer::ORIGIN;
use ray_tracer::pattern::*;
use ray_tracer::plane::Plane;
use ray_tracer::Projectile;
use ray_tracer::ray::Ray;
use ray_tracer::shape::{Shape, Actionable};
use ray_tracer::sphere::Sphere;
use ray_tracer::tick;
use ray_tracer::transformation::*;
use ray_tracer::tuple::Tuple;
use ray_tracer::WHITE;
use ray_tracer::world::World;
use std::fs;
use std::f64::consts::PI;

fn main() {
    //draw_projectile();
    //draw_clock();
    //draw_circle();
    //draw_rainbow();
    //draw_dither();
    //draw_sphere_scene();
    //draw_room_scene();
    //draw_pattern();
    draw_reflective_scene();
}

pub fn draw_projectile() {
    let mut projectile = Projectile {
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: (Tuple::vector(1.0, 1.8, 0.0).normalize()) * 11.25
    };

    let environment = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0)
    };

    let mut canvas = Canvas::new(900, 550);
    let height = *canvas.get_height() as u32;

    loop {
        println!("Projectile's current position: x - {}, y - {}, z - {}, w - {}", 
            projectile.position.x, projectile.position.y,
            projectile.position.z, projectile.position.w);
        
        canvas.write_pixel(projectile.position.x as u32, 
            height - projectile.position.y as u32, Color::new(1.0, 0.5, 0.5));

        projectile = tick(environment, projectile);

        if near_eq(projectile.position.y, 0.0) || projectile.position.y < 0.0 {
            break;
        }
    }
    
    fs::write("projectile.ppm", canvas.canvas_to_ppm()).expect("File could not be written.");
}

pub fn draw_clock() {
    let mut canvas = Canvas::new(400, 400);
    let origin = ORIGIN;
    let clock_radius = (3. / 8.) * (*canvas.get_width() as f64);

    let twelve_o_clock = translate(0., 0., 1.) * origin;
    for hour in 0..12 {
        let o_clock = rotate(hour as f64 * (PI / 6.), Axis::Y) * twelve_o_clock;

        let x = (clock_radius * o_clock.x) + (*canvas.get_width() as f64 / 2.);
        let y = (clock_radius * o_clock.z) + (*canvas.get_height() as f64 / 2.);
        canvas.write_pixel(x as u32, y as u32, Color::new(1.0, 0.5, 0.5));
    }
    
    fs::write("clock.ppm", canvas.canvas_to_ppm()).expect("File could not be written.");
}

pub fn draw_circle() {
    let ray_origin = Tuple::point(0., 0., -5.);
    let wall_z = 10.;
    let wall_size = 7.;
    
    let canvas_pixels = 100;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.;
    
    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let mut color;
    let mut shape = Sphere::new();
    shape.material = Default::default();
    shape.material.color = Color::new(1., 0.2, 1.);

    let light_position = Tuple::point(-10., 10., -10.);
    let light_color = WHITE;
    let light = Light::point_light(light_position, light_color);

    for y in 0..(canvas_pixels - 1) {
        let world_y = half - pixel_size * y as f64;

        for x in 0..(canvas_pixels - 1) {
            let world_x = -half + pixel_size * x as f64;
            let position = Tuple::point(world_x, world_y, wall_z);

            let ray = Ray::new(ray_origin, (position - ray_origin).normalize());
            let intersections = shape.intersect(ray);

            let hit = Intersection::hit(intersections);
            if hit.is_some() {
                let point = ray.position(hit.clone().unwrap().t);
                let normal = hit.clone().unwrap().object.normal_at(point);
                let eye = -ray.direction;

                let material = hit.unwrap().object.get_material();
                color = material.lighting(Shape::Sphere(shape.clone()), light, point, eye, normal, false);
                canvas.write_pixel(x as u32, y as u32, color);
            }
        }
    }
    
    fs::write("circle.ppm", canvas.canvas_to_ppm()).expect("File could not be written.");
}

pub fn draw_rainbow() {
    let mut canvas = Canvas::new(700, 700);
    let red = Color::new(1., 0., 0.);
    let orange = Color::new(1., 0.5, 0.);
    let yellow = Color::new(1., 1., 0.);
    let green = Color::new(0., 1., 0.);
    let blue = Color::new(0., 0., 1.);
    let indigo = Color::new(0.294118, 0., 0.509804);
    let violet = Color::new(0.580392, 0., 0.827451);

    for y in 0..*canvas.get_height() {
        for x in 0..*canvas.get_width() {
            match y / 100 {
                0 => canvas.write_pixel(x as u32, y as u32, red),
                1 => canvas.write_pixel(x as u32, y as u32, orange),
                2 => canvas.write_pixel(x as u32, y as u32, yellow),
                3 => canvas.write_pixel(x as u32, y as u32, green),
                4 => canvas.write_pixel(x as u32, y as u32, blue),
                5 => canvas.write_pixel(x as u32, y as u32, indigo),
                6 => canvas.write_pixel(x as u32, y as u32, violet),
                _ => (),
            }
        }
    }
    
    fs::write("rainbow.ppm", canvas.canvas_to_ppm()).expect("File could not be written.");
}

pub fn draw_dither() {
    let mut canvas = Canvas::new(100, 100);
    let red = Color::new(1., 0., 0.);
    let yellow = Color::new(1., 1., 0.);

    for y in 0..*canvas.get_height() {
        for x in 0..*canvas.get_width() {
            match y % 2 {
                0 => {
                    match x % 2 {
                        0 => canvas.write_pixel(x as u32, y as u32, red),
                        1 => canvas.write_pixel(x as u32, y as u32, yellow),
                        _ => (),
                    }
                },
                1 => {
                    match x % 2 {
                        0 => canvas.write_pixel(x as u32, y as u32, yellow),
                        1 => canvas.write_pixel(x as u32, y as u32, red),
                        _ => (),
                    }
                },
                _ => (),
            }
        }
    }
    
    fs::write("dither.ppm", canvas.canvas_to_ppm()).expect("File could not be written.");
}

pub fn draw_sphere_scene() {
    let mut floor = Sphere::new();
    floor.transform = scale(10., 0.01, 10.);
    floor.material.color = Color::new(1., 0.9, 0.9);
    floor.material.specular = 0.;

    let mut left_wall = Sphere::new();
    left_wall.transform = translate(0., 0., 5.) * rotate(-PI / 4., Axis::Y) * 
        rotate(PI / 2., Axis::X) * scale(10., 0.01, 10.);
    left_wall.material = floor.material.clone();

    let mut right_wall = Sphere::new();
    right_wall.transform = translate(0., 0., 5.) * rotate(PI / 4., Axis::Y) * 
        rotate(PI / 2., Axis::X) * scale(10., 0.01, 10.);
    right_wall.material = floor.material.clone();

    let mut middle = Sphere::new();
    middle.transform = translate(-0.5, 1., 0.5);
    middle.material.color = Color::new(0.1, 1., 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = Sphere::new();
    right.transform = translate(1.5, 0.5, -0.5) * scale(0.5, 0.5, 0.5);
    right.material.color = Color::new(0.5, 1., 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = Sphere::new();
    left.transform = translate(-1.5, 0.33, -0.75) * scale(0.33, 0.33, 0.33);
    left.material.color = Color::new(1., 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let mut world = World::new();
    world.lights.push(Light::point_light(Tuple::point(-10., 10., -10.), WHITE));
    world.lights.push(Light::point_light(Tuple::point(10., 10., -10.), Color::new(0.5, 0.5, 0.5)));
    world.objects.push(Shape::Sphere(floor));
    world.objects.push(Shape::Sphere(left_wall));
    world.objects.push(Shape::Sphere(right_wall));
    world.objects.push(Shape::Sphere(middle));
    world.objects.push(Shape::Sphere(right));
    world.objects.push(Shape::Sphere(left));

    let mut camera = Camera::new(100, 50, PI / 3.);
    camera.transform = view_transform(Tuple::point(0., 1.5, -5.), Tuple::point(0., 1., 0.), Tuple::vector(0., 1., 0.));
    
    let canvas = camera.render(world);

    fs::write("spheres.ppm", canvas.canvas_to_ppm()).expect("File could not be written.");
}

pub fn draw_sphere_spiral() {
    let mut sphere1 = Sphere::new();
    sphere1.material.color = Color::new(1., 0., 0.);
    let mut sphere2 = Sphere::new();
    sphere2.transform = translate(-1.5, 0., 1.);
    sphere2.material.color = Color::new(1., 1., 0.);

    let mut world = World::new();
    world.lights.push(Light::point_light(Tuple::point(-10., 10., -10.), WHITE));
    world.objects.push(Shape::Sphere(sphere1));
    world.objects.push(Shape::Sphere(sphere2));
    
    let mut camera = Camera::new(100, 50, PI / 3.);
    camera.transform = view_transform(Tuple::point(0., 2., -0.5), Tuple::point(0., -1., 0.), Tuple::vector(0., 1., 0.));
    
    let canvas = camera.render(world);

    fs::write("sphere_spiral.ppm", canvas.canvas_to_ppm()).expect("File could not be written.");
}

pub fn draw_room_scene() {
    let mut floor_material: Material = Default::default();
    floor_material.ambient = 0.;
    floor_material.diffuse = 0.4;
    floor_material.specular = 0.;
    floor_material.shininess = 50.;
    let mut floor = Shape::Plane(Plane::new());
    floor.set_material(floor_material);

    let mut wall_material: Material = Default::default();
    wall_material.ambient = 0.1;
    wall_material.diffuse = 0.4;
    wall_material.specular = 0.;
    wall_material.shininess = 50.;
    wall_material.color = Color::new(0., 1., 1.);

    let mut north_wall = Shape::Plane(Plane::new());
    north_wall.set_material(wall_material.clone());
    north_wall.set_transform(translate(0., 0., 2.) * rotate(PI / 2., Axis::X));
    
    let mut south_wall = Shape::Plane(Plane::new());
    south_wall.set_material(wall_material.clone());
    south_wall.set_transform(translate(0., 0., -2.) * rotate(PI / 2., Axis::X));

    let mut northeast_wall = Shape::Plane(Plane::new());
    northeast_wall.set_material(wall_material.clone());
    northeast_wall.set_transform(translate(0., 0., 3.) * rotate(PI / 4., Axis::Y) * rotate(PI / 2., Axis::X));

    let mut southeast_wall = Shape::Plane(Plane::new());
    southeast_wall.set_material(wall_material.clone());
    southeast_wall.set_transform(translate(0., 0., -3.) * rotate(-PI / 4., Axis::Y) * rotate(PI / 2., Axis::X));

    let mut northwest_wall = Shape::Plane(Plane::new());
    northwest_wall.set_material(wall_material.clone());
    northwest_wall.set_transform(translate(0., 0., 3.) * rotate(-PI / 4., Axis::Y) * rotate(PI / 2., Axis::X));

    let mut southwest_wall = Shape::Plane(Plane::new());
    southwest_wall.set_material(wall_material.clone());
    southwest_wall.set_transform(translate(0., 0., -3.) * rotate(PI / 4., Axis::Y) * rotate(PI / 2., Axis::X));

    let mut sphere_material: Material = Default::default();
    sphere_material.color = Color::new(0.8, 0.4, 0.);
    let mut sphere = Shape::Sphere(Sphere::new());
    sphere.set_material(sphere_material);
    sphere.set_transform(translate(0., 1., 0.) * scale(1., 1., 1.));
    
    let mut world = World::new();
    world.lights.push(Light::point_light(Tuple::point(0., 10., 0.), WHITE));
    world.objects.push(floor);
    world.objects.push(sphere);
    world.objects.push(north_wall);
    world.objects.push(south_wall);
    world.objects.push(northeast_wall);
    world.objects.push(southeast_wall);
    world.objects.push(northwest_wall);
    world.objects.push(southwest_wall);
    
    let mut camera = Camera::new(100, 100, PI / 2.);
    camera.transform = view_transform(Tuple::point(0., 4., 0.), Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));
    
    let canvas = camera.render(world);

    fs::write("room_scene.ppm", canvas.canvas_to_ppm()).expect("File could not be written.");
}

pub fn draw_pattern() {
    //let plane_pattern = Pattern::Checkered(CheckeredPattern::new(Color::new(0.6, 0.8, 1.), Color::new(0., 0., 1.)));
    //let plane_pattern = Pattern::Ring(RingPattern::new(Color::new(0.6, 0.8, 1.), Color::new(0., 0., 1.)));
    let plane_pattern = Pattern::RingGradient(RingGradientPattern::new(Color::new(0.6, 0.8, 1.), Color::new(0., 0., 1.)));
    let mut plane_material: Material = Default::default();
    plane_material.pattern = Some(plane_pattern);
    let mut plane = Shape::Plane(Plane::new());
    plane.set_material(plane_material);

    let mut sphere_pattern = Pattern::Ring(RingPattern::new(Color::new(0.5, 0.4, 0.), Color::new(1.0, 0.8, 0.)));
    sphere_pattern.set_transform(translate(0., 0., -1.) * scale(0.125, 0.125, 0.125));
    let mut sphere_material: Material = Default::default();
    sphere_material.pattern = Some(sphere_pattern);
    let mut sphere = Shape::Sphere(Sphere::new());
    sphere.set_material(sphere_material);
    sphere.set_transform(translate(0., 3., 0.) * scale(2., 2., 2.));

    let mut world = World::new();
    world.lights.push(Light::point_light(Tuple::point(-10., 10., -10.), WHITE));
    world.objects.push(plane);
    world.objects.push(sphere);
    
    let mut camera = Camera::new(100, 100, PI / 2.);
    camera.transform = view_transform(Tuple::point(-5., 3.5, 0.), Tuple::point(0., 0., 0.), Tuple::vector(0., 1., 0.));
    
    let canvas = camera.render(world);

    fs::write("pattern.ppm", canvas.canvas_to_ppm()).expect("File could not be written.");
}

pub fn draw_reflective_scene() {
    let plane_pattern = Pattern::Checkered(CheckeredPattern::new(WHITE, BLACK));
    let mut plane_material: Material = Default::default();
    plane_material.pattern = Some(plane_pattern);
    let mut plane = Shape::Plane(Plane::new());
    plane.set_material(plane_material);

    let mut sphere_material: Material = Default::default();
    sphere_material.color = Color::new(0.2, 0.6, 1.);
    sphere_material.reflective = 0.5;
    let mut sphere = Shape::Sphere(Sphere::new());
    sphere.set_material(sphere_material);
    sphere.set_transform(translate(0., 3., 0.) * scale(2., 2., 2.));

    let mut sphere2_material: Material = Default::default();
    sphere2_material.color = Color::new(1., 0.6, 0.6);
    let mut sphere2 = Shape::Sphere(Sphere::new());
    sphere2.set_material(sphere2_material);
    sphere2.set_transform(translate(2., 2., -2.) * scale(0.7, 0.7, 0.7));

    let mut world = World::new();
    world.lights.push(Light::point_light(Tuple::point(-10., 10., -10.), WHITE));
    world.objects.push(plane);
    world.objects.push(sphere);
    world.objects.push(sphere2);
    
    let mut camera = Camera::new(100, 100, PI / 2.);
    camera.transform = view_transform(Tuple::point(0., 3.5, -5.), Tuple::point(0., 0., 0.), Tuple::vector(0., 1., 0.));
    
    let canvas = camera.render(world);

    fs::write("reflective.ppm", canvas.canvas_to_ppm()).expect("File could not be written.");
}
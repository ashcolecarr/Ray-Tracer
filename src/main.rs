use ray_tracer::canvas::Canvas;
use ray_tracer::color::Color;
use ray_tracer::Environment;
use ray_tracer::intersection::Intersection;
use ray_tracer::light::Light;
use ray_tracer::near_eq;
use ray_tracer::Projectile;
use ray_tracer::ray::Ray;
use ray_tracer::sphere::Sphere;
use ray_tracer::tick;
use ray_tracer::transformation::*;
use ray_tracer::tuple::Tuple;
use std::fs;
use std::f64::consts::PI;

fn main() {
    //draw_projectile();
    //draw_clock();
    //draw_circle();
    //draw_rainbow();
    draw_dither();
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
    let origin = Tuple::point(0., 0., 0.);
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
    let light_color = Color::new(1., 1., 1.);
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

                let material = hit.unwrap().object.material;
                //color = material.lighting(Shape::Sphere(shape.clone()), light, point, eye, normal, false);
                color = material.lighting(light, point, eye, normal);
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
    //let sky_blue = Color::new(0.52941, 0.80784, 0.92157);
    //let pale_green = Color::new(0.59608, 0.98431, 0.59608);

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
use ray_tracer::BLACK;
use ray_tracer::camera::Camera;
use ray_tracer::canvas::Canvas;
use ray_tracer::color::Color;
use ray_tracer::cone::Cone;
use ray_tracer::cube::Cube;
use ray_tracer::cylinder::Cylinder;
use ray_tracer::Environment;
use ray_tracer::hexagon;
use ray_tracer::intersection::Intersection;
use ray_tracer::light::Light;
use ray_tracer::material::Material;
use ray_tracer::near_eq;
use ray_tracer::obj_file::obj_to_group;
use ray_tracer::ORIGIN;
use ray_tracer::obj_file::parse_obj_file;
use ray_tracer::pattern::*;
use ray_tracer::plane::Plane;
use ray_tracer::Projectile;
use ray_tracer::ray::Ray;
use ray_tracer::shape::{Shape, CommonShape};
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
    //draw_reflective_scene();
    //draw_glass_ball();
    //draw_reflection_refraction();
    //draw_table_scene();
    //draw_cylinder_scene();
    //draw_cone_scene();
    //draw_hexagon();
    //render_teapot();
    //render_test_cube();
    render_cover_image();
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

            let hit = Intersection::hit(intersections.clone());
            if hit.is_some() {
                let point = ray.position(hit.clone().unwrap().t);
                let normal = hit.clone().unwrap().object.normal_at(point, intersections[0].clone());
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

pub fn draw_glass_ball() {
    let mut plane_pattern = Pattern::Checkered(CheckeredPattern::new(WHITE, BLACK));
    plane_pattern.set_transform(translate(0., 0.1, 0.));
    let plane_material = Material::new().with_pattern(plane_pattern);
    let mut plane = Shape::Plane(Plane::new());
    plane.set_material(plane_material);
    plane.set_transform(translate(0., -10.1, 0.));

    let glass = Material::new().with_diffuse(0.1).with_shininess(300.)
        .with_reflective(1.).with_refractive_index(1.52).with_transparency(1.);
    let mut glass_ball = Shape::Sphere(Sphere::glass_sphere());
    glass_ball.set_material(glass);
    glass_ball.set_casts_shadow(false);

    let air = Material::new().with_diffuse(0.1).with_shininess(300.)
        .with_reflective(1.).with_refractive_index(1.).with_transparency(1.);
    let mut air_bubble = Shape::Sphere(Sphere::new());
    air_bubble.set_material(air);
    air_bubble.set_transform(scale(0.5, 0.5, 0.5));

    let mut world = World::new();
    world.lights.push(Light::point_light(Tuple::point(20., 10., 0.), Color::new(0.7, 0.7, 0.7)));
    world.objects.push(plane);
    world.objects.push(glass_ball);
    world.objects.push(air_bubble);
    
    let mut camera = Camera::new(400, 400, PI / 3.);
    camera.transform = view_transform(Tuple::point(0., 2.5, 0.), Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));
    
    let canvas = camera.render(world);

    fs::write("glass_ball.ppm", canvas.canvas_to_ppm()).expect("File could not be written.");
}

pub fn draw_reflection_refraction() {
    let mut world = World::new();
    world.lights.push(Light::point_light(Tuple::point(-4.9, 4.9, -1.), Color::new(1., 1., 1.)));

    let floor_pattern = Pattern::Checkered(CheckeredPattern::new(Color::new(0.35, 0.35, 0.35), Color::new(0.65, 0.65, 0.65)));
    let mut floor = Shape::Plane(Plane::new());
    floor.set_transform(rotate(0.31415, Axis::Y));
    floor.set_material(Material::new().with_pattern(floor_pattern)
        .with_specular(0.).with_reflective(0.4));
    world.objects.push(floor);

    let mut ceiling = Shape::Plane(Plane::new());
    ceiling.set_transform(translate(0., 5., 0.));
    ceiling.set_material(Material::new().with_color(Color::new(0.8, 0.8, 0.8))
        .with_ambient(0.3).with_specular(0.));
    world.objects.push(ceiling);

    let mut wall_pattern = Pattern::Striped(StripedPattern::new(Color::new(0.45, 0.45, 0.45), Color::new(0.55, 0.55, 0.55)));
    wall_pattern.set_transform(rotate(1.5708, Axis::Y) * scale(0.25, 0.25, 0.25));
    let wall_material = Material::new().with_pattern(wall_pattern)
        .with_ambient(0.).with_diffuse(0.4).with_specular(0.).with_reflective(0.3);

    let mut west_wall = Shape::Plane(Plane::new());
    west_wall.set_transform(translate(-5., 0., 0.) * rotate(1.5708, Axis::Z) * rotate(1.5708, Axis::Y));
    west_wall.set_material(wall_material.clone());
    world.objects.push(west_wall);

    let mut east_wall = Shape::Plane(Plane::new());
    east_wall.set_transform(translate(5., 0., 0.) * rotate(1.5708, Axis::Z) * rotate(1.5708, Axis::Y));
    east_wall.set_material(wall_material.clone());
    world.objects.push(east_wall);

    let mut north_wall = Shape::Plane(Plane::new());
    north_wall.set_transform(translate(0., 0., 5.) * rotate(1.5708, Axis::X));
    north_wall.set_material(wall_material.clone());
    world.objects.push(north_wall);
    
    let mut south_wall = Shape::Plane(Plane::new());
    south_wall.set_transform(translate(0., 0., -5.) * rotate(1.5708, Axis::X));
    south_wall.set_material(wall_material.clone());
    world.objects.push(south_wall);

    // Background spheres
    let mut background_sphere1 = Shape::Sphere(Sphere::new());
    background_sphere1.set_transform(translate(4.6, 0.4, 1.) * scale(0.4, 0.4, 0.4));
    background_sphere1.set_material(Material::new()
        .with_color(Color::new(0.8, 0.5, 0.3)).with_shininess(50.));
    world.objects.push(background_sphere1);

    let mut background_sphere2 = Shape::Sphere(Sphere::new());
    background_sphere2.set_transform(translate(4.7, 0.3, 0.4) * scale(0.3, 0.3, 0.3));
    background_sphere2.set_material(Material::new()
        .with_color(Color::new(0.9, 0.4, 0.5)).with_shininess(50.));
    world.objects.push(background_sphere2);

    let mut background_sphere3 = Shape::Sphere(Sphere::new());
    background_sphere3.set_transform(translate(-1., 0.5, 4.5) * scale(0.5, 0.5, 0.5));
    background_sphere3.set_material(Material::new()
        .with_color(Color::new(0.4, 0.9, 0.6)).with_shininess(50.));
    world.objects.push(background_sphere3);

    let mut background_sphere4 = Shape::Sphere(Sphere::new());
    background_sphere4.set_transform(translate(-1.7, 0.3, 4.7) * scale(0.3, 0.3, 0.3));
    background_sphere4.set_material(Material::new()
        .with_color(Color::new(0.4, 0.6, 0.9)).with_shininess(50.));
    world.objects.push(background_sphere4);

    // Foreground spheres
    let mut red_sphere = Shape::Sphere(Sphere::new());
    red_sphere.set_transform(translate(-0.6, 1., 0.6));
    red_sphere.set_material(Material::new().with_color(Color::new(1., 0.3, 0.2))
        .with_specular(0.4).with_shininess(5.));
    world.objects.push(red_sphere);

    let mut blue_glass_sphere = Shape::Sphere(Sphere::new());
    blue_glass_sphere.set_transform(translate(0.6, 0.7, -0.6) * scale(0.7, 0.7, 0.7));
    blue_glass_sphere.set_material(Material::new().with_color(Color::new(0., 0., 0.2))
        .with_ambient(0.).with_diffuse(0.4).with_specular(0.9)
        .with_shininess(300.).with_reflective(0.9)
        .with_transparency(0.9).with_refractive_index(1.5));
    world.objects.push(blue_glass_sphere);

    let mut green_glass_sphere = Shape::Sphere(Sphere::new());
    green_glass_sphere.set_transform(translate(-0.7, 0.5, -0.8) * scale(0.5, 0.5, 0.5));
    green_glass_sphere.set_material(Material::new().with_color(Color::new(0., 0.2, 0.))
        .with_ambient(0.).with_diffuse(0.4).with_specular(0.9)
        .with_shininess(300.).with_reflective(0.9)
        .with_transparency(0.9).with_refractive_index(1.5));
    world.objects.push(green_glass_sphere);

    let mut camera = Camera::new(400, 200, 1.152);
    camera.transform = view_transform(Tuple::point(-2.6, 1.5, -3.9), Tuple::point(-0.6, 1., -0.8), Tuple::vector(0., 1., 0.));
    
    let canvas = camera.render(world);

    fs::write("reflection_refraction.ppm", canvas.canvas_to_ppm()).expect("File could not be written.");
}

pub fn draw_table_scene() {
    let mut world = World::new();
    world.lights.push(Light::point_light(Tuple::point(0., 6.9, -5.), Color::new(1., 1., 0.9)));

    let mut floors_pattern = Pattern::Checkered(
        CheckeredPattern::new(BLACK, Color::new(0.25, 0.25, 0.25)));
    floors_pattern.set_transform(scale(0.07, 0.07, 0.07));
    let mut floors = Shape::Cube(Cube::new());
    floors.set_transform(scale(20., 7., 20.) * translate(0., 1., 0.));
    floors.set_material(Material::new().with_pattern(floors_pattern)
        .with_ambient(0.25).with_diffuse(0.7).with_specular(0.9)
        .with_shininess(300.).with_reflective(0.1));
    world.objects.push(floors);

    let mut walls_pattern = Pattern::Checkered(
        CheckeredPattern::new(Color::new(0.4863, 0.3765, 0.2941), Color::new(0.3725, 0.2902, 0.2275)));
    walls_pattern.set_transform(scale(0.05, 20., 0.05));
    let mut walls = Shape::Cube(Cube::new());
    walls.set_transform(scale(10., 10., 10.));
    walls.set_material(Material::new().with_pattern(walls_pattern)
        .with_ambient(0.1).with_diffuse(0.7).with_specular(0.9)
        .with_shininess(300.).with_reflective(0.1));
    world.objects.push(walls);

    let mut table_pattern = Pattern::Striped(
        StripedPattern::new(Color::new(0.5529, 0.4235, 0.3255), Color::new(0.6588, 0.5098, 0.4)));
    table_pattern.set_transform(scale(0.05, 0.05, 0.05) * rotate(0.1, Axis::Y));
    let mut table_top = Shape::Cube(Cube::new());
    table_top.set_transform(translate(0., 3.1, 0.) * scale(3., 0.1, 2.));
    table_top.set_material(Material::new().with_pattern(table_pattern)
        .with_ambient(0.1).with_diffuse(0.7).with_specular(0.9)
        .with_shininess(300.).with_reflective(0.2));
    world.objects.push(table_top);

    let mut table_leg1 = Shape::Cube(Cube::new());
    table_leg1.set_transform(translate(2.7, 1.5, -1.7) * scale(0.1, 1.5, 0.1));
    table_leg1.set_material(Material::new()
        .with_color(Color::new(0.5529, 0.4235, 0.3255))
        .with_ambient(0.2).with_diffuse(0.7));
    world.objects.push(table_leg1);

    let mut table_leg2 = Shape::Cube(Cube::new());
    table_leg2.set_transform(translate(2.7, 1.5, 1.7) * scale(0.1, 1.5, 0.1));
    table_leg2.set_material(Material::new()
        .with_color(Color::new(0.5529, 0.4235, 0.3255))
        .with_ambient(0.2).with_diffuse(0.7));
    world.objects.push(table_leg2);

    let mut table_leg3 = Shape::Cube(Cube::new());
    table_leg3.set_transform(translate(-2.7, 1.5, -1.7) * scale(0.1, 1.5, 0.1));
    table_leg3.set_material(Material::new()
        .with_color(Color::new(0.5529, 0.4235, 0.3255))
        .with_ambient(0.2).with_diffuse(0.7));
    world.objects.push(table_leg3);

    let mut table_leg4 = Shape::Cube(Cube::new());
    table_leg4.set_transform(translate(-2.7, 1.5, 1.7) * scale(0.1, 1.5, 0.1));
    table_leg4.set_material(Material::new()
        .with_color(Color::new(0.5529, 0.4235, 0.3255))
        .with_ambient(0.2).with_diffuse(0.7));
    world.objects.push(table_leg4);
    
    let mut glass_cube = Shape::Cube(Cube::new());
    glass_cube.set_transform(translate(0., 3.45001, 0.) * rotate(0.2, Axis::Y) * scale(0.25, 0.25, 0.25));
    glass_cube.set_casts_shadow(false);
    glass_cube.set_material(Material::new().with_color(Color::new(1., 1., 0.8))
        .with_ambient(0.).with_diffuse(0.3).with_specular(0.9)
        .with_shininess(300.).with_reflective(0.7).with_transparency(0.7)
        .with_refractive_index(1.5));
    world.objects.push(glass_cube);

    let mut little_cube1 = Shape::Cube(Cube::new());
    little_cube1.set_transform(translate(1., 3.35, -0.9) * rotate(-0.4, Axis::Y) * scale(0.15, 0.15, 0.15));
    little_cube1.set_material(Material::new().with_color(Color::new(1., 0.5, 0.5))
        .with_reflective(0.6).with_diffuse(0.3));
    world.objects.push(little_cube1);

    let mut little_cube2 = Shape::Cube(Cube::new());
    little_cube2.set_transform(translate(-1.5, 3.27, 0.3) * rotate(0.4, Axis::Y) * scale(0.15, 0.07, 0.15));
    little_cube2.set_material(Material::new().with_color(Color::new(1., 1., 0.5)));
    world.objects.push(little_cube2);

    let mut little_cube3 = Shape::Cube(Cube::new());
    little_cube3.set_transform(translate(0., 3.25, 1.) * rotate(0.4, Axis::Y) * scale(0.2, 0.05, 0.05));
    little_cube3.set_material(Material::new().with_color(Color::new(0.5, 1., 0.5)));
    world.objects.push(little_cube3);

    let mut little_cube4 = Shape::Cube(Cube::new());
    little_cube4.set_transform(translate(-0.6, 3.4, -1.) * rotate(0.8, Axis::Y) * scale(0.05, 0.2, 0.05));
    little_cube4.set_material(Material::new().with_color(Color::new(0.5, 0.5, 1.)));
    world.objects.push(little_cube4);

    let mut little_cube5 = Shape::Cube(Cube::new());
    little_cube5.set_transform(translate(2., 3.4, 1.) * rotate(0.8, Axis::Y) * scale(0.05, 0.2, 0.05));
    little_cube5.set_material(Material::new().with_color(Color::new(0.5, 1., 1.)));
    world.objects.push(little_cube5);

    let mut frame1 = Shape::Cube(Cube::new());
    frame1.set_transform(translate(-10., 4., 1.) * scale(0.05, 1., 1.));
    frame1.set_material(Material::new().with_color(Color::new(0.7098, 0.2471, 0.2196))
        .with_diffuse(0.6));
    world.objects.push(frame1);

    let mut frame2 = Shape::Cube(Cube::new());
    frame2.set_transform(translate(-10., 3.4, 2.7) * scale(0.05, 0.4, 0.4));
    frame2.set_material(Material::new().with_color(Color::new(0.2667, 0.2706, 0.6902))
        .with_diffuse(0.6));
    world.objects.push(frame2);

    let mut frame3 = Shape::Cube(Cube::new());
    frame3.set_transform(translate(-10., 4.6, 2.7) * scale(0.05, 0.4, 0.4));
    frame3.set_material(Material::new().with_color(Color::new(0.3098, 0.5961, 0.3098))
        .with_diffuse(0.6));
    world.objects.push(frame3);

    let mut mirror_frame = Shape::Cube(Cube::new());
    mirror_frame.set_transform(translate(-2., 3.5, 9.95) * scale(5., 1.5, 0.05));
    mirror_frame.set_material(Material::new().with_color(Color::new(0.3882, 0.2627, 0.1882))
        .with_diffuse(0.7));
    world.objects.push(mirror_frame);

    let mut mirror = Shape::Cube(Cube::new());
    mirror.set_transform(translate(-2., 3.5, 9.95) * scale(4.8, 1.4, 0.06));
    mirror.set_material(Material::new().with_color(BLACK).with_diffuse(0.)
        .with_ambient(0.).with_specular(1.).with_shininess(300.).with_reflective(1.));
    world.objects.push(mirror);

    let mut camera = Camera::new(400, 200, 0.785);
    camera.transform = view_transform(Tuple::point(8., 6., -8.), Tuple::point(0., 3., 0.), Tuple::vector(0., 1., 0.));
    
    let canvas = camera.render(world);

    fs::write("table_scene.ppm", canvas.canvas_to_ppm()).expect("File could not be written.");
}

pub fn draw_cylinder_scene() {
    let mut world = World::new();
    world.lights.push(Light::point_light(Tuple::point(1., 6.9, -4.9), Color::new(1., 1., 1.)));

    let mut floor_pattern = Pattern::Checkered(
        CheckeredPattern::new(Color::new(0.5, 0.5, 0.5), Color::new(0.75, 0.75, 0.75)));
    floor_pattern.set_transform(rotate(0.3, Axis::Y) * scale(0.25, 0.25, 0.25));
    let mut floor = Shape::Plane(Plane::new());
    floor.set_material(Material::new().with_pattern(floor_pattern)
        .with_ambient(0.2).with_diffuse(0.9).with_specular(0.));
    world.objects.push(floor);

    let mut cylinder1 = Shape::Cylinder(Cylinder::new());
    cylinder1.set_minimum(0.);
    cylinder1.set_maximum(0.75);
    cylinder1.set_closed(true);
    cylinder1.set_transform(translate(-1., 0., 1.) * scale(0.5, 1., 0.5));
    cylinder1.set_material(Material::new().with_color(Color::new(0., 0., 0.6))
        .with_diffuse(0.1).with_specular(0.9)
        .with_shininess(300.).with_reflective(0.9));
    world.objects.push(cylinder1);

    // Concentric cylinders
    let mut cylinder2 = Shape::Cylinder(Cylinder::new());
    cylinder2.set_minimum(0.);
    cylinder2.set_maximum(0.2);
    cylinder2.set_closed(false);
    cylinder2.set_transform(translate(1., 0., 0.) * scale(0.8, 1., 0.8));
    cylinder2.set_material(Material::new().with_color(Color::new(1., 1., 0.3))
        .with_ambient(0.1).with_diffuse(0.8)
        .with_specular(0.9).with_shininess(300.));
    world.objects.push(cylinder2);

    let mut cylinder3 = Shape::Cylinder(Cylinder::new());
    cylinder3.set_minimum(0.);
    cylinder3.set_maximum(0.3);
    cylinder3.set_closed(false);
    cylinder3.set_transform(translate(1., 0., 0.) * scale(0.6, 1., 0.6));
    cylinder3.set_material(Material::new().with_color(Color::new(1., 0.9, 0.4))
        .with_ambient(0.1).with_diffuse(0.8)
        .with_specular(0.9).with_shininess(300.));
    world.objects.push(cylinder3);

    let mut cylinder4 = Shape::Cylinder(Cylinder::new());
    cylinder4.set_minimum(0.);
    cylinder4.set_maximum(0.4);
    cylinder4.set_closed(false);
    cylinder4.set_transform(translate(1., 0., 0.) * scale(0.4, 1., 0.4));
    cylinder4.set_material(Material::new().with_color(Color::new(1., 0.8, 0.5))
        .with_ambient(0.1).with_diffuse(0.8)
        .with_specular(0.9).with_shininess(300.));
    world.objects.push(cylinder4);

    let mut cylinder5 = Shape::Cylinder(Cylinder::new());
    cylinder5.set_minimum(0.);
    cylinder5.set_maximum(0.5);
    cylinder5.set_closed(true);
    cylinder5.set_transform(translate(1., 0., 0.) * scale(0.2, 1., 0.2));
    cylinder5.set_material(Material::new().with_color(Color::new(1., 0.7, 0.6))
        .with_ambient(0.1).with_diffuse(0.8)
        .with_specular(0.9).with_shininess(300.));
    world.objects.push(cylinder5);

    // Decorative cylinders
    let mut cylinder6 = Shape::Cylinder(Cylinder::new());
    cylinder6.set_minimum(0.);
    cylinder6.set_maximum(0.3);
    cylinder6.set_closed(true);
    cylinder6.set_transform(translate(0., 0., -0.75) * scale(0.05, 1., 0.05));
    cylinder6.set_material(Material::new().with_color(Color::new(1., 0., 0.))
        .with_ambient(0.1).with_diffuse(0.9)
        .with_specular(0.9).with_shininess(300.));
    world.objects.push(cylinder6);

    let mut cylinder7 = Shape::Cylinder(Cylinder::new());
    cylinder7.set_minimum(0.);
    cylinder7.set_maximum(0.3);
    cylinder7.set_closed(true);
    cylinder7.set_transform(translate(0., 0., -2.25) * rotate(-0.15, Axis::Y) *
        translate(0., 0., 1.5) * scale(0.05, 1., 0.05));
    cylinder7.set_material(Material::new().with_color(Color::new(1., 1., 0.))
        .with_ambient(0.1).with_diffuse(0.9)
        .with_specular(0.9).with_shininess(300.));
    world.objects.push(cylinder7);

    let mut cylinder8 = Shape::Cylinder(Cylinder::new());
    cylinder8.set_minimum(0.);
    cylinder8.set_maximum(0.3);
    cylinder8.set_closed(true);
    cylinder8.set_transform(translate(0., 0., -2.25) * rotate(-0.3, Axis::Y) *
        translate(0., 0., 1.5) * scale(0.05, 1., 0.05));
    cylinder8.set_material(Material::new().with_color(Color::new(0., 1., 0.))
        .with_ambient(0.1).with_diffuse(0.9)
        .with_specular(0.9).with_shininess(300.));
    world.objects.push(cylinder8);

    let mut cylinder9 = Shape::Cylinder(Cylinder::new());
    cylinder9.set_minimum(0.);
    cylinder9.set_maximum(0.3);
    cylinder9.set_closed(true);
    cylinder9.set_transform(translate(0., 0., -2.25) * rotate(-0.45, Axis::Y) *
        translate(0., 0., 1.5) * scale(0.05, 1., 0.05));
    cylinder9.set_material(Material::new().with_color(Color::new(0., 1., 1.))
        .with_ambient(0.1).with_diffuse(0.9)
        .with_specular(0.9).with_shininess(300.));
    world.objects.push(cylinder9);

    // Glass Cylinder
    let mut cylinder10 = Shape::Cylinder(Cylinder::new());
    cylinder10.set_minimum(0.0001);
    cylinder10.set_maximum(0.5);
    cylinder10.set_closed(true);
    cylinder10.set_transform(translate(0., 0., -1.5) * scale(0.33, 1., 0.33));
    cylinder10.set_material(Material::new().with_color(Color::new(0.25, 0., 0.))
        .with_diffuse(0.1).with_specular(0.9)
        .with_shininess(300.).with_reflective(0.9)
        .with_transparency(0.9).with_refractive_index(1.5));
    world.objects.push(cylinder10);

    let mut camera = Camera::new(400, 200, 0.314);
    camera.transform = view_transform(Tuple::point(8., 3.5, -9.), Tuple::point(0., 0.3, 0.), Tuple::vector(0., 1., 0.));
    
    let canvas = camera.render(world);

    fs::write("cylinder_scene.ppm", canvas.canvas_to_ppm()).expect("File could not be written.");
}

pub fn draw_cone_scene() {
    let mut world = World::new();
    world.lights.push(Light::point_light(Tuple::point(10., 10., 10.), Color::new(1., 1., 1.)));

    let mut cone = Shape::Cone(Cone::new());
    //cone.set_minimum(0.);
    cone.set_maximum(3.);
    cone.set_closed(true);
    //cone.set_transform(translate(-1., 0., 1.) * scale(0.5, 1., 0.5));
    cone.set_material(Material::new().with_color(Color::new(1., 0., 1.)));
    world.objects.push(cone);

    let mut camera = Camera::new(100, 100, PI / 3.);
    camera.transform = view_transform(Tuple::point(4., 5., -4.), Tuple::point(0., 0.3, 0.), Tuple::vector(0., 1., 0.));
    
    let canvas = camera.render(world);

    fs::write("cone_scene.ppm", canvas.canvas_to_ppm()).expect("File could not be written.");
}

pub fn draw_hexagon() {
    let mut world = World::new();
    world.lights.push(Light::point_light(Tuple::point(0., 10., 0.), Color::new(1., 1., 1.)));

    let mut hexagon = hexagon();
    hexagon.set_material(Material::new().with_color(Color::new(1., 0., 0.)));
    world.objects.push(hexagon);

    let mut camera = Camera::new(100, 100, PI / 6.);
    camera.transform = view_transform(Tuple::point(0., 3., -3.), Tuple::point(0., 0.3, 0.), Tuple::vector(0., 1., 0.));
    
    let canvas = camera.render(world);

    fs::write("hexagon.ppm", canvas.canvas_to_ppm()).expect("File could not be written.");
}

pub fn render_teapot() {
    let mut world = World::new();
    world.lights.push(Light::point_light(Tuple::point(0., 10., 0.), Color::new(1., 1., 1.)));

    let file = "teapot-low.obj";
    let file_data =  fs::read_to_string(file);
    let parser = parse_obj_file(file_data.unwrap());

    let teapot = obj_to_group(parser);
    //teapot.set_material(Material::new().with_color(Color::new(1., 0., 0.)));
    //teapot.divide(1);
    world.objects.push(teapot);

    let mut camera = Camera::new(50, 50, PI / 3.);
    camera.transform = view_transform(Tuple::point(0., 3., -3.), Tuple::point(0., 0.3, 0.), Tuple::vector(0., 1., 0.));
    
    let canvas = camera.render(world);

    fs::write("teapot.ppm", canvas.canvas_to_ppm()).expect("File could not be written.");
}

pub fn render_test_cube() {
    let mut world = World::new();
    world.lights.push(Light::point_light(Tuple::point(0., 10., 0.), Color::new(1., 1., 1.)));

    let file = "test.obj";
    let file_data =  fs::read_to_string(file);
    let parser = parse_obj_file(file_data.unwrap());

    let test_cube = obj_to_group(parser);
    let mut sphere = Shape::Sphere(Sphere::new());
    sphere.set_transform(translate(0., 2., 0.) * scale(0.5, 0.5, 0.5));
    world.objects.push(test_cube);
    world.objects.push(sphere);

    let mut camera = Camera::new(400, 400, PI / 4.);
    camera.transform = view_transform(Tuple::point(0., 3., -3.), Tuple::point(0., 0., 0.), Tuple::vector(0., 1., 0.));
    
    let canvas = camera.render(world);

    fs::write("test_cube.ppm", canvas.canvas_to_ppm()).expect("File could not be written.");
}

pub fn render_cover_image() {
    let white_material = Material::new().with_color(WHITE).with_diffuse(0.7)
        .with_ambient(0.1).with_specular(0.).with_reflective(0.1);
    let blue_material = Material::new().with_color(Color::new(0.537, 0.831, 0.914))
        .with_diffuse(0.7).with_ambient(0.1).with_specular(0.).with_reflective(0.1);
    let red_material = Material::new().with_color(Color::new(0.941, 0.322, 0.388))
        .with_diffuse(0.7).with_ambient(0.1).with_specular(0.).with_reflective(0.1);
    let purple_material = Material::new().with_color(Color::new(0.373, 0.404, 0.55))
        .with_diffuse(0.7).with_ambient(0.1).with_specular(0.).with_reflective(0.1);

    let standard_transform = scale(0.5, 0.5, 0.5) * translate(1., -1., 1.);
    let large_object = scale(3.5, 3.5, 3.5) * standard_transform.clone();
    let medium_object = scale(3., 3., 3.) * standard_transform.clone();
    let small_object = scale(2., 2., 2.) * standard_transform;
    
    let mut world = World::new();
    world.lights.push(Light::point_light(Tuple::point(50., 100., -50.), Color::new(1., 1., 1.)));
    world.lights.push(Light::point_light(Tuple::point(-400., 50., -10.), Color::new(0.2, 0.2, 0.2)));

    let mut plane = Shape::Plane(Plane::new());
    let plane_material = Material::new().with_color(WHITE).with_ambient(1.)
        .with_diffuse(0.).with_specular(0.);
    let plane_transform = translate(0., 0., 500.) * rotate(1.5707963267948966, Axis::X);
    plane.set_material(plane_material);
    plane.set_transform(plane_transform);
    world.objects.push(plane);

    let mut sphere = Shape::Sphere(Sphere::new());
    let sphere_material = Material::new().with_color(Color::new(0.373, 0.404, 0.55))
        .with_diffuse(0.2).with_ambient(0.).with_specular(1.).with_shininess(200.)
        .with_reflective(0.7).with_transparency(0.7).with_refractive_index(1.5);
    sphere.set_material(sphere_material);
    sphere.set_transform(large_object.clone());
    world.objects.push(sphere);

    let mut white_cube1 = Shape::Cube(Cube::new());
    white_cube1.set_material(white_material.clone());
    white_cube1.set_transform(translate(4., 0., 0.) * medium_object.clone());
    world.objects.push(white_cube1);

    let mut blue_cube1 = Shape::Cube(Cube::new());
    blue_cube1.set_material(blue_material.clone());
    blue_cube1.set_transform(translate(8.5, 1.5, -0.5) * large_object.clone());
    world.objects.push(blue_cube1);

    let mut red_cube1 = Shape::Cube(Cube::new());
    red_cube1.set_material(red_material.clone());
    red_cube1.set_transform(translate(0., 0., 4.) * large_object.clone());
    world.objects.push(red_cube1);

    let mut white_cube2 = Shape::Cube(Cube::new());
    white_cube2.set_material(white_material.clone());
    white_cube2.set_transform(translate(4., 0., 4.) * small_object.clone());
    world.objects.push(white_cube2);

    let mut purple_cube1 = Shape::Cube(Cube::new());
    purple_cube1.set_material(purple_material.clone());
    purple_cube1.set_transform(translate(7.5, 0.5, 4.) * medium_object.clone());
    world.objects.push(purple_cube1);

    let mut white_cube3 = Shape::Cube(Cube::new());
    white_cube3.set_material(white_material.clone());
    white_cube3.set_transform(translate(-0.25, 0.25, 8.) * medium_object.clone());
    world.objects.push(white_cube3);

    let mut blue_cube2 = Shape::Cube(Cube::new());
    blue_cube2.set_material(blue_material.clone());
    blue_cube2.set_transform(translate(4., 1., 7.5) * large_object.clone());
    world.objects.push(blue_cube2);

    let mut red_cube2 = Shape::Cube(Cube::new());
    red_cube2.set_material(red_material.clone());
    red_cube2.set_transform(translate(10., 2., 7.5) * medium_object.clone());
    world.objects.push(red_cube2);

    let mut white_cube4 = Shape::Cube(Cube::new());
    white_cube4.set_material(white_material.clone());
    white_cube4.set_transform(translate(8., 2., 12.) * small_object.clone());
    world.objects.push(white_cube4);

    let mut white_cube5 = Shape::Cube(Cube::new());
    white_cube5.set_material(white_material.clone());
    white_cube5.set_transform(translate(20., 1., 9.) * small_object.clone());
    world.objects.push(white_cube5);

    let mut blue_cube3 = Shape::Cube(Cube::new());
    blue_cube3.set_material(blue_material.clone());
    blue_cube3.set_transform(translate(-0.5, -5., 0.25) * large_object.clone());
    world.objects.push(blue_cube3);

    let mut red_cube3 = Shape::Cube(Cube::new());
    red_cube3.set_material(red_material.clone());
    red_cube3.set_transform(translate(4., -4., 0.) * large_object.clone());
    world.objects.push(red_cube3);

    let mut white_cube6 = Shape::Cube(Cube::new());
    white_cube6.set_material(white_material.clone());
    white_cube6.set_transform(translate(8.5, -4., 0.) * large_object.clone());
    world.objects.push(white_cube6);

    let mut white_cube7 = Shape::Cube(Cube::new());
    white_cube7.set_material(white_material.clone());
    white_cube7.set_transform(translate(0., -4., 4.) * large_object.clone());
    world.objects.push(white_cube7);

    let mut purple_cube2 = Shape::Cube(Cube::new());
    purple_cube2.set_material(purple_material.clone());
    purple_cube2.set_transform(translate(-0.5, -4.5, 8.) * large_object.clone());
    world.objects.push(purple_cube2);

    let mut white_cube8 = Shape::Cube(Cube::new());
    white_cube8.set_material(white_material.clone());
    white_cube8.set_transform(translate(0., -8., 4.) * large_object.clone());
    world.objects.push(white_cube8);

    let mut white_cube9 = Shape::Cube(Cube::new());
    white_cube9.set_material(white_material.clone());
    white_cube9.set_transform(translate(-0.5, -8.5, 8.) * large_object.clone());
    world.objects.push(white_cube9);

    let mut camera = Camera::new(500, 500, 0.785);
    camera.transform = view_transform(Tuple::point(-6., 6., -10.), Tuple::point(6., 0., 6.), Tuple::vector(-0.45, 1., 0.));
    
    let canvas = camera.render(world);

    fs::write("cover_image.ppm", canvas.canvas_to_ppm()).expect("File could not be written.");
}
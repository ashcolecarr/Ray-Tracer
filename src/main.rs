use ray_tracer::canvas::Canvas;
use ray_tracer::color::Color;
use ray_tracer::Environment;
use ray_tracer::near_eq;
use ray_tracer::Projectile;
use ray_tracer::tick;
use ray_tracer::tuple::Tuple;
use std::fs;

fn main() {
    draw_projectile();
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
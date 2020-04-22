use ray_tracer::Environment;
use ray_tracer::near_eq;
use ray_tracer::Projectile;
use ray_tracer::tick;
use ray_tracer::tuple::Tuple;

fn main() {
    let mut projectile = Projectile { 
        position: Tuple::point(0., 1., 0.), 
        velocity: Tuple::vector(1., 1., 0.).normalize()
    };
    let environment = Environment {
        gravity: Tuple::vector(0., -0.1, 0.),
        wind: Tuple::vector(-0.01, 0., 0.)
    };

    loop {
        println!("Projectile's current position: x - {}, y - {}, z - {}, w - {}",
            projectile.position.x, projectile.position.y,
            projectile.position.z, projectile.position.w);

        projectile = tick(environment, projectile);

        if near_eq(projectile.position.y, 0.) || projectile.position.y < 0. {
            break;
        }
    }
}

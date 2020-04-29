use super::canvas::Canvas;
use super::matrix::Matrix;
use super::near_eq;
use super::ORIGIN;
use super::ray::Ray;
use super::tuple::Tuple;
use super::world::World;

pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub field_of_view: f64,
    pub transform: Matrix,
    pub half_width: f64,
    pub half_height: f64,
    pub pixel_size: f64,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Self {
        let half_view = (field_of_view / 2.).tan();
        let aspect = hsize as f64 / vsize as f64;

        let (half_width, half_height) = if aspect > 1. || near_eq(aspect, 1.) {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };

        let pixel_size = (half_width * 2.) / hsize as f64;

        Camera { hsize, vsize, field_of_view, transform: Matrix::identity(4),
            half_width, half_height, pixel_size }
    }

    pub fn ray_for_pixel(&self, px: u32, py: u32) -> Ray {
        let x_offset = (px as f64 + 0.5) * self.pixel_size;
        let y_offset = (py as f64 + 0.5) * self.pixel_size;

        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        let pixel = self.transform.inverse().unwrap() * Tuple::point(world_x, world_y, -1.);
        let origin = self.transform.inverse().unwrap() * ORIGIN;
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }

    pub fn render(&self, world: World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);

        for y in 0..self.vsize - 1 {
            for x in 0..self.hsize - 1 {
                let ray = self.ray_for_pixel(x as u32, y as u32);
                let color = world.color_at(ray);
                image.write_pixel(x as u32, y as u32, color);
            }
        }

        image
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::color::Color;
    use super::super::matrix::Matrix;
    use super::super::ray::Ray;
    use super::super::transformation::*;
    use super::super::tuple::Tuple;
    use super::super::world::World;
    use std::f64::consts::PI;

    #[test]
    fn constructing_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = PI / 2.;

        let expected_hsize = hsize;
        let expected_vsize = vsize;
        let expected_field_of_view = field_of_view;
        let expected_transform = Matrix::identity(4);

        let actual = Camera::new(hsize, vsize, field_of_view);

        assert_eq!(expected_hsize, actual.hsize);
        assert_eq!(expected_vsize, actual.vsize);
        assert_eq!(expected_field_of_view, actual.field_of_view);
        assert_eq!(expected_transform, actual.transform);
    }

    #[test]
    fn pixel_size_for_horizontal_canvas() {
        let camera = Camera::new(200, 125, PI / 2.);

        let expected = 0.01;

        let actual = camera.pixel_size;

        assert!(near_eq(expected, actual));
    }

    #[test]
    fn pixel_size_for_vertical_canvas() {
        let camera = Camera::new(125, 200, PI / 2.);

        let expected = 0.01;

        let actual = camera.pixel_size;

        assert!(near_eq(expected, actual));
    }

    #[test]
    fn constructing_ray_through_center_of_canvas() {
        let camera = Camera::new(201, 101, PI / 2.);
        
        let expected = Ray::new(ORIGIN, Tuple::vector(0., 0., -1.));
        
        let actual = camera.ray_for_pixel(100, 50);
        
        assert_eq!(expected, actual);
    }

    #[test]
    fn constructing_ray_through_corner_of_canvas() {
        let camera = Camera::new(201, 101, PI / 2.);
        
        let expected = Ray::new(ORIGIN, Tuple::vector(0.66519, 0.33259, -0.66851));
        
        let actual = camera.ray_for_pixel(0, 0);
        
        assert_eq!(expected, actual);
    }

    #[test]
    fn constructing_ray_when_camera_is_transformed() {
        let mut camera = Camera::new(201, 101, PI / 2.);
        camera.transform = rotate(PI / 4., Axis::Y) * translate(0., -2., 5.);
        
        let expected = Ray::new(Tuple::point(0., 2., -5.), Tuple::vector(2_f64.sqrt() / 2., 0., -2_f64.sqrt() / 2.));
        
        let actual = camera.ray_for_pixel(100, 50);
        
        assert_eq!(expected, actual);
    }

    #[test]
    fn rendering_world_with_camera() {
        let world: World = Default::default();
        let mut camera = Camera::new(11, 11, PI / 2.);
        let from = Tuple::point(0., 0., -5.);
        let to = ORIGIN;
        let up = Tuple::vector(0., 1., 0.);
        camera.transform = view_transform(from, to, up);
        let image = camera.render(world);

        let expected = Color::new(0.38066, 0.47583, 0.2855);

        let actual = image.pixel_at(5, 5);

        assert_eq!(expected, actual);
    }
}
use super::matrix::Matrix;
use super::near_eq;

pub struct Camera {
    pub hsize: u32,
    pub vsize: u32,
    pub field_of_view: f64,
    pub transform: Matrix,
    pub half_width: f64,
    pub half_height: f64,
    pub pixel_size: f64,
}

impl Camera {
    pub fn new(hsize: u32, vsize: u32, field_of_view: f64) -> Self {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::matrix::Matrix;
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
}
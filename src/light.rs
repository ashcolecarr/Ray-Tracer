use super::color::Color;
use super::tuple::Tuple;

#[derive(Debug, Copy, Clone)]
pub struct Light {
    pub position: Tuple,
    pub intensity: Color,
}

impl Light {
    pub fn point_light(position: Tuple, intensity: Color) -> Self {
        Light { position, intensity }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::color::Color;
    use super::super::tuple::Tuple;

    #[test]
    fn point_light_has_position_and_intensity() {
        let intensity = Color::new(1., 1., 1.);
        let position = Tuple::point(0., 0., 0.);

        let expected_position = position;
        let expected_intensity = intensity;

        let actual = Light::point_light(position, intensity);

        assert_eq!(expected_position, actual.position);
        assert_eq!(expected_intensity, actual.intensity);
    }
}
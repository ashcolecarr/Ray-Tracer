use super::color::Color;
use super::tuple::Tuple;

#[derive(Debug, Copy, Clone)]
pub struct Light {
    pub position: Tuple,
    pub intensity: Color,
}

impl PartialEq for Light {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.intensity == other.intensity
    }
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
    use super::super::ORIGIN;

    #[test]
    fn point_light_has_position_and_intensity() {
        let intensity = Color::new(1., 1., 1.);
        let position = ORIGIN;

        let expected_position = position;
        let expected_intensity = intensity;

        let actual = Light::point_light(position, intensity);

        assert_eq!(expected_position, actual.position);
        assert_eq!(expected_intensity, actual.intensity);
    }
}
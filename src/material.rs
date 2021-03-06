use super::BLACK;
use super::color::Color;
use super::light::Light;
use super::near_eq;
use super::pattern::{Pattern, PatternTrait};
use super::shape::Shape;
use super::tuple::Tuple;
use super::WHITE;

#[derive(Debug, Clone)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub pattern: Option<Pattern>,
    pub reflective: f64,
    pub transparency: f64,
    pub refractive_index: f64,
}

impl Default for Material {
    fn default() -> Self {
        Material {
            color: WHITE,
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            pattern: None,
            reflective: 0.,
            transparency: 0.,
            refractive_index: 1.,
        }
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color && near_eq(self.ambient, other.ambient) &&
            near_eq(self.diffuse, other.diffuse) && near_eq(self.specular, other.specular) &&
            near_eq(self.shininess, other.shininess) && self.pattern == other.pattern
    }
}

impl Material {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn with_ambient(mut self, ambient: f64) -> Self {
        self.ambient = ambient;
        self
    }

    pub fn with_diffuse(mut self, diffuse: f64) -> Self {
        self.diffuse = diffuse;
        self
    }

    pub fn with_specular(mut self, specular: f64) -> Self {
        self.specular = specular;
        self
    }

    pub fn with_shininess(mut self, shininess: f64) -> Self {
        self.shininess = shininess;
        self
    }

    pub fn with_pattern(mut self, pattern: Pattern) -> Self {
        self.pattern = Some(pattern);
        self
    }

    pub fn with_reflective(mut self, reflective: f64) -> Self {
        self.reflective = reflective;
        self
    }

    pub fn with_transparency(mut self, transparency: f64) -> Self {
        self.transparency = transparency;
        self
    }

    pub fn with_refractive_index(mut self, refractive_index: f64) -> Self {
        self.refractive_index = refractive_index;
        self
    }

    pub fn lighting(&self, object: Shape, light: Light, point: Tuple, eye_vector: Tuple, normal_vector: Tuple, in_shadow: bool) -> Color {
        let real_color = if self.pattern.is_some() {
            self.pattern.clone().unwrap().pattern_at_shape(object, point)
        } else {
            self.color
        };

        let effective_color = real_color * light.intensity;
        let light_vector = (light.position - point).normalize();
        let ambient = effective_color * self.ambient;

        let light_dot_normal = light_vector.dot(normal_vector);
        let diffuse: Color;
        let specular: Color;

        if light_dot_normal < 0. {
            diffuse = BLACK;
            specular = BLACK;
        } else {
            diffuse = effective_color * self.diffuse * light_dot_normal;
            
            let reflect_vector = (-light_vector).reflect(normal_vector);
            let reflect_dot_eye = reflect_vector.dot(eye_vector);
            if near_eq(0., reflect_dot_eye) || reflect_dot_eye < 0. {
                specular = BLACK; 
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }

        if in_shadow { ambient } else { ambient + diffuse + specular }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::color::Color;
    use super::super::light::Light;
    use super::super::ORIGIN;
    use super::super::pattern::*;
    use super::super::sphere::Sphere;
    use super::super::tuple::Tuple;
    use super::super::WHITE;

    #[test]
    fn default_material() {
        let expected_color = WHITE;
        let expected_ambient = 0.1;
        let expected_diffuse = 0.9;
        let expected_specular = 0.9;
        let expected_shininess = 200.;

        let actual: Material = Default::default();

        assert_eq!(expected_color, actual.color);
        assert_eq!(expected_ambient, actual.ambient);
        assert_eq!(expected_diffuse, actual.diffuse);
        assert_eq!(expected_specular, actual.specular);
        assert_eq!(expected_shininess, actual.shininess);
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface() {
        let material: Material = Default::default();
        let position = ORIGIN;
        let eye_vector = Tuple::vector(0., 0., -1.);
        let normal_vector = Tuple::vector(0., 0., -1.);
        let light = Light::point_light(Tuple::point(0., 0., -10.), WHITE);
        let sphere = Shape::Sphere(Sphere::new());

        let expected = Color::new(1.9, 1.9, 1.9);

        let actual = material.lighting(sphere, light, position, eye_vector, normal_vector, false);

        assert_eq!(expected, actual);
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_eye_offset_45deg() {
        let material: Material = Default::default();
        let position = ORIGIN;
        let eye_vector = Tuple::vector(0., 2_f64.sqrt() / 2., -2_f64.sqrt() / 2.);
        let normal_vector = Tuple::vector(0., 0., -1.);
        let light = Light::point_light(Tuple::point(0., 0., -10.), WHITE);
        let sphere = Shape::Sphere(Sphere::new());

        let expected = WHITE;

        let actual = material.lighting(sphere, light, position, eye_vector, normal_vector, false);

        assert_eq!(expected, actual);
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45deg() {
        let material: Material = Default::default();
        let position = ORIGIN;
        let eye_vector = Tuple::vector(0., 0., -1.);
        let normal_vector = Tuple::vector(0., 0., -1.);
        let light = Light::point_light(Tuple::point(0., 10., -10.), WHITE);
        let sphere = Shape::Sphere(Sphere::new());

        let expected = Color::new(0.7364, 0.7364, 0.7364);

        let actual = material.lighting(sphere, light, position, eye_vector, normal_vector, false);

        assert_eq!(expected, actual);
    }

    #[test]
    fn lighting_with_eye_in_path_of_reflection_vector() {
        let material: Material = Default::default();
        let position = ORIGIN;
        let eye_vector = Tuple::vector(0., -2_f64.sqrt() / 2., -2_f64.sqrt() / 2.);
        let normal_vector = Tuple::vector(0., 0., -1.);
        let light = Light::point_light(Tuple::point(0., 10., -10.), WHITE);
        let sphere = Shape::Sphere(Sphere::new());

        let expected = Color::new(1.6364, 1.6364, 1.6364);

        let actual = material.lighting(sphere, light, position, eye_vector, normal_vector, false);

        assert_eq!(expected, actual);
    }

    #[test]
    fn lighting_with_light_behind_surface() {
        let material: Material = Default::default();
        let position = ORIGIN;
        let eye_vector = Tuple::vector(0., 0., -1.);
        let normal_vector = Tuple::vector(0., 0., -1.);
        let light = Light::point_light(Tuple::point(0., 0., 10.), WHITE);
        let sphere = Shape::Sphere(Sphere::new());

        let expected = Color::new(0.1, 0.1, 0.1);

        let actual = material.lighting(sphere, light, position, eye_vector, normal_vector, false);

        assert_eq!(expected, actual);
    }

    #[test]
    fn lighting_with_surface_in_shadow() {
        let material: Material = Default::default();
        let position = ORIGIN;
        let eye_vector = Tuple::vector(0., 0., -1.);
        let normal_vector = Tuple::vector(0., 0., -1.);
        let light = Light::point_light(Tuple::point(0., 0., -10.), WHITE);
        let in_shadow = true;
        let sphere = Shape::Sphere(Sphere::new());

        let expected = Color::new(0.1, 0.1, 0.1);

        let actual = material.lighting(sphere, light, position, eye_vector, normal_vector, in_shadow);

        assert_eq!(expected, actual);
    }

    #[test]
    fn lighting_with_pattern_applied() {
        let mut material: Material = Default::default();
        material.pattern = Some(Pattern::Striped(StripedPattern::new(WHITE, BLACK)));
        material.ambient = 1.;
        material.diffuse = 0.;
        material.specular = 0.;
        let eye_vector = Tuple::vector(0., 0., -1.);
        let normal_vector = Tuple::vector(0., 0., -1.);
        let light = Light::point_light(Tuple::point(0., 0., -10.), WHITE);
        let sphere = Shape::Sphere(Sphere::new());
        
        let expected_color1 = WHITE;
        let expected_color2 = BLACK;

        let actual_color1 = material.lighting(sphere.clone(), light, Tuple::point(0.9, 0., 0.), eye_vector, normal_vector, false);
        let actual_color2 = material.lighting(sphere, light, Tuple::point(1.1, 0., 0.), eye_vector, normal_vector, false);

        assert_eq!(expected_color1, actual_color1);
        assert_eq!(expected_color2, actual_color2);
    }

    #[test]
    fn reflectivity_for_default_material() {
        let material: Material = Default::default();

        let expected = 0.;

        let actual = material.reflective;

        assert_eq!(expected, actual);
    }

    #[test]
    fn transparency_and_refractive_index_for_default_material() {
        let material: Material = Default::default();

        let expected_transparency = 0.;
        let expected_refractive_index = 1.;

        let actual_transparency = material.transparency;
        let actual_refractive_index = material.refractive_index;

        assert_eq!(expected_transparency, actual_transparency);
        assert_eq!(expected_refractive_index, actual_refractive_index);
    }

    #[test]
    fn material_builder_sets_material() {
        let mut expected: Material = Default::default();
        expected.color = Color::new(1., 1., 0.);
        expected.shininess = 300.;
        expected.transparency = 0.1;

        let actual = Material::new().with_color(Color::new(1., 1., 0.))
            .with_shininess(300.).with_transparency(0.1);
        
        assert_eq!(expected, actual);
    }
}
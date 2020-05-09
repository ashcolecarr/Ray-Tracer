use super::color::Color;
use super::matrix::Matrix;
use super::shape::{Shape, Actionable};
use super::tuple::Tuple;

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Striped (StripedPattern),
    Gradient (GradientPattern),
    Ring (RingPattern),
    Checkered (CheckeredPattern),
    RingGradient (RingGradientPattern),
    Test (TestPattern),
}

pub trait PatternTrait {
    fn pattern_at_shape(&self, object: Shape, world_point: Tuple) -> Color;
    fn get_transform(&self) -> Matrix;
    fn set_transform(&mut self, transform: Matrix);
}

impl PatternTrait for Pattern {
    fn pattern_at_shape(&self, object: Shape, world_point: Tuple) -> Color {
        let object_point = object.get_transform().inverse().unwrap() * world_point;
        let pattern_point = self.get_transform().inverse().unwrap() * object_point;

        match self {
            Pattern::Striped(striped) => striped.pattern_at(pattern_point),
            Pattern::Gradient(gradient) => gradient.pattern_at(pattern_point),
            Pattern::Ring(ring) => ring.pattern_at(pattern_point),
            Pattern::Checkered(checkered) => checkered.pattern_at(pattern_point),
            Pattern::RingGradient(ring_gradient) => ring_gradient.pattern_at(pattern_point),
            Pattern::Test(test) => test.pattern_at(pattern_point),
        }
    }

    fn get_transform(&self) -> Matrix {
        match self.clone() {
            Pattern::Striped(striped) => striped.transform,
            Pattern::Gradient(gradient) => gradient.transform,
            Pattern::Ring(ring) => ring.transform,
            Pattern::Checkered(checkered) => checkered.transform,
            Pattern::RingGradient(ring_gradient) => ring_gradient.transform,
            Pattern::Test(test) => test.transform,
        }
    }

    fn set_transform(&mut self, transform: Matrix) {
        match self {
            Pattern::Striped(striped) => striped.transform = transform,
            Pattern::Gradient(gradient) => gradient.transform = transform,
            Pattern::Ring(ring) => ring.transform = transform,
            Pattern::Checkered(checkered) => checkered.transform = transform,
            Pattern::RingGradient(ring_gradient) => ring_gradient.transform = transform,
            Pattern::Test(test) => test.transform = transform,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StripedPattern {
    pub a: Color,
    pub b: Color,
    pub transform: Matrix,
}

impl StripedPattern {
    pub fn new(a: Color, b: Color) -> Self {
        Self { a, b, transform: Matrix::identity(4) }
    }

    pub fn pattern_at(&self, point: Tuple) -> Color {
        if point.x.floor() as i32 % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GradientPattern {
    pub a: Color,
    pub b: Color,
    pub transform: Matrix,
}

impl GradientPattern {
    pub fn new(a: Color, b: Color) -> Self {
        Self { a, b, transform: Matrix::identity(4) }
    }

    pub fn pattern_at(&self, point: Tuple) -> Color {
        let distance = self.b - self.a;
        let fraction = point.x - point.x.floor();

        self.a + distance * fraction
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RingPattern {
    pub a: Color,
    pub b: Color,
    pub transform: Matrix,
}

impl RingPattern {
    pub fn new(a: Color, b: Color) -> Self {
        Self { a, b, transform: Matrix::identity(4) }
    }

    pub fn pattern_at(&self, point: Tuple) -> Color {
        if ((point.x.powi(2) + point.z.powi(2)).sqrt().floor() as i32) % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CheckeredPattern {
    pub a: Color,
    pub b: Color,
    pub transform: Matrix,
}

impl CheckeredPattern {
    pub fn new(a: Color, b: Color) -> Self {
        Self { a, b, transform: Matrix::identity(4) }
    }

    pub fn pattern_at(&self, point: Tuple) -> Color {
        if (point.x.floor() + point.y.floor() + point.z.floor()) as i32 % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RingGradientPattern {
    pub a: Color,
    pub b: Color,
    pub transform: Matrix,
}

impl RingGradientPattern {
    pub fn new(a: Color, b: Color) -> Self {
        Self { a, b, transform: Matrix::identity(4) }
    }

    pub fn pattern_at(&self, point: Tuple) -> Color {
        let distance = self.b - self.a;
        let c = (point.x.powi(2) + point.z.powi(2)).sqrt();
        let fraction = c - c.floor();

        self.a + distance * fraction
    }
}

/// For testing purposes only--not meant to be used directly.
#[derive(Debug, Clone, PartialEq)]
pub struct TestPattern {
    pub transform: Matrix,
}

impl TestPattern {
    pub fn new() -> Self {
        Self { transform: Matrix::identity(4) }
    }

    pub fn pattern_at(&self, point: Tuple) -> Color {
        Color::new(point.x, point.y, point.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::BLACK;
    use super::super::ORIGIN;
    use super::super::shape::{Shape, Actionable};
    use super::super::sphere::Sphere;
    use super::super::transformation::*;
    use super::super::tuple::Tuple;
    use super::super::WHITE;

    #[test]
    fn default_pattern_transformation() {
        let pattern = Pattern::Test(TestPattern::new());

        let expected = Matrix::identity(4);

        let actual = pattern.get_transform();

        assert_eq!(expected, actual);
    }

    #[test]
    fn assigning_transformation() {
        let mut pattern = Pattern::Test(TestPattern::new());
        pattern.set_transform(translate(1., 2., 3.));

        let expected = translate(1., 2., 3.);

        let actual = pattern.get_transform();

        assert_eq!(expected, actual);
    }
    
    #[test]
    fn creating_stripe_pattern() {
        let expected_color1 = WHITE;
        let expected_color2 = BLACK;

        let actual = StripedPattern::new(WHITE, BLACK);

        assert_eq!(expected_color1, actual.a);
        assert_eq!(expected_color2, actual.b);
    }

    #[test]
    fn stripe_pattern_is_constant_in_y() {
        let pattern = StripedPattern::new(WHITE, BLACK);

        let expected_color = WHITE;

        let actual1 = pattern.pattern_at(ORIGIN);
        let actual2 = pattern.pattern_at(Tuple::point(0., 1., 0.));
        let actual3 = pattern.pattern_at(Tuple::point(0., 2., 0.));

        assert_eq!(expected_color, actual1);
        assert_eq!(expected_color, actual2);
        assert_eq!(expected_color, actual3);
    }

    #[test]
    fn stripe_pattern_is_constant_in_z() {
        let pattern = StripedPattern::new(WHITE, BLACK);

        let expected_color = WHITE;

        let actual1 = pattern.pattern_at(ORIGIN);
        let actual2 = pattern.pattern_at(Tuple::point(0., 0., 1.));
        let actual3 = pattern.pattern_at(Tuple::point(0., 0., 2.));

        assert_eq!(expected_color, actual1);
        assert_eq!(expected_color, actual2);
        assert_eq!(expected_color, actual3);
    }

    #[test]
    fn stripe_pattern_alternates_in_x() {
        let pattern = StripedPattern::new(WHITE, BLACK);

        let expected_color1 = WHITE;
        let expected_color2 = BLACK;

        let actual1 = pattern.pattern_at(ORIGIN);
        let actual2 = pattern.pattern_at(Tuple::point(0.9, 0., 0.));
        let actual3 = pattern.pattern_at(Tuple::point(1., 0., 0.));
        let actual4 = pattern.pattern_at(Tuple::point(-0.1, 0., 0.));
        let actual5 = pattern.pattern_at(Tuple::point(-1., 0., 0.));
        let actual6 = pattern.pattern_at(Tuple::point(-1.1, 0., 0.));

        assert_eq!(expected_color1, actual1);
        assert_eq!(expected_color1, actual2);
        assert_eq!(expected_color2, actual3);
        assert_eq!(expected_color2, actual4);
        assert_eq!(expected_color2, actual5);
        assert_eq!(expected_color1, actual6);
    }

    #[test]
    fn pattern_with_object_transformation() {
        let mut shape = Shape::Sphere(Sphere::new());
        shape.set_transform(scale(2., 2., 2.));
        let pattern = Pattern::Test(TestPattern::new());

        let expected = Color::new(1., 1.5, 2.);

        let actual = pattern.pattern_at_shape(shape, Tuple::point(2., 3., 4.));

        assert_eq!(expected, actual);
    }

    #[test]
    fn pattern_with_pattern_transformation() {
        let shape = Shape::Sphere(Sphere::new());
        let mut pattern = Pattern::Test(TestPattern::new());
        pattern.set_transform(scale(2., 2., 2.));

        let expected = Color::new(1., 1.5, 2.);

        let actual = pattern.pattern_at_shape(shape, Tuple::point(2., 3., 4.));

        assert_eq!(expected, actual);
    }

    #[test]
    fn pattern_with_both_object_and_pattern_transformation() {
        let mut shape = Shape::Sphere(Sphere::new());
        shape.set_transform(scale(2., 2., 2.));
        let mut pattern = Pattern::Test(TestPattern::new());
        pattern.set_transform(translate(0.5, 1., 1.5));

        let expected = Color::new(0.75, 0.5, 0.25);

        let actual = pattern.pattern_at_shape(shape, Tuple::point(2.5, 3., 3.5));

        assert_eq!(expected, actual);
    }

    #[test]
    fn gradient_linearly_interpolates_between_colors() {
        let pattern = GradientPattern::new(WHITE, BLACK);

        let expected_color1 = WHITE;
        let expected_color2 = Color::new(0.75, 0.75, 0.75);
        let expected_color3 = Color::new(0.5, 0.5, 0.5);
        let expected_color4 = Color::new(0.25, 0.25, 0.25);

        let actual_color1 = pattern.pattern_at(ORIGIN);
        let actual_color2 = pattern.pattern_at(Tuple::point(0.25, 0., 0.));
        let actual_color3 = pattern.pattern_at(Tuple::point(0.5, 0., 0.));
        let actual_color4 = pattern.pattern_at(Tuple::point(0.75, 0., 0.));

        assert_eq!(expected_color1, actual_color1);
        assert_eq!(expected_color2, actual_color2);
        assert_eq!(expected_color3, actual_color3);
        assert_eq!(expected_color4, actual_color4);
    }

    #[test]
    fn ring_should_extend_in_both_x_and_z() {
        let pattern = RingPattern::new(WHITE, BLACK);

        let expected_color1 = WHITE; 
        let expected_color2 = BLACK;
        let expected_color3 = BLACK;
        let expected_color4 = BLACK;

        let actual_color1 = pattern.pattern_at(ORIGIN);
        let actual_color2 = pattern.pattern_at(Tuple::point(1., 0., 0.));
        let actual_color3 = pattern.pattern_at(Tuple::point(0., 0., 1.));
        let actual_color4 = pattern.pattern_at(Tuple::point(0.708, 0., 0.708));

        assert_eq!(expected_color1, actual_color1);
        assert_eq!(expected_color2, actual_color2);
        assert_eq!(expected_color3, actual_color3);
        assert_eq!(expected_color4, actual_color4);
    }

    #[test]
    fn checkers_should_repeat_in_x() {
        let pattern = CheckeredPattern::new(WHITE, BLACK);

        let expected_color1 = WHITE; 
        let expected_color2 = WHITE; 
        let expected_color3 = BLACK; 

        let actual_color1 = pattern.pattern_at(ORIGIN);
        let actual_color2 = pattern.pattern_at(Tuple::point(0.99, 0., 0.));
        let actual_color3 = pattern.pattern_at(Tuple::point(1.01, 0., 0.));

        assert_eq!(expected_color1, actual_color1);
        assert_eq!(expected_color2, actual_color2);
        assert_eq!(expected_color3, actual_color3);
    }

    #[test]
    fn checkers_should_repeat_in_y() {
        let pattern = CheckeredPattern::new(WHITE, BLACK);

        let expected_color1 = WHITE; 
        let expected_color2 = WHITE; 
        let expected_color3 = BLACK; 

        let actual_color1 = pattern.pattern_at(ORIGIN);
        let actual_color2 = pattern.pattern_at(Tuple::point(0., 0.99, 0.));
        let actual_color3 = pattern.pattern_at(Tuple::point(0., 1.01, 0.));

        assert_eq!(expected_color1, actual_color1);
        assert_eq!(expected_color2, actual_color2);
        assert_eq!(expected_color3, actual_color3);
    }
    
    #[test]
    fn checkers_should_repeat_in_z() {
        let pattern = CheckeredPattern::new(WHITE, BLACK);

        let expected_color1 = WHITE; 
        let expected_color2 = WHITE; 
        let expected_color3 = BLACK; 

        let actual_color1 = pattern.pattern_at(ORIGIN);
        let actual_color2 = pattern.pattern_at(Tuple::point(0., 0., 0.99));
        let actual_color3 = pattern.pattern_at(Tuple::point(0., 0., 1.01));

        assert_eq!(expected_color1, actual_color1);
        assert_eq!(expected_color2, actual_color2);
        assert_eq!(expected_color3, actual_color3);
    }

    #[test]
    fn ring_gradient_radially_interpolates_between_colors() {
        let pattern = GradientPattern::new(WHITE, BLACK);

        let expected_color1 = WHITE;
        let expected_color2 = Color::new(0.75, 0.75, 0.75);
        let expected_color3 = Color::new(0.5, 0.5, 0.5);
        let expected_color4 = Color::new(0.25, 0.25, 0.25);

        let actual_color1 = pattern.pattern_at(ORIGIN);
        let actual_color2 = pattern.pattern_at(Tuple::point(0.25, 0., 0.25));
        let actual_color3 = pattern.pattern_at(Tuple::point(0.5, 0., 0.5));
        let actual_color4 = pattern.pattern_at(Tuple::point(0.75, 0., 0.75));

        assert_eq!(expected_color1, actual_color1);
        assert_eq!(expected_color2, actual_color2);
        assert_eq!(expected_color3, actual_color3);
        assert_eq!(expected_color4, actual_color4);
    }
}
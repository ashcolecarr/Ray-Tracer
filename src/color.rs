use super::near_eq;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        near_eq(self.red, other.red) && near_eq(self.green, other.green) &&
            near_eq(self.blue, other.blue)
    }
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self { red, green, blue }
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Self) -> Self {
        Self {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue
        }
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, other: Self) -> Self {
        Self {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, other: f64) -> Self {
        Self {
            red: self.red * other,
            green: self.green * other,
            blue: self.blue * other
        }
    }
}

// Calculate the Hadamard product of two colors.
impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Self) -> Self {
        Self {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::near_eq;

    #[test]
    fn colors_are_red_green_blue_tuples() {
        let expected_red = -0.5;
        let expected_green = 0.4;
        let expected_blue = 1.7;

        let actual = Color { red: -0.5, green: 0.4, blue: 1.7 };

        assert!(near_eq(expected_red, actual.red));
        assert!(near_eq(expected_green, actual.green));
        assert!(near_eq(expected_blue, actual.blue));
    }

    #[test]
    fn adding_colors() {
        let color1 = Color::new(0.9, 0.6, 0.75);
        let color2 = Color::new(0.7, 0.1, 0.25);
        
        let expected = Color::new(1.6, 0.7, 1.);

        let actual = color1 + color2;

        assert_eq!(expected, actual);
    }

    #[test]
    fn subtracting_colors() {
        let color1 = Color::new(0.9, 0.6, 0.75);
        let color2 = Color::new(0.7, 0.1, 0.25);
        
        let expected = Color::new(0.2, 0.5, 0.5);

        let actual = color1 - color2;

        assert_eq!(expected, actual);
    }

    #[test]
    fn multiplying_color_by_scalar() {
        let color = Color::new(0.2, 0.3, 0.4);
        
        let expected = Color::new(0.4, 0.6, 0.8);

        let actual = color * 2.;

        assert_eq!(expected, actual);
    }

    #[test]
    fn multiplying_colors() {
        let color1 = Color::new(1., 0.2, 0.4);
        let color2 = Color::new(0.9, 1., 0.1);
        
        let expected = Color::new(0.9, 0.2, 0.04);

        let actual = color1 * color2;

        assert_eq!(expected, actual);
    }
}
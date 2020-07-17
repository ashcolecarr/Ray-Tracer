use super::BLACK;
use super::color::Color;

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = vec![BLACK; width * height];

        Canvas { width, height, pixels }
    }

    pub fn get_width(&self) -> &usize {
        &self.width
    }
    
    pub fn get_height(&self) -> &usize {
        &self.height
    }

    pub fn write_pixel(&mut self, x: u32, y: u32, color: Color) {
        self.pixels[y as usize * self.width as usize + x as usize] = color;
    }

    pub fn pixel_at(&self, x: u32, y: u32) -> Color {
        self.pixels[y as usize * self.width as usize + x as usize]
    }

    pub fn canvas_to_ppm(&self) -> String {
        const MAX_COLOR: i32 = 255;
        let mut ppm_data = String::new();

        // Write the header data.
        ppm_data.push_str("P3\n");
        ppm_data.push_str(format!("{} {}\n", self.width, self.height).as_str());
        ppm_data.push_str(format!("{}\n", MAX_COLOR).as_str());

        // Write the pixel data.
        let mut pixel_color = String::new();
        for (pos, pixel) in self.pixels.iter().enumerate() {
            let red = Self::scale_color(pixel.red, MAX_COLOR).to_string();
            let green = Self::scale_color(pixel.green, MAX_COLOR).to_string();
            let blue = Self::scale_color(pixel.blue, MAX_COLOR).to_string();

            // Some programs don't work correctly if the line length 
            // is greater than 70.
            if pixel_color.len() + red.len() >= 70 {
                pixel_color.pop();
                pixel_color.push_str("\n");
                ppm_data.push_str(pixel_color.as_str());
                pixel_color.clear();
            }
            pixel_color.push_str((red + " ").as_str());

            if pixel_color.len() + green.len() >= 70 {
                pixel_color.pop();
                pixel_color.push_str("\n");
                ppm_data.push_str(pixel_color.as_str());
                pixel_color.clear();
            }
            pixel_color.push_str((green + " ").as_str());

            if pixel_color.len() + blue.len() >= 70 {
                pixel_color.pop();
                pixel_color.push_str("\n");
                ppm_data.push_str(pixel_color.as_str());
                pixel_color.clear();
            }
            pixel_color.push_str((blue + " ").as_str());

            if (pos + 1) % self.width == 0 {
                pixel_color.pop();
                pixel_color.push_str("\n");
                ppm_data.push_str(pixel_color.as_str());
                pixel_color.clear();
            }
        }

        ppm_data
    }

    fn scale_color(color_value: f64, max_color_value: i32) -> i32 {
        // Clamp color, if necessary.
        if color_value > 1.0 {
            return 255;
        }

        if color_value < 0.0 {
            return 0;
        }

        let scaled_color = color_value * max_color_value as f64;
        scaled_color.round() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::color::Color;

    #[test]
    fn creating_canvas() {
        let expected_width = 10;
        let expected_height = 20;
        let expected_color = Color::new(0., 0., 0.);

        let actual = Canvas::new(10, 20);

        assert_eq!(expected_width, actual.width);
        assert_eq!(expected_height, actual.height);
        for actual_color in actual.pixels {
            assert_eq!(expected_color, actual_color);
        }
    }

    #[test]
    fn writing_pixels_to_canvas() {
        let mut canvas = Canvas::new(10, 20);
        let red = Color::new(1., 0., 0.);
        
        let expected = Color::new(1., 0., 0.);

        canvas.write_pixel(2, 3, red);

        let actual = canvas.pixel_at(2, 3);

        assert_eq!(expected, actual);
    }

    #[test]
    fn constructing_ppm_header() {
        let canvas = Canvas::new(5, 3);

        let expected = String::from("P3\n\
                                     5 3\n\
                                     255\n");

        let actual = canvas.canvas_to_ppm().split('\n').take(3)
            .collect::<Vec<&str>>().join("\n") + "\n";
        
        assert_eq!(expected, actual);
    }

    #[test]
    fn constructing_ppm_pixel_data() {
        let mut canvas = Canvas::new(5, 3);
        let color1 = Color::new(1.5, 0., 0.);
        let color2 = Color::new(0., 0.5, 0.);
        let color3 = Color::new(-0.5, 0., 1.);

        canvas.write_pixel(0, 0, color1);
        canvas.write_pixel(2, 1, color2);
        canvas.write_pixel(4, 2, color3);

        let expected = String::from("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n\
                                     0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n\
                                     0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n");

        let actual = canvas.canvas_to_ppm().split('\n').skip(3).take(3)
            .collect::<Vec<&str>>().join("\n") + "\n"; 

        assert_eq!(expected, actual);
    }

    #[test]
    fn splitting_long_lines_in_ppm_files() {
        let mut canvas = Canvas::new(10, 2);

        let expected = String::from("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n\
                                     153 255 204 153 255 204 153 255 204 153 255 204 153\n\
                                     255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n\
                                     153 255 204 153 255 204 153 255 204 153 255 204 153\n");
        
        for i in 0..10 {
            for j in 0..2 {
                canvas.write_pixel(i, j, Color::new(1.0, 0.8, 0.6));
            }
        }
        let actual = canvas.canvas_to_ppm().split('\n').skip(3).take(4)
            .collect::<Vec<&str>>().join("\n") + "\n"; 

        assert_eq!(expected, actual);
    }

    #[test]
    fn ppm_files_are_terminated_by_newline_character() {
        let canvas = Canvas::new(5, 3);

        let expected = '\n';

        let actual = canvas.canvas_to_ppm().chars()
            .last().unwrap();

        assert_eq!(expected, actual);
    }
}
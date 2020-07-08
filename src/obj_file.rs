use super::group::Group;
use super::ORIGIN;
use super::shape::{Shape, CommonShape};
use super::triangle::Triangle;
use super::tuple::Tuple;

#[derive(Debug)]
pub struct Parser {
    pub vertices: Vec<Tuple>,
    pub groups: Vec<Shape>,
    pub lines_ignored: i32,
}

impl Parser {
    pub fn new(vertices: Vec<Tuple>, groups: Vec<Shape>, lines_ignored: i32) -> Self {
        Self {
            vertices,
            groups,
            lines_ignored,
        }
    }
}

pub fn parse_obj_file(data: String) -> Parser {
    let lines = data.lines();
    let mut lines_ignored = 0;

    let mut vertices = vec![ORIGIN];
    let mut groups = vec![Shape::Group(Group::new())];
    'outer: for line in lines {
        let record: Vec<&str> = line.split_whitespace().collect();
        if record.is_empty() {
            lines_ignored += 1;
            continue;
        }

        if record[0] == "v" {
            let mut coordinates: Vec<f64> = vec![];

            for rec in &record[1..=3] {
                match rec.parse() {
                    Ok(r) => coordinates.push(r),
                    Err(_e) => {
                        continue 'outer;
                    }
                }
            }

            vertices.push(Tuple::point(coordinates[0], coordinates[1], coordinates[2]));
        } else if record[0] == "f" {
            let mut points: Vec<usize> = vec![];

            for rec in &record[1..] {
                match rec.parse() {
                    Ok(r) => points.push(r),
                    Err(_e) => {
                        continue 'outer;
                    }
                }
            }

            if points.len() > 3 {
                let triangles = fan_triangulation(&vertices);
                for mut triangle in triangles {
                    groups.last_mut().unwrap().add_child(&mut triangle);
                }
            }

            let mut triangle = Shape::Triangle(Triangle::new(vertices[points[0]], vertices[points[1]], vertices[points[2]]));
            groups.last_mut().unwrap().add_child(&mut triangle);
        } else if record[0] == "g" {
            groups.push(Shape::Group(Group::new()));
        } else {
            lines_ignored += 1;
        }
    }

    Parser::new(vertices, groups, lines_ignored)
}

fn fan_triangulation(vertices: &Vec<Tuple>) -> Vec<Shape> {
    let mut triangles: Vec<Shape> = vec![];

    for index in 2..vertices.len() - 1 {
        triangles.push(Shape::Triangle(Triangle::new(vertices[1], vertices[index], vertices[index + 1])));
    }

    triangles
}

pub fn obj_to_group(parser: Parser) -> Shape {
    let mut main_group = Shape::Group(Group::new());

    for mut group in parser.groups {
        main_group.add_child(&mut group);
    }

    main_group
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn ignoring_unrecognized_lines() {
        let gibberish = String::from("There was a young lady named Bright\n\
                                      who traveled much faster than light.\n\
                                      She set out one day\n\
                                      in a relative way,\n\
                                      and came back the previous night.\n");

        let expected_lines_ignored = 5;

        let actual = parse_obj_file(gibberish);

        assert_eq!(expected_lines_ignored, actual.lines_ignored);
    }

    #[test]
    fn vertex_records() {
        let file = String::from("v -1 1 0\n\
                                 v -1.0000 0.5000 0.0000\n\
                                 v 1 0 0\n\
                                 v 1 1 0\n");

        let expected_point1 = Tuple::point(-1., 1., 0.);
        let expected_point2 = Tuple::point(-1., 0.5, 0.);
        let expected_point3 = Tuple::point(1., 0., 0.); 
        let expected_point4 = Tuple::point(1., 1., 0.);

        let actual = parse_obj_file(file);

        assert_eq!(expected_point1, actual.vertices[1]);
        assert_eq!(expected_point2, actual.vertices[2]);
        assert_eq!(expected_point3, actual.vertices[3]);
        assert_eq!(expected_point4, actual.vertices[4]);
    } 

    #[test]
    fn parsing_triangle_faces() {
        let file = String::from("v -1 1 0\n\
                                 v -1 0 0\n\
                                 v 1 0 0\n\
                                 v 1 1 0\n\
                                 f 1 2 3\n\
                                 f 1 3 4\n");

        let actual = parse_obj_file(file);
        assert_eq!(actual.vertices[1], actual.groups[0].get_shapes()[0].get_points().0);
        assert_eq!(actual.vertices[2], actual.groups[0].get_shapes()[0].get_points().1);
        assert_eq!(actual.vertices[3], actual.groups[0].get_shapes()[0].get_points().2);
        assert_eq!(actual.vertices[1], actual.groups[0].get_shapes()[1].get_points().0);
        assert_eq!(actual.vertices[3], actual.groups[0].get_shapes()[1].get_points().1);
        assert_eq!(actual.vertices[4], actual.groups[0].get_shapes()[1].get_points().2);
    }

    #[test]
    fn triangulating_polygons() {
        let file = String::from("v -1 1 0\n\
                                 v -1 0 0\n\
                                 v 1 0 0\n\
                                 v 1 1 0\n\
                                 v 0 2 0\n\
                                 f 1 2 3 4 5\n");

        let actual = parse_obj_file(file);

        assert_eq!(actual.vertices[1], actual.groups[0].get_shapes()[0].get_points().0);
        assert_eq!(actual.vertices[2], actual.groups[0].get_shapes()[0].get_points().1);
        assert_eq!(actual.vertices[3], actual.groups[0].get_shapes()[0].get_points().2);
        assert_eq!(actual.vertices[1], actual.groups[0].get_shapes()[1].get_points().0);
        assert_eq!(actual.vertices[3], actual.groups[0].get_shapes()[1].get_points().1);
        assert_eq!(actual.vertices[4], actual.groups[0].get_shapes()[1].get_points().2);
        assert_eq!(actual.vertices[1], actual.groups[0].get_shapes()[2].get_points().0);
        assert_eq!(actual.vertices[4], actual.groups[0].get_shapes()[2].get_points().1);
        assert_eq!(actual.vertices[5], actual.groups[0].get_shapes()[2].get_points().2);
    }

    #[test]
    fn triangles_in_groups() {
        let file = "triangles.obj";
        let file_data =  fs::read_to_string(file);

        let actual = parse_obj_file(file_data.unwrap());

        assert_eq!(actual.vertices[1], actual.groups[1].get_shapes()[0].get_points().0);
        assert_eq!(actual.vertices[2], actual.groups[1].get_shapes()[0].get_points().1);
        assert_eq!(actual.vertices[3], actual.groups[1].get_shapes()[0].get_points().2);
        assert_eq!(actual.vertices[1], actual.groups[2].get_shapes()[0].get_points().0);
        assert_eq!(actual.vertices[3], actual.groups[2].get_shapes()[0].get_points().1);
        assert_eq!(actual.vertices[4], actual.groups[2].get_shapes()[0].get_points().2);
    }

    #[test]
    fn converting_obj_file_to_group() {
        let file = "triangles.obj";
        let file_data =  fs::read_to_string(file);
        let parser = parse_obj_file(file_data.unwrap());

        let expected_group_count = 3;
        let expected_first_group_id = parser.groups[1].get_id();
        let expected_second_group_id = parser.groups[2].get_id();

        let actual = obj_to_group(parser);

        assert_eq!(expected_group_count, actual.get_shapes().len());
        assert_eq!(expected_first_group_id, actual.get_shapes()[1].get_id());
        assert_eq!(expected_second_group_id, actual.get_shapes()[2].get_id());
    }
}
use super::near_eq;
use super::tuple::Tuple;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Mul;

#[derive(Debug, Clone)]
pub struct Matrix {
    rows: usize,
    columns: usize,
    values: Vec<f64>,
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.rows == other.rows && self.columns == other.columns {
            for row in 0..self.rows {
                for column in 0..self.columns {
                    if !near_eq(self[row][column], other[row][column]) {
                        return false;
                    }
                }
            }

            return true;
        }

        false
    }
}

impl Matrix {
    pub fn new(rows: usize, columns: usize, values: Vec<f64>) -> Self {
        Matrix { rows, columns, values }
    }
    
    pub fn get_rows(&self) -> &usize {
        &self.rows
    }

    pub fn get_columns(&self) -> &usize {
        &self.columns
    }

    pub fn identity(size: usize) -> Self {
        let mut matrix = Matrix::new(size, size, vec![0.; size * size]);

        for row in 0..matrix.rows {
            matrix[row][row] = 1.;
        }

        matrix
    }

    pub fn transpose(&self) -> Self {
        let mut transposition = Matrix::new(self.rows, self.columns, 
            vec![0.0; self.rows * self.columns]);
        
        for row in 0..self.rows {
            for column in 0..self.columns {
                transposition[row][column] = self[column][row];
            }
        }

        transposition
    }
}

impl Index<usize> for Matrix {
    type Output = [f64];

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index * self.columns .. (index + 1) * self.columns]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index * self.columns .. (index + 1) * self.columns]
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut product = Matrix::new(self.rows, other.columns, 
            vec![0.0; self.rows * other.columns]);

        for row in 0..self.rows {
            for column in 0..other.columns {
                for element in 0..self.columns {
                    product[row][column] += self[row][element] * other[element][column];
                }
            } 
        }

        product
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, other: Tuple) -> Tuple {
        let tuple = Matrix::new(4, 1, vec![other.x, other.y, other.z, other.w]);

        let product = self * tuple;

        Tuple { x: product[0][0], y: product[1][0], z: product[2][0], w: product[3][0] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::near_eq;

    #[test]
    fn constructing_and_inspecting_4x4_matrix() {
        let values = vec![1., 2., 3., 4., 5.5, 6.5, 7.5, 8.5,
            9., 10., 11., 12., 13.5, 14.5, 15.5, 16.5];

        let expected_0_0 = 1.;        
        let expected_0_3 = 4.;        
        let expected_1_0 = 5.5;        
        let expected_1_2 = 7.5;        
        let expected_2_2 = 11.;        
        let expected_3_0 = 13.5;        
        let expected_3_2 = 15.5;        

        let actual = Matrix::new(4, 4, values);

        assert!(near_eq(expected_0_0, actual[0][0]));
        assert!(near_eq(expected_0_3, actual[0][3]));
        assert!(near_eq(expected_1_0, actual[1][0]));
        assert!(near_eq(expected_1_2, actual[1][2]));
        assert!(near_eq(expected_2_2, actual[2][2]));
        assert!(near_eq(expected_3_0, actual[3][0]));
        assert!(near_eq(expected_3_2, actual[3][2]));
    }

    #[test]
    fn a_2x2_matrix_ought_to_be_representable() {
        let values = vec![-3., 5., 1., -2.];

        let expected_0_0 = -3.;        
        let expected_0_1 = 5.;        
        let expected_1_0 = 1.;        
        let expected_1_1 = -2.;        

        let actual = Matrix::new(2, 2, values);
 
        assert!(near_eq(expected_0_0, actual[0][0]));
        assert!(near_eq(expected_0_1, actual[0][1]));
        assert!(near_eq(expected_1_0, actual[1][0]));
        assert!(near_eq(expected_1_1, actual[1][1]));
    }

    #[test]
    fn a_3x3_matrix_ought_to_be_representable() {
        let values = vec![-3., 5., 0., 1., -2., -7., 0., 1., 1.];

        let expected_0_0 = -3.;        
        let expected_1_1 = -2.;        
        let expected_2_2 = 1.;        

        let actual = Matrix::new(3, 3, values);
 
        assert!(near_eq(expected_0_0, actual[0][0]));
        assert!(near_eq(expected_1_1, actual[1][1]));
        assert!(near_eq(expected_2_2, actual[2][2]));
    }

    #[test]
    fn matrix_equality_with_identical_matrices() {
        let values1 = vec![1., 2., 3., 4., 5., 6., 7., 8.,
            9., 8., 7., 6., 5., 4., 3., 2.];
        let values2 = vec![1., 2., 3., 4., 5., 6., 7., 8.,
            9., 8., 7., 6., 5., 4., 3., 2.];

        let actual1 = Matrix::new(4, 4, values1);

        let actual2 = Matrix::new(4, 4, values2);

        assert_eq!(actual1, actual2);
    }

    #[test]
    fn matrix_equality_with_different_matrices() {
        let values1 = vec![2., 3., 4., 5., 6., 7., 8., 9.,
            8., 7., 6., 5., 4., 3., 2., 1.];
        let values2 = vec![1., 2., 3., 4., 5., 6., 7., 8.,
            9., 8., 7., 6., 5., 4., 3., 2.];

        let actual1 = Matrix::new(4, 4, values1);

        let actual2 = Matrix::new(4, 4, values2);

        assert_ne!(actual1, actual2);
    }

    #[test]
    fn multiplying_two_matrices() {
        let values1 = vec![1., 2., 3., 4., 5., 6., 7., 8.,
            9., 8., 7., 6., 5., 4., 3., 2.];
        let values2 = vec![-2., 1., 2., 3., 3., 2., 1., -1.,
            4., 3., 6., 5., 1., 2., 7., 8.];
    
        let matrix1 = Matrix::new(4, 4, values1);
        let matrix2 = Matrix::new(4, 4, values2);

        let values3 = vec![20., 22., 50., 48., 44., 54., 114., 108.,
            40., 58., 110., 102., 16., 26., 46., 42.];
        let expected = Matrix::new(4, 4, values3);

        let actual = matrix1 * matrix2;

        assert_eq!(expected, actual);
    }

    #[test]
    fn matrix_multiplied_by_tuple() {
        let values = vec![1., 2., 3., 4., 2., 4., 4., 2.,
            8., 6., 4., 1., 0., 0., 0., 1.];
        let matrix = Matrix::new(4, 4, values);
        let tuple = Tuple::point(1., 2., 3.);

        let expected = Tuple::point(18., 24., 33.);

        let actual = matrix * tuple;

        assert_eq!(expected, actual);
    }

    #[test]
    fn multiplying_matrix_by_identity_matrix() {
        let values = vec![0., 1., 2., 4., 1., 2., 4., 8.,
            2., 4., 8., 16., 4., 8., 16., 32.];
        let matrix = Matrix::new(4, 4, values.clone());
        let identity_matrix = Matrix::identity(4);

        let expected = Matrix::new(4, 4, values);

        let actual = matrix * identity_matrix;

        assert_eq!(expected, actual);
    }

    #[test]
    fn multiplying_identity_matrix_by_tuple() {
        let tuple = Tuple { x: 1., y: 2., z: 3., w: 4. };
        let identity_matrix = Matrix::identity(4);

        let expected = Tuple { x: 1., y: 2., z: 3., w: 4. };

        let actual = identity_matrix * tuple;

        assert_eq!(expected, actual);
    }

    #[test]
    fn transposing_matrix() {
        let values1 = vec![0., 9., 3., 0., 9., 8., 0., 8.,
            1., 8., 5., 3., 0., 0., 5., 8.];
        let matrix = Matrix::new(4, 4, values1);

        let values2 = vec![0., 9., 1., 0., 9., 8., 8., 0.,
            3., 0., 5., 5., 0., 8., 3., 8.];
        let expected = Matrix::new(4, 4, values2);

        let actual = matrix.transpose();

        assert_eq!(expected, actual);
    }

    #[test]
    fn transposing_identity_matrix() {
        let identity_matrix = Matrix::identity(4);

        let expected = Matrix::identity(4);

        let actual = identity_matrix.transpose();

        assert_eq!(expected, actual);
    }
}
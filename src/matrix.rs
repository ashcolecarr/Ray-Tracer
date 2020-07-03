use super::CACHED_INVERSES;
use super::generate_matrix_id;
use super::near_eq;
use super::tuple::Tuple;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Mul;

#[derive(Debug, Clone)]
pub struct Matrix {
    id: i32,
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
        Matrix { id: generate_matrix_id(), rows, columns, values }
    }

    pub fn new_inverse(id: i32, rows: usize, columns: usize, values: Vec<f64>) -> Self {
        Matrix { id, rows, columns, values }
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

    pub fn determinant(&self) -> f64 {
        if self.rows == 2 && self.columns == 2 {
            return (self[0][0] * self[1][1]) - (self[0][1] * self[1][0]);
        }

        let mut determinant = 0.;
        for column in 0..self.columns {
            determinant += self[0][column] * self.cofactor(0, column);
        }

        determinant
    }

    pub fn submatrix(&self, row_to_remove: usize, column_to_remove: usize) -> Self {
        let mut submatrix = Matrix::new(self.rows - 1, self.columns - 1,
            vec![0.0; (self.rows - 1) * (self.columns - 1)]);
        let mut submatrix_row_count = 0;
        for row in 0..self.rows {
            if row == row_to_remove {
                continue;
            }

            let mut submatrix_column_count = 0;
            for column in 0..self.columns {
                if column == column_to_remove {
                    continue;
                }

                submatrix[submatrix_row_count][submatrix_column_count] = self[row][column];
                submatrix_column_count += 1;
            }
            submatrix_row_count += 1;
        }

        submatrix
    }

    pub fn minor(&self, row: usize, column: usize) -> f64 {
        let submatrix = self.submatrix(row, column);

        submatrix.determinant()
    }

    pub fn cofactor(&self, row: usize, column: usize) -> f64 {
        let minor = self.minor(row, column);

        if (row + column) % 2 == 1 { minor * -1. } else { minor }
    }

    pub fn inverse(&self) -> Option<Self> {
        let read_reference = CACHED_INVERSES.read().unwrap();
        let index = read_reference.iter().position(|ci| ci.id == self.id);

        match index {
            Some(i) => return Some(read_reference[i].clone()),
            None => (),
        };
        
        let determinant = self.determinant();
        if near_eq(determinant, 0.) {
            return None;
        }

        let mut inverse = Matrix::new_inverse(self.id, self.rows, self.columns, 
            vec![0.0; self.rows * self.columns]);
        
        for row in 0..self.rows {
            for column in 0..self.columns {
                let cofactor = self.cofactor(row, column);

                inverse[column][row] = cofactor / determinant;
            }
        }

        drop(read_reference);
        let mut write_reference = CACHED_INVERSES.write().unwrap();
        write_reference.push(inverse.clone());

        Some(inverse)
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

    #[test]
    fn calculating_determinant_of_2x2_matrix() {
        let matrix = Matrix::new(2, 2, vec![1., 5., -3., 2.]);

        let expected = 17.;

        let actual = matrix.determinant();

        assert!(near_eq(expected, actual));
    }

    #[test]
    fn submatrix_of_3x3_matrix_is_2x2_matrix() {
        let values = vec![1., 5., 0., -3., 2., 7., 0., 6., -3.];
        let matrix = Matrix::new(3, 3, values);

        let expected = Matrix::new(2, 2, vec![-3., 2., 0., 6.]);

        let actual = matrix.submatrix(0, 2);

        assert_eq!(expected, actual);
    }

    #[test]
    fn submatrix_of_4x4_matrix_is_3x3_matrix() {
        let values = vec![-6., 1., 1., 6., -8., 5., 8., 6.,
            -1., 0., 8., 2., -7., 1., -1., 1.];
        let matrix = Matrix::new(4, 4, values);

        let expected = Matrix::new(3, 3, vec![-6., 1., 6., -8., 8., 6.,
            -7., -1., 1.]);

        let actual = matrix.submatrix(2, 1);

        assert_eq!(expected, actual);
    }

    #[test]
    fn calculating_minor_of_3x3_matrix() {
        let values = vec![3., 5., 0., 2., -1., -7., 6., -1., 5.];
        let matrix = Matrix::new(3, 3, values);
        let submatrix = matrix.submatrix(1, 0);

        let expected = 25.;

        let actual1 = submatrix.determinant();
        let actual2 = matrix.minor(1, 0);

        assert!(near_eq(expected, actual1));
        assert!(near_eq(expected, actual2));
    }

    #[test]
    fn calculating_cofactor_of_3x3_matrix() {
        let values = vec![3., 5., 0., 2., -1., -7., 6., -1., 5.];
        let matrix = Matrix::new(3, 3, values);

        let expected_minor1 = -12.;
        let expected_cofactor1 = -12.;
        let expected_minor2 = 25.;
        let expected_cofactor2 = -25.;

        let actual_minor1 = matrix.minor(0, 0);
        let actual_cofactor1 = matrix.cofactor(0, 0);
        let actual_minor2 = matrix.minor(1, 0);
        let actual_cofactor2 = matrix.cofactor(1, 0);

        assert!(near_eq(expected_minor1, actual_minor1));
        assert!(near_eq(expected_cofactor1, actual_cofactor1));
        assert!(near_eq(expected_minor2, actual_minor2));
        assert!(near_eq(expected_cofactor2, actual_cofactor2));
    }

    #[test]
    fn calculating_determinant_of_3x3_matrix() {
        let values = vec![1., 2., 6., -5., 8., -4., 2., 6., 4.];
        let matrix = Matrix::new(3, 3, values);

        let expected_cofactor1 = 56.;
        let expected_cofactor2 = 12.;
        let expected_cofactor3 = -46.;
        let expected_determinant = -196.;

        let actual_cofactor1 = matrix.cofactor(0, 0);
        let actual_cofactor2 = matrix.cofactor(0, 1);
        let actual_cofactor3 = matrix.cofactor(0, 2);
        let actual_determinant = matrix.determinant();

        assert!(near_eq(expected_cofactor1, actual_cofactor1));
        assert!(near_eq(expected_cofactor2, actual_cofactor2));
        assert!(near_eq(expected_cofactor3, actual_cofactor3));
        assert!(near_eq(expected_determinant, actual_determinant));
    }

    #[test]
    fn calculating_determinant_of_4x4_matrix() {
        let values = vec![-2., -8., 3., 5., -3., 1., 7., 3.,
            1., 2., -9., 6., -6., 7., 7., -9.];
        let matrix = Matrix::new(4, 4, values);

        let expected_cofactor1 = 690.;
        let expected_cofactor2 = 447.;
        let expected_cofactor3 = 210.;
        let expected_cofactor4 = 51.;
        let expected_determinant = -4071.;

        let actual_cofactor1 = matrix.cofactor(0, 0);
        let actual_cofactor2 = matrix.cofactor(0, 1);
        let actual_cofactor3 = matrix.cofactor(0, 2);
        let actual_cofactor4 = matrix.cofactor(0, 3);
        let actual_determinant = matrix.determinant();

        assert!(near_eq(expected_cofactor1, actual_cofactor1));
        assert!(near_eq(expected_cofactor2, actual_cofactor2));
        assert!(near_eq(expected_cofactor3, actual_cofactor3));
        assert!(near_eq(expected_cofactor4, actual_cofactor4));
        assert!(near_eq(expected_determinant, actual_determinant));
    }

    #[test]
    fn testing_invertible_matrix_for_invertibility() {
        let values = vec![6., 4., 4., 4., 5., 5., 7., 6.,
            4., -9., 3., -7., 9., 1., 7., -6.];
        let matrix = Matrix::new(4, 4, values);

        let expected = -2120.;

        let actual = matrix.determinant();

        assert!(near_eq(expected, actual));
        assert!(matrix.inverse().is_some());
    }

    #[test]
    fn testing_noninvertible_matrix_for_invertibility() {
        let values = vec![-4., 2., -2., -3., 9., 6., 2., 6.,
            0., -5., 1., -5., 0., 0., 0., 0.];
        let matrix = Matrix::new(4, 4, values);

        let expected = 0.;

        let actual = matrix.determinant();

        assert!(near_eq(expected, actual));
        assert!(matrix.inverse().is_none());
    }

    #[test]
    fn calculating_inverse_of_matrix() {
        let values1 = vec![-5., 2., 6., -8., 1., -5., 1., 8.,
            7., 7., -6., -7., 1., -3., 7., 4.];
        let matrix = Matrix::new(4, 4, values1);

        let expected_determinant = 532.;
        let expected_cofactor1 = -160.;
        let expected_3_2 = -160. / 532.;
        let expected_cofactor2 = 105.;
        let expected_2_3 = 105. / 532.;
        let values2 = vec![0.21805, 0.45113, 0.24060, -0.04511, 
            -0.80827, -1.45677, -0.44361, 0.52068,
            -0.07895, -0.22368, -0.05263, 0.19737, 
            -0.52256, -0.81391, -0.30075, 0.30639];
        let expected_matrix = Matrix::new(4, 4, values2);

        let actual_matrix = matrix.inverse().unwrap();
        let actual_determinant = matrix.determinant();
        let actual_cofactor1 = matrix.cofactor(2, 3);
        let actual_3_2 = actual_matrix[3][2];
        let actual_cofactor2 = matrix.cofactor(3, 2);
        let actual_2_3 = actual_matrix[2][3];

        assert!(near_eq(expected_determinant, actual_determinant));
        assert!(near_eq(expected_cofactor1, actual_cofactor1));
        assert!(near_eq(expected_3_2, actual_3_2));
        assert!(near_eq(expected_cofactor2, actual_cofactor2));
        assert!(near_eq(expected_2_3, actual_2_3));
        assert_eq!(expected_matrix, actual_matrix);
    }

    #[test]
    fn calculating_inverse_of_another_matrix() {
        let values1 = vec![8., -5., 9., 2., 7., 5., 6., 1.,
            -6., 0., 9., 6., -3., 0., -9., -4.];
        let matrix = Matrix::new(4, 4, values1);

        let values2 = vec![-0.15385, -0.15385, -0.28205, -0.53846, 
            -0.07692, 0.12308, 0.02564, 0.03077, 
            0.35897, 0.35897, 0.43590, 0.92308, 
            -0.69231, -0.69231, -0.76923, -1.92308];
        let expected = Matrix::new(4, 4, values2);

        let actual = matrix.inverse().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn calculating_inverse_of_third_matrix() {
        let values1 = vec![9., 3., 0., 9., -5., -2., -6., -3.,
            -4., 9., 6., 4., -7., 6., 6., 2.];
        let matrix = Matrix::new(4, 4, values1);

        let values2 = vec![-0.04074, -0.07778, 0.14444, -0.22222,
            -0.07778, 0.03333, 0.36667, -0.33333,
            -0.02901, -0.14630, -0.10926, 0.12963,
            0.17778, 0.06667, -0.26667, 0.33333];

        let expected = Matrix::new(4, 4, values2);

        let actual = matrix.inverse().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn multiplying_product_by_its_inverse() {
        let values1 = vec![3., -9., 7., 3., 3., -8., 2., -9.,
            -4., 4., 4., 1., -6., 5., -1., 1.];
        let matrix1 = Matrix::new(4, 4, values1);
        let values2 = vec![8., 2., 2., 2., 3., -1., 7., 0., 
            7., 0., 5., 4., 6., -2., 0., 5.];
        let matrix2 = Matrix::new(4, 4, values2);
        let matrix3 = matrix1.clone() * matrix2.clone(); 

        let expected = matrix1;

        let actual = matrix3 * matrix2.inverse().unwrap();

        assert_eq!(expected, actual);
    }
}
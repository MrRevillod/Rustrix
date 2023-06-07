
use std::ops::{Add, Mul, Sub};
use crate::utils;

pub struct Matrix {
    array: Vec<Vec<f64>>,
    det: Option<f64>,
    rank: Option<usize>,
    shape: (usize, usize),
}

impl Matrix {

    pub fn new(array: Vec<Vec<f64>>) -> Matrix {

        let shape = (array.len(), array[0].len());

        Matrix {
            array,
            det: None,
            rank: None,
            shape,
        }
    }

    pub fn show(&self) {
        println!();

        for row in &self.array {
            let truncated_row: Vec<String> = row.iter()
                .map(|&e| {
                    if e.fract() == 0.0 {
                        e.to_string()
                    } else {
                        utils::to_frac(e, 3).to_string()
                    }
                })
                .collect();

            println!("[{}]", truncated_row.join(" "));
        }

        println!();
    }



    pub fn is_valid(&self, other: &Matrix, op: &str) -> bool {

        match op {
            "+" | "-" => self.shape == other.shape,
            "x" => self.shape.1 == other.shape.0,
            _ => false,
        }
    }

    pub fn det(&mut self) -> f64 {

        if let Some(det) = self.det {
            return det
        }

        let n = self.shape.0;

        if n == 1 {
            return self.array[0][0]
        }

        let mut det = 1.0;
        let mut array = self.array.clone();

        for i in 0..n {

            let mut pivot = array[i][i];

            if pivot == 0.0 {

                for j in (i + 1)..n {
                    if array[j][i] != 0.0 {
                        array.swap(i, j);
                        det *= -1.0;
                        break;
                    }
                }

                if array[i][i] == 0.0 {
                    return 0.0;
                }

                pivot = array[i][i];
            }

            for j in (i + 1)..n {
                let factor = array[j][i] / pivot;
                for k in i..n {
                    array[j][k] -= array[i][k] * factor;
                }
            }

            det *= pivot;
        }

        det = (det * 100.0).round() / 100.0;
        self.det = Some(det);
        self.det.unwrap()
    }

    pub fn transposed(&self) -> Matrix {

        let rows = self.shape.0;
        let cols = self.shape.1;
        let mut res = self.array.clone();

        for i in 0..rows {
            for j in 0..cols {
                res[j][i] = self.array[i][j];
            }
        }

        Matrix::new(res)
    }

    pub fn minor(&self, i: usize, j: usize) -> Matrix {

        let rows = self.shape.0;
        let cols = self.shape.1;
        let mut res = vec![vec![0.0; cols - 1]; rows - 1];
        let mut i_new = 0;

        for r in 0..rows {
            if r + 1 == i {
                continue;
            }
            let mut j_new = 0;
            for c in 0..cols {
                if c + 1 == j {
                    continue;
                }
                res[i_new][j_new] = self.array[r][c];
                j_new += 1;
            }

            i_new += 1;
        }

        Matrix::new(res)
    }

    pub fn cof(&self, i: usize, j: usize) -> f64 {
        let mut minor = self.minor(i, j);
        let sign = if (i + j) % 2 == 0 {1.0} else {-1.0};
        return sign * minor.det()
    }

    pub fn cofactors(&self) -> Matrix {
        let rows = self.shape.0;
        let cols = self.shape.1;

        if rows == 1 && cols == 1 {
            return Matrix::new(vec![vec![1.0]])
        }


        let mut cofactors = vec![vec![0.0; cols]; rows];

        for r in 0..rows {
            for c in 0..cols {
                cofactors[r][c] = self.cof(r + 1, c + 1);
            }
        }

        Matrix::new(cofactors)
    }

    pub fn inv(&mut self) -> Matrix {
        let det = self.det();
        let cofactors = self.cofactors();
        let inv = cofactors.transposed() * (1.0 / det);
        inv
    }

}

impl Add<Matrix> for Matrix {

    type Output = Matrix;

    fn add(self, other: Matrix ) -> Matrix {

        let rows = self.shape.0;
        let cols = self.shape.1;
        let mut res = vec![vec![0.0; cols]; rows];

        for m in 0..rows {
            for n in 0..cols {
                res[m][n] = self.array[m][n] + other.array[m][n];
            }
        }

        Matrix::new(res)
    }
}

impl Sub<Matrix> for Matrix {

    type Output = Matrix;

    fn sub(self, other: Matrix ) -> Matrix {

        let rows = self.shape.0;
        let cols = self.shape.1;
        let mut res = vec![vec![0.0; cols]; rows];

        for m in 0..rows {
            for n in 0..cols {
                res[m][n] = self.array[m][n] - other.array[m][n];
            }
        }

        Matrix::new(res)
    }
}

impl Mul<Matrix> for Matrix {

    type Output = Matrix;

    fn mul(self, other: Matrix ) -> Matrix {

        let rows = self.shape.0;  // Matríz A rows
        let cols = other.shape.1; // Matríz B cols

        let mut res = vec![vec![0.0; rows]; cols];

        for i in 0..rows {
            for j in 0..cols {
                for k in 0..other.shape.0 {
                    res[i][j] += self.array[i][k] * other.array[k][j];
                }
            }
        }

        Matrix::new(res)
    }
}

impl Mul<f64> for Matrix {

    type Output = Matrix;

    fn mul(self, scalar: f64) -> Matrix {

        let mut res = self.array;
        for row in &mut res {
            for element in row {
                *element *= scalar;
            }
        }

        Matrix::new(res)
    }
}







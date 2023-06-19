use std::ops::Mul;
use std::result::Result;
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
        utils::show_array(&self.array);
    }

    pub fn det(&mut self) -> f64 {
        if let Some(det) = self.det {
            return det
        }

        let rows = self.shape.0;
        let mut det = 1.0;
        let mut array = self.array.clone();

        if rows == 1 {
            return self.array[0][0]
        }

        for i in 0..rows {
            let mut pivot = array[i][i];

            if pivot == 0.0 {

                for j in (i + 1)..rows {
                    if array[j][i] != 0.0 {
                        array.swap(i, j);
                        det *= -1.0;

                        println!("cambio de fila {} y {}", i + 1, j + 1);
                        break;
                    }
                }

                if array[i][i] == 0.0 {
                    return 0.0
                }

                pivot = array[i][i];
            }

            for j in (i + 1)..rows {
                let factor = -array[j][i] / pivot;
                for k in i..rows {
                    array[j][k] += array[i][k] * factor;
                    println!("TL Fila {} + Fila {} * {}", j + 1, i + 1, factor);
                }
            }

            det *= pivot;
        }

        utils::show_array(&array);

        self.det = Some(det);
        self.det.unwrap()
    }


    pub fn rank(&mut self) -> usize {
        if let Some(rank) = self.rank {
            return rank;
        }

        let (rows, cols) = self.shape;
        let min_dim = rows.min(cols) as usize;

        for size in (1..=min_dim).rev() {
            for row in 0..=rows - size {
                for col in 0..=cols - size {
                    let mut submatrix = self.submatrix(row, col, size);
                    if submatrix.det() != 0.0 {
                        self.rank = Some(size);
                        return size;
                    }
                }
            }
        }

        self.rank = Some(0);
        return 0
    }

    fn submatrix(&self, start_row: usize, start_col: usize, size: usize) -> Matrix {
        let mut array = vec![vec![0.0; size]; size];
        for i in 0..size {
            for j in 0..size {
                array[i][j] = self.array[start_row + i][start_col + j];
            }
        }
        Matrix::new(array)
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
        let sign = if (i + j) % 2 == 0 { 1.0 } else { -1.0 };
        return sign * minor.det();
    }

    pub fn cofactors(&self) -> Matrix {
        let rows = self.shape.0;
        let cols = self.shape.1;

        if rows == 1 && cols == 1 {
            return Matrix::new(vec![vec![1.0]]);
        }

        let mut cofactors = vec![vec![0.0; cols]; rows];

        for r in 0..rows {
            for c in 0..cols {
                cofactors[r][c] = self.cof(r + 1, c + 1);
            }
        }

        Matrix::new(cofactors)
    }

    pub fn inv(&mut self) -> Result<Matrix, String> {
        let det = self.det();

        if det == 0.0 || det.abs() < 1e-10 {
            return Err("Matríz singular. ".to_string())
        }

        let cofactors = self.cofactors();
        let inv = cofactors.transposed() * (1.0 / det);
        return Ok(inv)
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

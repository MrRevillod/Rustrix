use std::ops::Mul;

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
            print!(" [ ");
            for value in row {
                print!(" {:.2} ", value);
            }
            println!("]");
        }
        println!();
    }

    pub fn det(&mut self) -> f64 {
        if let Some(det) = self.det {
            return det;
        }

        let n = self.shape.0;

        if n == 1 {
            return self.array[0][0];
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

        self.det = Some(det);
        self.det.unwrap()
    }

/*

    pub fn rank(&mut self) -> usize {
        if let Some(rank) = self.rank {
            return rank;
        }

        let mut array = self.array.clone();
        let rows = self.shape.0;
        let cols = self.shape.1;

        if rows == 1 || cols == 1 {
            let rank = array.iter().flatten().filter(|&&x| x != 0.0).count();
            self.rank = Some(rank);
            return rank;
        }

        let mut rank = 0;

        for r in 0..rows {
            if rank >= cols {
                break;
            }
            let mut pivot = array[r][rank];

            if pivot == 0.0 {
                for j in (r + 1)..rows {
                    if array[j][rank] != 0.0 {
                        array.swap(r, j);
                        break;
                    }
                }

                if array[r][rank] == 0.0 {
                    continue;
                }
                pivot = array[r][rank];
            }

            for j in (r + 1)..rows{
                let factor = array[j][rank] / pivot;
                for k in rank..cols {
                    array[j][k] -= array[r][k] * factor;
                }
            }

            rank += 1;
        }

        self.rank = Some(rank);
        rank
    }

*/


    pub fn debug_rank(&mut self) -> usize {
        if let Some(rank) = self.rank {
            return rank;
        }

        let mut array = self.array.clone();
        let rows = self.shape.0;
        let cols = self.shape.1;

        if rows == 1 || cols == 1 {
            let rank = array.iter().flatten().filter(|&&x| x != 0.0).count();
            self.rank = Some(rank);
            return rank;
        }

        let mut rank = 0;

        for r in 0..rows {
            if rank >= cols {
                break;
            }

            let mut pivot = array[r][rank];

            if pivot == 0.0 {
                println!(" [-] Pivote cero encontrado en la fila {}, buscando un pivote no nulo\n", r + 1);

                for j in (r + 1)..rows {
                    if array[j][rank] != 0.0 {
                        println!(" [-] Intercambiando fila {} con fila {}", r + 1, j + 1);
                        array.swap(r, j);
                        break;
                    }
                }

                if array[r][rank] == 0.0 {
                    println!("No se encontró un pivote no nulo, pasando a la siguiente iteración...");
                    continue;
                }
                pivot = array[r][rank];
            }

            println!(" [-] Iteración {}: Pivote encontrado en la fila {}", rank + 1, r + 1);

            for j in (r + 1)..rows {
                let factor = -array[j][rank] / pivot;
                println!(" [-] Eliminación hacia abajo : Fila {} + Fila {} * {}", j + 1, r + 1, factor);
                for k in rank..cols {
                    array[j][k] += array[r][k] * factor;
                }
            }

            rank += 1;
        }

        println!("\n Matríz final: ");
        let matrix = Matrix::new(array);
        matrix.show();

        self.rank = Some(rank);
        rank
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

    pub fn inv(&mut self) -> Matrix {
        let det = self.det();

        let cofactors = self.cofactors();
        let inv = cofactors.transposed() * (1.0 / det);
        return inv;
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

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
                }
            }

            det *= pivot;
        }

        self.det = Some(det);
        self.det.unwrap()
    }

    pub fn rank(&mut self) -> usize {
        if let Some(rank) = self.rank {
            return rank
        }

        let (rows, cols) = self.shape;
        let max_dim = rows.min(cols) as usize;

        println!(" Rango máximo: {}", max_dim);

        for size in (1..=max_dim).rev() {
            println!(" Tamaño de la submatriz actual: {}\n", size);
            for row in 0..=rows - size {
                println!(" Fila: {}, resta: {}", row, rows - size);
                for col in 0..=cols - size {
                    println!(" Columna: {}, resta: {}\n", col, cols - size);
                    println!(" Posición de inicio de la submatriz: ({}, {})", row, col);
                    let mut submatrix = self.submatrix(row, col, size);
                    println!(" Submatriz actual:");
                    submatrix.show();
                    if submatrix.det() != 0.0 {
                        println!(" [-] Determinante != 0 encontrado...");
                        self.rank = Some(size);
                        return size
                    }
                    println!(" [-] Determinante == 0 encontrado...");
                }
            }
        }

        self.rank = Some(0);
        return 0
    }

    fn submatrix(&self, row: usize, col: usize, size: usize) -> Matrix {
        let mut array = vec![vec![0.0; size]; size];
        for i in 0..size {
            for j in 0..size {
                array[i][j] = self.array[row + i][col + j];
            }
        }

        Matrix::new(array)
    }

    pub fn info() {
        println!(
            "
                Se recorren las filas. Si el pivot es cero:
                se busca un pivote no nulo por las filas i + 1

                Si se encuentra un pivote no nulo se hace swap
                y se cambia el signo del determinante.


                Luego de verificar lo anterior se recorren las filas
                por debajo del pivote en busca de la eliminación de gauss

                Se calcula el factor (eliminación) y se recorren
                nuevamente las filas, por cada elemento en la fila
                a eliminar se le sima su elemento superior * el factor de eliminación
            "
        );
    }
}

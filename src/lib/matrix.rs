
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
        return self.det.unwrap()
    }

    pub fn rank(&mut self) -> usize {
        if let Some(rank) = self.rank {
            return rank
        }

        let (rows, cols) = self.shape;
        let max_dim = rows.min(cols) as usize;

        println!(" Rango máximo: {}", max_dim);

        /* Bucle que recorre todas las dim de submatrices desde max_dim hasta 1.*/

        for size in (1..=max_dim).rev() {
            println!(" Tamaño de la submatriz actual: {}\n", size);

            /* Bucles que recorren rows y cols de la matriz en busca de una submatríz*/
            /*    Se usa el rango maximo de rows-size para evitar desbordamiento     */

            for row in 0..=rows - size {
                for col in 0..=cols - size {
                    println!(" Posición de inicio de la submatriz: ({}, {})", row, col);

                    /*    Se crea una submatríz cuadrada de mayor orden posible   */

                    let mut submatrix = self.submatrix(row, col, size);

                    println!(" Submatriz actual:");
                    submatrix.show();

                    /*  Se comprueba que el determinante de la submatríz sea != 0 */

                    if submatrix.det() != 0.0 {
                        println!(" [-] Determinante != 0 encontrado...");
                        self.rank = Some(size);
                        return size
                    }

                    /*          Si el det es != 0 se retorna el tamaño (size)         */
                    /* De lo contrario se pasa a la sig iteración del ciclo principal */

                    println!(" [-] Determinante == 0 encontrado...\n");
                }
            }
        }

        self.rank = Some(0);
        return 0
    }

    /* Función submatrix*/
    /* Recibe las coordenadas de fila y columna de inicio y el tamaño de la nueva matríz */

    fn submatrix(&self, row: usize, col: usize, size: usize) -> Matrix {

        /* Se crea una matriz de tamaño sizeXsize llena de 0 */

        let mut array = vec![vec![0.0; size]; size];

        /* Se itera en rango de las dimenciones establecidas */

        for i in 0..size {
            for j in 0..size {

                /* La nueva matríz estará formada por los elementos de la matríz og         */
                /* pero con los elementos de las posiciones establecidas por los parametros */

                array[i][j] = self.array[row + i][col + j];
            }
        }

        return Matrix::new(array)
    }
}

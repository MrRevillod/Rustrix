
struct Matrix {
    array: Vec<Vec<f64>>,
    det: Option<f64>,
    rank: Option<usize>,
    shape: (usize, usize),
}

impl Matrix {

    fn new(array: Vec<Vec<f64>>) -> Matrix {

        let shape = (array.len(), array[0].len());
        Matrix {
            array,
            det: None,
            rank: None,
            shape,
        }
    }

    fn show(&self) {
        println!();
        for row in &self.array {
            println!("  [{}]", row.iter().map(|&e| e.to_string()).collect::<Vec<String>>().join(" "));
        }
        println!();
    }

    fn get_det(&mut self) -> f64 {

        if let Some(det) = self.det {
            return det;
        }

        let det = self.calculate_det();
        self.det = Some(det);
        return det
    }

    fn calculate_det(&mut self) -> f64 {

        let n = self.shape.0;
        let mut det = 1.0;

        let mut array = self.array.clone();

        for i in 0..n {

            // Si el pivote es 0, intercambiar filas
            // por lo tanto segun esa trans elemental
            // se debe cambiar el signo del det

            let mut pivot = array[i][i];

            if pivot == 0.0 {
                for j in (i + 1)..n {
                    if array[j][i] != 0.0 {
                        array.swap(i, j);
                        det *= -1.0;
                        break;
                    }
                }

                // Si todas las filas debajo del pivote son nulas
                // el determinante es 0
                // Se asume esto ya que no hay transformacion que cambie el pivot

                if array[i][i] == 0.0 {
                    return 0.0;
                }

                pivot = array[i][i];
            }

            // Eliminar elementos por debajo del pivote
            // En este paso se aplican tranformaciones del 3er
            // tipo o sea no alteran el determinante


            for j in (i + 1)..n {
                let factor = array[j][i] / pivot;
                for k in i..n {
                    array[j][k] -= array[i][k] * factor;
                }
            }

            // Se multiplica el determinante actual por el pivote
            // presente en la diagonal principal

            det *= pivot;
        }

        det = (det * 100.0).round() / 100.0;
        self.det = Some(det);

        return det
    }
}







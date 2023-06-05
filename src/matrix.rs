
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
            println!("  [{}]",
                     row.iter()
                     .map(|&e| e.to_string())
                     .collect::<Vec<String>>()
                     .join(" ")
            );
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

}




use crate::Matrix;
use crate::utils;

pub struct Menu;

impl Menu {

    pub fn new() -> Menu {
        Menu
    }

    pub fn run(&mut self) {

        /* Inserta aquí la matríz */

        let array = vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
        ];

        let shape = (array.len() as u32, array[0].len() as u32);
        let mut matrix = Matrix::new(array);

        utils::clear_term();

        println!("\n [-] Tu matriz es:");
        matrix.show();

        self.get_det(&mut matrix, shape);
        self.get_rank(&mut matrix);
    }

    pub fn get_det(&self, matrix: &mut Matrix, shape: (u32, u32)) {
        if shape.0 != shape.1 {
            println!("\n[-] Para calcular el determinante, la matriz debe ser cuadrada.\n");
            return;
        }

        let det = matrix.det();
        println!(" [-] El determinante de la matriz es: {}\n", det);
    }

    pub fn get_rank(&self, matrix: &mut Matrix) {
        let rank = matrix.rank();
        println!(" [-] El rango de la matriz es: {}\n", rank);
    }
}


mod lib {
    pub mod matrix;
    pub mod utils;
}

use lib::matrix::Matrix;
use lib::utils;

fn get_matrix() -> Matrix {
    /* Inserta aquí la matriz */

    let array = vec![
        vec![32.0, 8.0, 11.0],
        vec![8.0, 20.0, 17.0],
        vec![11.0, 17.0, 14.0],
    ];

    Matrix::new(array)
}

fn get_det(matrix: &mut Matrix) {
    match matrix.det() {
        Ok(det) => {
            println!("\n [-] El determinante de la matriz es: {}\n", det);
        }
        Err(msg) => {
            println!("\n [-] Error: {}\n", msg);
        }
    }
}

fn get_rank(matrix: &mut Matrix) {
    let rank = matrix.rank();
    println!(" [-] El rango de la matriz es: {}\n", rank);
}

fn main() {
    utils::clear_term();
    let mut x = get_matrix();

    get_det(&mut x);
    get_rank(&mut x);
}

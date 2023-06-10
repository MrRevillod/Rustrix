use crate::Matrix;
use std::io::{self, Write};
use std::process;

pub fn clear_term() {
    if cfg!(target_os = "windows") {
        let _ = process::Command::new("cmd").arg("/c").arg("cls").status();
    } else {
        let _ = process::Command::new("clear").status();
    }
}

pub fn get_integer(message: &str) -> i32 {
    loop {
        print!("{}", message);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<i32>() {
            Ok(num) => return num,
            Err(_) => {
                println!("\n [-] Error: Debes ingresar un número entero.\n");
            }
        }
    }
}

pub fn get_float(message: &str) -> f64 {
    loop {
        print!("{}", message);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let trimmed_input = input.trim();

        if let Ok(num) = trimmed_input.parse::<f64>() {
            return num;
        } else if let Ok(num) = trimmed_input.parse::<i32>() {
            return num as f64;
        } else {
            println!("\n [-] Error: Debes ingresar un número.\n");
        }
    }
}

pub fn get_dimensions() -> (usize, usize) {
    loop {
        let rows = get_integer(" [-] Cantidad de filas de tu matriz: ") as usize;
        let cols = get_integer(" [-] Cantidad de columnas de tu matriz: ") as usize;
        println!(" ");

        if rows | cols != 0 {
            return (rows, cols);
        } else {
            println!("\n [-] Error: Has ingresado un número invalido.");
        }
    }
}

pub fn create_matrix(shape: (usize, usize)) -> Matrix {
    let mut array: Vec<Vec<f64>> = Vec::new();

    for i in 0..shape.0 {
        let mut row: Vec<f64> = Vec::new();

        for j in 0..shape.1 {
            let element = get_float(&format!(" [»] Ingrese el elemento ({},{}): ", i + 1, j + 1));
            row.push(element);
        }

        array.push(row);
    }

    println!(" ");

    Matrix::new(array)
}

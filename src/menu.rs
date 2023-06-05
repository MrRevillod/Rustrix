use dialoguer::{Input, Select};
use std::process;
use std::io::{self, Write};

use crate::Matrix;

pub struct Menu {
    run: bool,
}

impl Menu {

    pub fn new() -> Menu {
        Menu { run: false }
    }

    pub fn create_menu(&self) -> usize {
        self.clear_term();

        let options = &[
            " [1]. Calcular Determinante           ",
            " [2]. Calcular Rango                  ",
            " [3]. Calcular Inversa                ",
            " [4]. Calcular Transpuesta            ",
            " [5]. Salir                           ",
        ];

        let select = Select::new()
            .with_prompt("Seleccione una opción")
            .items(options)
            .interact()
            .unwrap();

        select
    }

    pub fn run(&mut self) {

        loop {
            let select = self.create_menu();
            self.clear_term();

            match select {
                0 => self.get_det(),
                1 => self.get_det(),
                2 => self.get_det(),
                3 => self.get_trans(),
                4 => {
                    self.bye();
                    break;
                }
                _ => {
                    println!(" [-] Opción inválida");
                }
            }

            let repeat: String = Input::new()
                .with_prompt(" [?] ¿Deseas realizar otra operación? (s/n): ")
                .interact()
                .unwrap();

            if repeat.to_ascii_lowercase() != "s" {
                self.bye();
                break;
            } else {
                self.clear_term();
            }
        }
    }

    pub fn bye(&self) {
        println!("\n [-] ¡Hasta luego!\n");
    }

    pub fn clear_term(&self) {

        if cfg!(target_os = "windows") {
            let _ = process::Command::new("cmd").arg("/c").arg("cls").status();
        } else {
            let _ = process::Command::new("clear").status();
        }
    }

    pub fn get_integer(&self, message: &str) -> i32 {

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

    pub fn get_float(&self, message: &str) -> f64 {

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

    pub fn get_cant(&self) -> i32 {
        let num = self.get_integer(" [-] Cant. de matrices con las que vas a operar: ");
        println!(" ");

        num
    }

    pub fn get_dimensions(&self) -> (usize, usize) {
        let rows = self.get_integer(" [-] Cantidad de filas de tu matriz: ") as usize;
        let cols = self.get_integer(" [-] Cantidad de columnas de tu matriz: ") as usize;
        println!(" ");

        (rows, cols)
    }

    pub fn create_matrix(&self, shape: (usize, usize)) -> Matrix {
 
        let mut array: Vec<Vec<f64>> = Vec::new();

        for i in 0..shape.0 {
            let mut row: Vec<f64> = Vec::new();

            for j in 0..shape.1 {
                let element = self.get_float(&format!(" [»] Ingrese el elemento ({},{}): ", i + 1, j + 1));
                row.push(element);
            }

            array.push(row);
        }

        println!(" ");

        Matrix::new(array)
    }

    pub fn get_det(&self) {

        let shape = self.get_dimensions();
        if shape.0 != shape.1 {
            println!("\n [-] Para calcular determinante la matríz debe ser cuadrada. \n");
            return;
        }

        let mut matrix = self.create_matrix(shape);
        let det = matrix.det();

        println!(" [-] El determinante de la matríz es: {}", det);
    }

    pub fn get_trans(&self) {

        let shape = self.get_dimensions();
        let matrix = self.create_matrix(shape);

        let trans = matrix.transposed();

        trans.show();
    }
}




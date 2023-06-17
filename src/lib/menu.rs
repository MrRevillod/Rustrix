use crate::utils;
use dialoguer::{Input, Select};

pub struct Menu;

impl Menu {
    pub fn new() -> Menu {
        Menu
    }

    pub fn create_menu(&self) -> usize {
        utils::clear_term();

        let options = &[
            " [1]. Calcular Determinante           ",
            " [2]. Calcular Rango                  ",
            " [3]. Calcular Inversa                ",
            " [4]. Salir                           ",
        ];

        let select = Select::new()
            .with_prompt("\n Seleccione una opción")
            .items(options)
            .interact()
            .unwrap();

        select
    }

    pub fn run(&mut self) {
        loop {
            let select = self.create_menu();
            utils::clear_term();

            match select {
                0 => self.get_det(),
                1 => self.get_rank(),
                2 => self.get_inv(),
                3 => {
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

            if repeat.to_lowercase() != "s" {
                self.bye();
                break;
            } else {
                utils::clear_term();
            }
        }
    }

    pub fn bye(&self) {
        println!("\n [-] ¡Hasta luego!\n");
    }

    pub fn get_det(&self) {
        let shape = utils::get_dimensions();
        if shape.0 != shape.1 {
            println!("\n [-] Para calcular determinante la matríz debe ser cuadrada. \n");
            return;
        }

        let mut matrix = utils::create_matrix(shape);
        matrix.show();
        let det = matrix.det();
        println!(" [-] El determinante de la matríz es: {}", det);
    }

    pub fn get_rank(&self) {
        let shape = utils::get_dimensions();
        let mut matrix = utils::create_matrix(shape);

        matrix.show();
        let rank = matrix.rank();
        println!(" [-] El rango de la matríz es: {}", rank);
    }

    pub fn get_inv(&self) {
        let shape = utils::get_dimensions();
        if shape.0 != shape.1 {
            println!("\n [-] Para calcular la inversa, la matriz debe ser cuadrada. \n");
            return;
        }

        let mut matrix = utils::create_matrix(shape);
        matrix.show();
        let inverse = matrix.inv();

        match inverse {
            Ok(inv) => {
                println!(" [-] La inversa de la matriz es:");
                inv.show();
            }
            Err(err) => {
                println!(" [-] Error al calcular la inversa: {}", err);
            }
        }
    }
}

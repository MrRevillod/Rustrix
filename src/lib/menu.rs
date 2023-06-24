use crate::utils;
use dialoguer::{Input, Select};

pub struct Menu;

impl Menu {
    pub fn new() -> Menu {
        Menu
    }

    pub fn create_menu(&self) -> usize {
        utils::clear_term();
        println!("\n");

        let options = &[
            " [1]. Calcular Determinante           ",
            " [2]. Calcular Rango                  ",
            " [3]. Salir                           ",
        ];

        let select = Select::new()
            .items(options)
            .interact()
            .unwrap();

        println!();
        select
    }

    pub fn run(&mut self) {
        loop {
            let select = self.create_menu();
            utils::clear_term();

            match select {
                0 => self.get_det(),
                1 => self.get_rank(),
                2 => {
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
        println!(" Tu matríz es: ");
        matrix.show();
        let det = matrix.det();
        println!(" [-] El determinante de la matríz es: {}", det);
    }

    pub fn get_rank(&self) {
        let shape = utils::get_dimensions();
        let mut matrix = utils::create_matrix(shape);

        println!(" Tu matríz es: ");
        matrix.show();
        let rank = matrix.rank();
        println!(" [-] El rango de la matríz es: {}", rank);
    }
}

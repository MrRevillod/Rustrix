use std::process::Command;

pub fn show_array(array: &Vec<Vec<f64>>) {
    println!();
    for row in array {
        print!(" [ ");
        for value in row {
            print!(" {:.2} ", value);
        }
        println!("]");
    }
    println!();
}

pub fn clear_term() {
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd").arg("/c").arg("cls").status();
    } else {
        let _ = Command::new("clear").status();
    }
}

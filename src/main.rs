
fn main() {

    let array = vec![
        vec![1.1, 2.5],
        vec![3.0, 0.6],
    ];

    let mut matrix = Matrix::new(array);

    matrix.show();
    let det = matrix.get_det();
    println!("Determinante: {}", det);
}

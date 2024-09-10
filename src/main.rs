mod matrix;
use matrix::Matrix;

fn main() {
    let mut matrix = Matrix::new(vec![vec![1, 0, 1], vec![0, 1, 1], vec![1, 0, 1]]);

    let (result_matrix, operations) = matrix.echf_2();
    println!("{:?}", result_matrix.elements);
    println!("{:?}", operations);
}

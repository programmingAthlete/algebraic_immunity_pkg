mod vector;

use vector::GF2Vector;

struct GF2Matrix {
    elements: Vec<GF2Vector>,
}

impl GF2Matrix {
    fn new(elements: Vec<GF2Vector>) -> Self {
        GF2Matrix { elements }
    }
}

fn main() {
    let v1 = GF2Vector::new(vec![1, 0, 1]);
    let v2 = GF2Vector::new(vec![0, 1, 0]);
    println!("{:?}", v1);
    println!("{:?}", v2);
    let v3 = vector_sum(&v1, &v2);
    println!("{:?}", v3);
    print!("{}", v1);

    let ops = [(0, 1), (1, 1)];
    let m = GF2Matrix::new(vec![v1, v2]);
    print!("{}", m.elements[0]);
}

fn vector_sum(v1: &GF2Vector, v2: &GF2Vector) -> GF2Vector {
    assert!(
        v1.elements.len() == v2.elements.len(),
        "Vectors must be of the same lengh"
    );

    let mut result: Vec<u8> = Vec::new();

    for i in 0..v1.elements.len() {
        result.push((v1.elements[i] + v2.elements[i]) % 2);
    }

    let res: Vec<u8> = v1
        .elements
        .iter()
        .zip(v2.elements.iter())
        .map(|(&a, &b)| (a + b) % 2)
        .collect();
    assert!(result == res);

    GF2Vector::new(res)
}

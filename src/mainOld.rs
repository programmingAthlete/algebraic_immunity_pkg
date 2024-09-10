mod vector;

use vector::GF2Vector;

pub struct GF2Matrix {
    pub elements: Vec<GF2Vector>,
}

impl GF2Matrix {
    pub fn new(elements: Vec<GF2Vector>) -> Self {
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

fn echelon_form_part_matrix(m: &GF2Matrix) {
    let l = *m.elements.last().unwrap();
    for i in 0..l.elements.len() {
        let mut p_index = l.elements.len() + 1;
        for (j, &value) in l.elements.iter().enumerate() {
            if (value == 1) {
                p_index = j;
                break;
            }
        }
        let mut p_row: GF2Vector;
        let mut j_index;
        for j in 0..(m.elements.len() - 1) {
            if m.elements[j].elements[p_index] == 1
                && (0..p_index).all(|k| m.elements[j].elements[k] != 1)
            {
                p_row = GF2Vector::new(Some(m.elements[j].elements.clone()).unwrap());
                j_index = Some(j);
            }
        }
        println!("Index: {}, Value: {}", i, l.elements[i]);
    }
}

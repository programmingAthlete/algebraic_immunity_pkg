use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone)]
pub struct Matrix {
    pub elements: Vec<Vec<u8>>,
}

impl Matrix {
    pub fn new(elements: Vec<Vec<u8>>) -> Self {
        Matrix { elements }
    }

    fn nrows(&self) -> usize {
        self.elements.len()
    }

    fn ncols(&self) -> usize {
        if !self.elements.is_empty() {
            self.elements[0].len()
        } else {
            0
        }
    }

    fn copy(&self) -> Self {
        self.clone()
    }

    fn get(&self, row: usize, col: usize) -> u8 {
        self.elements[row][col]
    }

    fn get_pivot(row: &Vec<u8>) -> Option<usize> {
        row.iter().position(|&x| x == 1)
    }

    fn add_rows(&mut self, target: usize, source: usize) {
        for i in 0..self.ncols() {
            self.elements[target][i] ^= self.elements[source][i];
        }
    }

    fn swap_rows(&mut self, row1: usize, row2: usize) {
        self.elements.swap(row1, row2);
    }

    fn is_zero_row(&self, row: usize) -> bool {
        self.elements[row].iter().all(|&x| x == 0)
    }

    pub fn echf_2(&mut self) -> (Self, Vec<(usize, usize)>) {
        let mut m_copy = self.copy();
        let mut last_row = m_copy.elements[m_copy.nrows() - 1].clone();
        let last_row_index = m_copy.nrows() - 1;
        let mut operations = Vec::new();

        for _ in 0..m_copy.ncols() {
            let p_index = Matrix::get_pivot(&last_row);
            if p_index.is_none() {
                break;
            }
            let p_index = p_index.unwrap();

            let mut p_row: Option<Vec<u8>> = None;
            let mut j_index: Option<usize> = None;

            for j in 0..m_copy.nrows() - 1 {
                if m_copy.get(j, p_index) == 1 && !(0..p_index).any(|k| m_copy.get(j, k) == 1) {
                    p_row = Some(m_copy.elements[j].clone());
                    j_index = Some(j);
                }
            }

            if p_row.is_none() {
                if p_index == last_row_index {
                    let mut swap_index_row: Option<usize> = None;
                    for r in 0..m_copy.nrows() - 1 {
                        if m_copy.is_zero_row(r) {
                            swap_index_row = Some(r);
                            break;
                        }
                    }

                    if let Some(swap_index_row) = swap_index_row {
                        m_copy.swap_rows(last_row_index, swap_index_row);
                        operations.push((swap_index_row, last_row_index));
                        operations.push((last_row_index, swap_index_row));
                        operations.push((swap_index_row, last_row_index));
                    }
                    break;
                }
                m_copy.swap_rows(last_row_index, p_index);
                last_row = m_copy.elements[last_row_index].clone();
                operations.push((p_index, last_row_index));
                operations.push((last_row_index, p_index));
                operations.push((p_index, last_row_index));
            } else if p_row.unwrap()[p_index] == 1 {
                m_copy.add_rows(last_row_index, j_index.unwrap());
                last_row = m_copy.elements[last_row_index].clone();
                operations.push((last_row_index, j_index.unwrap()));
            }
        }

        (m_copy, operations)
    }
}

#[pymodule]
fn algebraic_immunity_pkg(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Matrix>()?;
    Ok(())
}

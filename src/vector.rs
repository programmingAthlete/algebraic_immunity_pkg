use std::fmt::Display;

#[derive(Debug)]
pub struct GF2Vector {
    pub elements: Vec<u8>,
}

impl GF2Vector {
    pub fn new(elements: Vec<u8>) -> Self {
        assert!(elements.iter().all(|&x| x == 0 || x == 1));
        GF2Vector { elements }
    }
}

impl Display for GF2Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let elements_str = self
            .elements
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",");
        write!(f, "GF2Vector: [{}]", elements_str)
    }
}

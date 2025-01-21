use crate::vector::Vects;

pub struct VectorStore {
    vectors: Vec<Vects>,
}

impl VectorStore {
    pub fn new() -> Self {
        Self {
            vectors: Vec::new(),
        }
    }

    pub fn push(&mut self, vector: Vects) {
        self.vectors.push(vector);
    }
}

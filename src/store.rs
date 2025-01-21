use crate::vector::Vects;
use std::{borrow::Borrow, error::Error};

#[derive(Debug)]
pub struct VectorStore {
    pub vectors: Vec<Vects>,
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

    pub fn get_by_id(&self, id: i32) -> Result<&Vects, String> {
        self.vectors
            .iter()
            .find(|v| v.id == id)
            .ok_or_else(|| format!("id {id:?} not found"))
    }

    pub fn remove_by_id(&mut self, id: i32) -> Result<(), String> {
        let hit = self
            .vectors
            .iter()
            .position(|v| v.id == id)
            .ok_or_else(|| format!("id {id:?} not found"))
            .unwrap();

        self.vectors.remove(hit);

        Ok(())
    }
}

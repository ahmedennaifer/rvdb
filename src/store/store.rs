use crate::ops::metrics::cosine_similarity;
use crate::vector::vector::Vects;

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
    pub fn k_nearest_neighbor(&self, embedding: &Vects, top_k: i32) -> Vec<(f32, Vects)> {
        // loop through all vects
        // calc cosine_sim of input embds and vects
        // return top N vects with score
        let mut res = Vec::new();
        for vec in &self.vectors {
            let sim = cosine_similarity(vec, embedding);
            res.push((sim, vec.clone()));
        }
        res.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        res.truncate(top_k as usize);
        res
    }
    pub fn search_by_keyword(&self, keyword: String) -> Result<Vec<&Vects>, String> {
        let hit: Vec<&Vects> = self
            .vectors
            .iter()
            .filter(|v| v.chunk.contains(&keyword))
            .collect();

        if hit.is_empty() {
            Err(format!("No matches found for keyword: {}", &keyword))
        } else {
            println!("Found {} hits for keyword {}", hit.len(), &keyword);
            Ok(hit)
        }
    }
}

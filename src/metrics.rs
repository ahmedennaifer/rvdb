use crate::vector::Vects;

pub fn euclidean_distance(v1: &Vects, v2: &Vects) -> f32 {
    v1.embeddings
        .iter()
        .zip(v2.embeddings.iter())
        .map(|(a, b)| (a - b) * (a - b))
        .sum::<f32>()
        .sqrt()
}

pub fn dot_product(v1: &Vects, v2: &Vects) -> f32 {
    v1.embeddings
        .iter()
        .zip(v2.embeddings.iter())
        .map(|(a, b)| a * b)
        .sum()
}

pub fn cosine_similarity(v1: &Vects, v2: &Vects) -> f32 {
    1.0 // TODO: Implement
}

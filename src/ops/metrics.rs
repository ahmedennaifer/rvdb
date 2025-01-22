use crate::vector::vector::Vects;

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
    let dot = dot_product(v1, v2);

    let norm1 = v1.embeddings.iter().map(|x| x * x).sum::<f32>().sqrt();

    let norm2 = v2.embeddings.iter().map(|x| x * x).sum::<f32>().sqrt();

    dot / (norm1 * norm2)
}

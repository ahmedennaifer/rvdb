#[derive(Debug)]
pub struct Vects {
    pub chunk: String,
    pub id: i32,
    pub dimensions: i32,
    pub embeddings: Vec<f32>,
}

impl Vects {
    pub fn new(
        store: &Vec<Vects>,
        dimensions: i32,
        embeddings: Vec<f32>,
        chunk: String,
    ) -> Result<Self, String> {
        if dimensions <= 0 {
            return Err("Dimensions cannot be <= 0".to_string());
        }
        if dimensions != embeddings.len() as i32 {
            return Err(format!(
                "Expected {} dimensions, got {}",
                dimensions,
                embeddings.len()
            ));
        }
        if chunk.is_empty() {
            return Err("Chunks cannot be empty!".to_string());
        }
        Ok(Self {
            chunk,
            id: Self::generate_new_id(store),
            dimensions,
            embeddings,
        })
    }

    fn generate_new_id(store: &Vec<Vects>) -> i32 {
        (store.len() + 1) as i32
    }
}

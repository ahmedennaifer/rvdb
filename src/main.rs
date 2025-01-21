mod metrics;
mod store;
mod vector;

use crate::metrics::*;
use crate::store::VectorStore;
use crate::vector::Vects;

fn main() {
    // let mut store: Vec<Vects> = Vec::new();
    let chunk = "Hello this is a test chunk".to_string();
    let chunk2 = "Hello this is a second chunk".to_string();

    let mut store = VectorStore::new();

    let v1 = vec![
        -0.07354960337397,
        -0.05765195584041892,
        0.062176459864264066,
        0.04832337423482705,
        0.013338488781146072,
        0.04283848723002973,
        0.0341769510771582,
        0.03369134144974533,
        0.022621200985113627,
        -0.053638809189174554,
    ];

    let v2 = vec![
        -0.34547147598963884,
        -0.285593942468885,
        0.3658436140085148,
        -0.061224927236876295,
        0.2721806947862131,
        0.019194425589383637,
        0.5458427652462112,
        0.42503980299112604,
        -0.09781361823566571,
        0.31444252046744164,
    ];

    match Vects::new(&store, 10, v1, chunk) {
        Ok(v) => {
            println!("Vector with id {} created", v.id);
            &store.push(v);
        }
        Err(e) => {
            println!("Erorr creating the vector: {e:?}");
            return;
        }
    };
    match Vects::new(&store, 10, v2, chunk2) {
        Ok(v) => {
            println!("Vector with id {} created", v.id);
            store.push(v);
        }
        Err(e) => {
            println!("Error creating the vector: {e:?}");
            return;
        }
    };

    let vec1 = store.get_by_id(1).unwrap();
    let vec2 = store.get_by_id(2).unwrap();
    let distance = euclidean_distance(vec1, vec2);
    let dot_prod = dot_product(vec1, vec2);
    let cosine_sim = cosine_similarity(vec1, vec2);

    // println!("euc dist: {distance:?}");
    // println!("cos dist: {cosine_sim:?}");
    // println!("dot: {dot_prod:?}");
    let v3 = vec![
        -0.363884,
        -0.2468885,
        0.0085148,
        0.236876295,
        -0.7862131,
        0.9383637,
        -0.5458427652462112,
        0.80299112604,
        -0.1823566571,
        0.352046744164,
    ];
    let v3_chunk = "Lorem ipsum".to_string();
    let query = Vects::new(&store, 10, v3, v3_chunk).unwrap();
    let res = store.k_nearest_neighbor(&query, 2);
    println!("{res:?}");
}

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Soda {
    id: usize,
    name: String,
    price: f32,
}

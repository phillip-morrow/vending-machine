use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Soda {
    id: usize,
    name: String,
    price: f32,
}

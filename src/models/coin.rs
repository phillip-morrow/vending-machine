use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Coin {
    id: usize,
    name: String,
    value: f32,
}
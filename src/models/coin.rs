use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Coin {
    id: usize,
    name: String,
    value: f32,
}
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Bill {
    id: usize,
    name: String,
    value: f32,
}
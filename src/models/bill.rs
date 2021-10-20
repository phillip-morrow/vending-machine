use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Bill {
    id: usize,
    name: String,
    value: f32,
}
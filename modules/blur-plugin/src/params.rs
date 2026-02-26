use serde::Deserialize;

// Struct is passed as JSON string, no strict fields order is required
#[derive(Debug, Default, Deserialize)]
pub struct Params {
  pub radius: u32,
  pub iterations: u32,
}

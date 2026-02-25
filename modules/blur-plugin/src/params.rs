use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct Params {
  pub radius: u32,
  pub iterations: u32,
}

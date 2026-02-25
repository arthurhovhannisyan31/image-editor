use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct Params {
  pub horizontal: bool,
  pub vertical: bool,
}

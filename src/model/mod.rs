use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub enum Error {}

#[derive(Clone)]
pub struct ModelManager {}

impl ModelManager {
	pub async fn new() -> Self {
		Self {}
	}
}

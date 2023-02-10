use crate::resources::strings::PROJECT_DEFAULT_NAME;

pub struct Project {
	pub name: String,
}

impl Project {
	pub fn new(name: &str) -> Self {
		Self {
			name: name.to_owned(),
		}
	}
}

impl Default for Project {
	fn default() -> Self {
		Self::new(PROJECT_DEFAULT_NAME)
	}
}

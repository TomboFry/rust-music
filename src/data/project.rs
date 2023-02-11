use crate::resources::strings;

pub struct Project {
	pub name: String,
	pub tempo: f64,
	pub time_signature_numerator: usize,
	pub time_signature_denominator: usize,
}

impl Project {
	pub fn new(name: &str, tempo: f64) -> Self {
		Self {
			name: name.to_owned(),
			tempo,
			time_signature_numerator: 4,
			time_signature_denominator: 4,
		}
	}
}

impl Default for Project {
	fn default() -> Self {
		Self::new(strings::PROJECT_DEFAULT_NAME, 140.0)
	}
}

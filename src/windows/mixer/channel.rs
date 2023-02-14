use crate::resources::strings;

pub struct Channel {
	pub volume: f32,
	pub panning: f32,
	pub muted: bool,
	pub name: String,
}

impl Channel {
	pub fn new(name: Option<&str>) -> Self {
		Self {
			volume: 0.0,
			panning: 0.0,
			muted: false,
			name: name.unwrap_or(strings::CHANNEL_DEFAULT_NAME).to_owned(),
		}
	}
}

use super::Channel;
use crate::resources::strings;

pub struct Mixer {
	pub channels: Vec<Channel>,
}

impl Mixer {
	pub fn add_channel(&mut self) {
		let len = self.channels.len();
		let name = format!("{} {}", strings::CHANNEL_DEFAULT_NAME, len + 1);
		let channel = Channel::new(&name);

		self.channels.push(channel);
	}
}

impl Default for Mixer {
	fn default() -> Self {
		Self {
			channels: vec![
				Channel::new("Master"),
				Channel::new("Channel 1"),
				Channel::new("Channel 2"),
			],
		}
	}
}

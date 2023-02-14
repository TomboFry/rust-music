use super::Channel;
use crate::resources::strings;

pub struct Mixer {
	pub channels: Vec<Channel>,
	pub remove_queue: Vec<usize>,
}

impl Mixer {
	pub fn add_channel(&mut self) {
		let len = self.channels.len();
		let name = format!("{} {}", strings::CHANNEL_DEFAULT_NAME, len + 1);
		let channel = Channel::new(Some(&name));

		self.channels.push(channel);
	}

	pub fn clean_channels(&mut self) {
		if self.remove_queue.len() == 0 {
			return;
		}

		self.remove_queue.iter().for_each(|idx| {
			self.channels.remove(*idx);
		});

		self.remove_queue.clear();
	}
}

impl Default for Mixer {
	fn default() -> Self {
		Self {
			channels: vec![
				Channel::new(Some("Master")),
				Channel::new(Some("Channel 1")),
				Channel::new(Some("Channel 2")),
			],
			remove_queue: Vec::with_capacity(1),
		}
	}
}

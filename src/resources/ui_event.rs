use super::{PlayState, Progress};
use std::path::PathBuf;
use std::time::Duration;

pub enum UiEvent {
	// General Project
	ProjectName(String),
	ProjectTempo(f64),
	ProjectTimeSignatureNumerator(usize),
	ProjectTimeSignatureDenominator(usize),
	ProjectPlayState(PlayState),
	ProjectSongPosition(Duration),

	// Mixer
	AddChannel {
		name: String,
	},
	RemoveChannel {
		channel_index: usize,
	},
	ChannelVolume {
		channel_index: usize,
		volume: f32,
	},
	ChannelPanning {
		channel_index: usize,
		panning: f32,
	},

	// Sampler
	AddSample {
		path: PathBuf,
	},
	LoadSampleProgress {
		sample_index: usize,
		progress: Progress,
	},
	RemoveSample {
		sample_index: usize,
	},
	SampleChannel {
		sample_index: usize,
		channel_index: usize,
	},
	PlayPauseSample {
		sample_index: usize,
		play_state: PlayState,
	},

	// Settings
	OutputDevice(usize),
	OutputDeviceSampleRate(u32),
	OutputDeviceChannels(usize),
	InputDevice(usize),
}

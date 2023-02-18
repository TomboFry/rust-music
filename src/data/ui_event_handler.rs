use super::SystemState;
use crate::resources::UiEvent;
use std::{
	collections::VecDeque,
	sync::{Arc, RwLock},
};

pub fn ui_event_handler(state: &Arc<RwLock<SystemState>>, events: &mut VecDeque<UiEvent>) {
	let mut state = state.write().unwrap();

	if events.len() > 0 {
		println!(
			"{:?}: {} new events!",
			std::time::Instant::now(),
			events.len()
		);
	}

	while let Some(event) = events.pop_front() {
		match event {
			// General Project
			UiEvent::ProjectName(name) => {
				state.project.name = name.to_string();
			}
			UiEvent::ProjectTempo(tempo) => {
				state.project.tempo = tempo;
			}
			UiEvent::ProjectTimeSignatureNumerator(numerator) => {
				state.project.time_signature_numerator = numerator;
			}
			UiEvent::ProjectTimeSignatureDenominator(denominator) => {
				state.project.time_signature_denominator = denominator;
			}
			UiEvent::ProjectPlayState(play_state) => {
				state.project.play_state = play_state;
			}
			UiEvent::ProjectSongPosition(song_position) => {
				state.project.song_position = song_position;
			}

			// Mixer
			UiEvent::AddChannel => {
				state.mixer.add_channel();
			}
			UiEvent::RemoveChannel { channel_index } => {
				state.mixer.channels.remove(channel_index);
			}
			UiEvent::ChannelPanning {
				channel_index,
				panning,
			} => state.mixer.channels[channel_index].panning = panning,
			UiEvent::ChannelVolume {
				channel_index,
				volume,
			} => state.mixer.channels[channel_index].volume = volume,
			UiEvent::ChannelMuted {
				channel_index,
				muted,
			} => state.mixer.channels[channel_index].muted = muted,

			// Sampler
			// UiEvent::AddSample { path } => {}
			// UiEvent::LoadSampleProgress { sample_index, progress } => {},
			UiEvent::RemoveSample { sample_index } => {
				state.sampler.files.remove(sample_index);
			}
			UiEvent::SampleChannel {
				sample_index,
				channel_index,
			} => {
				state.sampler.files[sample_index].channel = channel_index;
			}
			UiEvent::PlayPauseSample {
				sample_index,
				play_state,
			} => {
				state.sampler.files[sample_index].play_state = play_state;
			}

			// Settings
			UiEvent::OutputDevice(device_index) => {
				state.audio.update_output_config(device_index);
			}
			UiEvent::OutputDeviceSampleRate(sample_rate) => {
				state.audio.output_sample_rate = sample_rate;

				let device_index = state.audio.active_output_index;
				state.audio.update_output_config(device_index);
			}
			UiEvent::OutputDeviceChannels(channels) => {
				state.audio.output_channels = channels;

				let device_index = state.audio.active_output_index;
				state.audio.update_output_config(device_index);
			}
			// UiEvent::InputDevice(device_index) => state.audio.update_input_config(*device_index),
			x => {
				eprintln!("Unimplemented: {:#?}", x);
			}
		}
	}

	assert!(events.len() == 0);
}

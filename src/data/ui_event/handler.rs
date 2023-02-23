use super::UiEvent;
use crate::data::Project;
use std::{
	collections::VecDeque,
	sync::{Arc, RwLock},
};

pub fn ui_event_handler(state: &mut Arc<RwLock<Project>>, events: &mut VecDeque<UiEvent>) {
	if events.len() == 0 {
		return;
	}

	println!("{:?}", events);
	let mut state = state.write().unwrap();

	while let Some(event) = events.pop_front() {
		match event {
			// General Project
			UiEvent::ProjectName(name) => {
				state.name = name.to_string();
			}
			UiEvent::ProjectTempo(tempo) => {
				state.tempo = tempo;
			}
			UiEvent::ProjectTimeSignatureNumerator(numerator) => {
				state.time_signature_numerator = numerator;
			}
			UiEvent::ProjectTimeSignatureDenominator(denominator) => {
				state.time_signature_denominator = denominator;
			}
			UiEvent::ProjectPlayState(play_state) => {
				state.play_state = play_state;
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
			UiEvent::ChannelName {
				channel_index,
				name,
			} => state.mixer.channels[channel_index].name = name,
			UiEvent::SelectChannel { channel_index } => {
				state.mixer.selected_channel = channel_index
			}

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

			x => {
				eprintln!("Unimplemented: {:#?}", x);
			}
		}
	}

	assert!(events.len() == 0);
}

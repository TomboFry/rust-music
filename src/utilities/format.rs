use crate::resources::PlayState;
use std::time::{Duration, Instant};

pub fn format_duration(duration: &Duration) -> String {
	let milliseconds_total = duration.as_millis() as f64;
	let milliseconds_segment = milliseconds_total % 1000.0;
	let seconds = (milliseconds_total / 1000.0).trunc() % 60.0;
	let minutes = (milliseconds_total / 36000.0).trunc();
	format!(
		"{:0>2}:{:0>2}.{:0>3}",
		minutes, seconds, milliseconds_segment
	)
}

pub fn duration_since_play_state(play_state: &PlayState) -> Duration {
	match play_state {
		PlayState::Playing { start_time } => Instant::now().duration_since(*start_time),
		_ => Duration::ZERO,
	}
}

pub fn format_play_state(play_state: &PlayState) -> String {
	format_duration(&duration_since_play_state(&play_state))
}

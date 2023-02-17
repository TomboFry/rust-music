use std::{error::Error, time::Instant};

pub enum PlayState {
	Stopped,
	// Paused,
	Playing(Instant),
}

pub enum Progress {
	InProgress(f32),
	Finished,
	Cancelled,
	Error(Box<dyn Error>),
}

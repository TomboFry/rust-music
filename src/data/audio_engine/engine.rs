use super::AudioEngineEvent;
use crate::data::Project;
use cpal::{
	traits::{DeviceTrait, HostTrait, StreamTrait},
	Sample,
};
use rtrb::Consumer;
use std::{
	sync::{Arc, RwLock},
	thread::{self, JoinHandle},
	time::Duration,
};

#[derive(Copy, Clone)]
struct EngineConfig {
	pub sample_rate: f64,
	pub channels: usize,
	pub timer: f64,
}

pub struct AudioEngine {
	stream: Option<cpal::Stream>,
}

impl AudioEngine {
	pub fn init(
		project: Arc<RwLock<Project>>,
		mut event_rx: Consumer<AudioEngineEvent>,
	) -> JoinHandle<()> {
		thread::spawn(move || {
			println!("AudioEngine Thread Created!");
			let mut engine = AudioEngine { stream: None };

			loop {
				if let Ok(upd) = event_rx.pop() {
					match upd {
						AudioEngineEvent::Disable => {
							println!("Disabling engine");
							engine.stream = None;
						}
						AudioEngineEvent::Enable {
							buffer_size,
							config,
							device_index,
						} => {
							println!("Enabling engine");
							println!(
								"{} channels at {} Hz (buffer size: {})",
								config.channels(),
								config.sample_rate().0,
								buffer_size,
							);

							engine.stream = AudioEngine::build_stream(
								config,
								device_index,
								buffer_size,
								project.clone(),
							);

							if let Some(stream) = &engine.stream {
								stream.play().unwrap();
								println!("Playing!");
							} else {
								panic!("Could not create stream");
							}
						}
					}
				}

				// TODO: Automatically wait for new events, rather than sleeping?
				std::thread::sleep(Duration::from_millis(100));
			}
		})
	}

	/// Create a new CPAL audio stream to handle project data
	fn build_stream(
		config: cpal::SupportedStreamConfig,
		device_index: usize,
		buffer_size: u32,
		project: Arc<RwLock<Project>>,
	) -> Option<cpal::Stream> {
		let device = &cpal::default_host()
			.output_devices()
			.unwrap()
			.collect::<Vec<cpal::Device>>()[device_index];

		println!("{} {}", device.name().unwrap(), config.sample_format());

		let build_config = cpal::StreamConfig {
			buffer_size: cpal::BufferSize::Fixed(buffer_size),
			channels: config.channels(),
			sample_rate: config.sample_rate(),
		};

		let mut stream_config = EngineConfig {
			sample_rate: config.sample_rate().0 as f64,
			channels: config.channels() as usize,
			timer: 0.0,
		};

		let err_fn = move |err| eprintln!("{err}");

		match config.sample_format() {
			cpal::SampleFormat::F32 => device.build_output_stream(
				&build_config,
				move |data: &mut [f32], info: &cpal::OutputCallbackInfo| {
					AudioEngine::process_audio::<f32>(data, info, &project, &mut stream_config);
				},
				err_fn,
				None,
			),

			// TODO: Support other sample formats
			_ => panic!("Whoops!"),
		}
		.ok()
	}

	/// Process all synths, effects, and mixer channels.
	/// TODO: Implement! Get audio based on project play state (eg. "Playing", "Stopped", etc).
	fn process_audio<T: cpal::Sample + cpal::FromSample<f64>>(
		data: &mut [T],
		_info: &cpal::OutputCallbackInfo,
		project: &Arc<RwLock<Project>>,
		config: &mut EngineConfig,
	) {
		let project = project.read().unwrap();

		// Frame is slice containing both left and right samples
		for frame in data.chunks_mut(config.channels) {
			// Generate a sine wave based on the master channel volume and panning
			let volume = (project.mixer.channels[0].volume).clamp(0.0, 100.0) / 100.0;
			let pan = (project.mixer.channels[0].panning as f64 + 0.5) * 8.0;

			let tau = std::f64::consts::TAU;
			let sine = ((config.timer * pan * 440.0 * tau) / config.sample_rate).sin();
			let val = T::from_sample(sine * volume);

			// Increase the timer
			config.timer = (config.timer + 1.0) % config.sample_rate;

			for sample in frame.iter_mut() {
				*sample = val;
			}
		}
	}
}

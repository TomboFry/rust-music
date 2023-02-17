use super::AudioEngineEvent;
use crate::data::SystemState;
use cpal::{
	traits::{DeviceTrait, StreamTrait},
	Sample,
};
use rtrb::{Consumer, Producer, RingBuffer};
use std::{
	sync::{Arc, Mutex},
	thread,
	time::{Duration, Instant},
};

/// 2 seconds of stereo 192kHz audio.
const MAX_SAMPLES: usize = 192000 * 2 * 2;

pub struct AudioEngine {
	pub active: bool,
	pub buffer_size: u32,
	pub sample_rate: usize,
	pub stream: Option<cpal::Stream>,
}

impl AudioEngine {
	pub fn new(
		state: Arc<Mutex<SystemState>>,
		mut event_rx: Consumer<AudioEngineEvent>,
	) -> Self {
		let mut engine = AudioEngine {
			active: false,
			buffer_size: 1024,
			sample_rate: 48000,
			stream: None,
		};

		thread::spawn(move || {
			let (mut audio_tx, audio_rx) = RingBuffer::<f32>::new(MAX_SAMPLES);
			let mut audio_rx = Some(audio_rx);
			let mut timer = Instant::now();
			let mut _stream: Option<cpal::Stream> = None;

			loop {
				if let Ok(upd) = event_rx.pop() {
					match upd {
						AudioEngineEvent::Disable => {
							println!("Disabling engine");
							engine.active = false;
						}
						AudioEngineEvent::Enable {
							buffer_size,
							config,
							device_index,
						} => {
							println!("Enabling engine");
							println!(
								"{:?} channels at {} Hz (buffer size: {})",
								config.channels(),
								config.sample_rate().0,
								buffer_size,
							);

							engine.active = true;
							engine.buffer_size = buffer_size;
							engine.sample_rate =
								config.sample_rate().0 as usize;

							let f = audio_rx.take();

							_stream = AudioEngine::build_stream(
								state.clone(),
								f.unwrap(),
								config,
								device_index,
								buffer_size,
							);

							println!(
								"New buffer size: {}",
								engine.buffer_size
							);

							if let Some(stream) = &_stream {
								stream.play().unwrap();
								println!("Playing!");
							} else {
								panic!("Could not create stream");
							}
						}
					}
				}

				let seconds = (engine.buffer_size as f64
					/ engine.sample_rate as f64) * 0.9;
				let target_instant = timer + Duration::from_secs_f64(seconds);

				let sleep_duration = target_instant
					.duration_since(timer)
					.checked_sub(timer.elapsed());

				if let Some(sleep_duration) = sleep_duration {
					std::thread::sleep(sleep_duration);
				} else {
					eprintln!("Can't keep up! Consider increasing buffer size");
				}

				if engine.active == false {
					continue;
				}

				// If not successful (due to GUI having access to state),
				// try again
				let mut attempts = 5;
				let mut successful = false;
				while successful == false && attempts > 0 {
					std::thread::sleep(Duration::from_micros(10));
					successful = AudioEngine::process_audio(
						&mut audio_tx,
						state.clone(),
						engine.buffer_size,
					)
					.is_ok();
					attempts -= 1;
				}

				if successful {
					timer = Instant::now();
				}
			}
		});

		engine
	}

	fn build_stream(
		state: Arc<Mutex<SystemState>>,
		mut consumer: Consumer<f32>,
		config: cpal::SupportedStreamConfig,
		device_index: usize,
		buffer_size: u32,
	) -> Option<cpal::Stream> {
		let state = state.lock().unwrap();
		let device = &state.audio.available_outputs[device_index];

		println!("{} {}", device.name().unwrap(), config.sample_format());

		let err_fn = move |err| eprintln!("{err}");

		let final_config = cpal::StreamConfig {
			buffer_size: cpal::BufferSize::Fixed(buffer_size),
			channels: 2,
			sample_rate: config.sample_rate(),
		};

		match config.sample_format() {
			cpal::SampleFormat::F32 => device.build_output_stream(
				&final_config,
				move |data: &mut [f32], info: &cpal::OutputCallbackInfo| {
					AudioEngine::stream_fn::<f32>(data, info, &mut consumer);
				},
				err_fn,
				None,
			),

			// TODO: Support other sample formats
			_ => panic!("Whoops!"),
		}
		.ok()
	}

	fn stream_fn<T: cpal::Sample + cpal::FromSample<f32>>(
		data: &mut [T],
		_: &cpal::OutputCallbackInfo,
		consumer: &mut Consumer<f32>,
	) {
		// println!("Trying to lock consumer");
		// let mut consumer = consumer.lock().unwrap();

		// Output as many samples as possible
		let size = data.len().min(consumer.slots());

		if size < data.len() {
			println!("Buffer underrun! ({} < {})", size, data.len());
		}

		if size == 0 {
			return;
		}

		let chunk = consumer.read_chunk(size).unwrap();

		// Two slices, used if ring buffer wraps around
		let (buffer, _) = chunk.as_slices();
		let mut index = 0;

		// println!("debug: {}, {}", size, d.len());

		for sample in data {
			if index >= buffer.len() - 1 {
				break;
			}

			*sample = buffer[index].to_sample::<T>();
			index += 1;
		}

		// We're done with this data
		chunk.commit(size);
	}

	pub fn process_audio(
		producer: &mut Producer<f32>,
		state: Arc<Mutex<SystemState>>,
		buffer_size: u32,
	) -> Result<(), ()> {
		let state = state.try_lock();
		if state.is_err() {
			eprintln!("...");
			return Err(());
		}

		// let state = state.unwrap();

		println!("{}", producer.slots());

		let mut chunk = producer.write_chunk(buffer_size as usize * 2).unwrap();
		let (a, _) = chunk.as_mut_slices();

		// Generate silence
		// a.fill(0.0);

		for sample in a {
			*sample = (rand::random::<f32>() * 0.25) - 0.125;
		}

		chunk.commit_all();

		Ok(())
	}
}

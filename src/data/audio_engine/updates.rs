pub enum AudioEngineEvent {
	Disable,
	Enable {
		device_index: usize,
		buffer_size: u32,
		config: cpal::SupportedStreamConfig,
	},
}

pub const WINDOW_TITLE: &'static str = "Tombo's DAW";
pub const PROJECT_DEFAULT_NAME: &'static str = "Untitled Project";
pub const WINDOWS_LABEL: &'static str = "Windows:";

pub const MENU_LABEL_FILE: &'static str = "File";
pub const MENU_LABEL_ADD: &'static str = "Add";
pub const MENU_LABEL_VIEW: &'static str = "View";

pub const PROJECT_TEMPO: &'static str = "Tempo";
pub const PROJECT_TIME_SIGNATURE: &'static str = "Time Signature";

pub const CHANNEL_DEFAULT_NAME: &'static str = "Channel";
pub const MIXER_NEW_CHANNEL: &'static str = "Add New Channel";

pub const SETTINGS_OUTPUT_DEVICE: &'static str = "Output Device";
pub const SETTINGS_INPUT_DEVICE: &'static str = "Input Device";
pub const SETTINGS_SAMPLE_RATE: &'static str = "Sample Rate";
pub const SETTINGS_VST_RELOAD: &'static str = "Rescan VSTs";

pub const FILE_PICKER_AUDIO_NAME: &'static str = "Audio files (*.mp3, *.wav, *.flac, etc)";
pub const FILE_PICKER_AUDIO_EXTENSIONS: &[&'static str] = &["mp3", "flac", "wav", "opus"];

pub const SAMPLER_ADD_LABEL: &'static str = "Add Audio Files";

// TODO: Allow user to specify custom paths
pub const VST_PATHS: [&'static str; 2] = [
	// 32-bit VSTs cannot run on 64-bit Rust application.
	// I sure have taken FL Studio's Fruity Wrapper / Bridge for granted...
	// r"C:\Program Files (x86)\VstPlugins",
	// r"C:\Program Files (x86)\Steinberg\VSTPlugins",
	r"C:\Program Files\Steinberg\VstPlugins",
	r"C:\Program Files\VSTPlugins",
];

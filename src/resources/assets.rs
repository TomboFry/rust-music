pub const FONT_JB_REGULAR: &[u8] = include_bytes!("../../assets/fonts/JetBrainsMono-Regular.ttf");
pub const FONT_JB_ITALIC: &[u8] = include_bytes!("../../assets/fonts/JetBrainsMono-Italic.ttf");
pub const FONT_JB_BOLD: &[u8] = include_bytes!("../../assets/fonts/JetBrainsMono-Bold.ttf");
pub const FONT_JB_BOLDITALIC: &[u8] =
	include_bytes!("../../assets/fonts/JetBrainsMono-BoldItalic.ttf");

pub fn setup_custom_fonts(ctx: &egui::Context) {
	// Start with the default fonts (we will be adding to them rather than replacing them).
	let mut fonts = egui::FontDefinitions::default();

	// Install my own font (maybe supporting non-latin characters).
	// .ttf and .otf files supported.
	fonts.font_data.insert(
		"JetBrains Mono".to_owned(),
		egui::FontData::from_static(FONT_JB_REGULAR),
	);

	fonts.font_data.insert(
		"JetBrains Mono Italic".to_owned(),
		egui::FontData::from_static(FONT_JB_ITALIC),
	);

	fonts.font_data.insert(
		"JetBrains Mono Bold".to_owned(),
		egui::FontData::from_static(FONT_JB_BOLD),
	);

	fonts.font_data.insert(
		"JetBrains Mono Bold Italic".to_owned(),
		egui::FontData::from_static(FONT_JB_BOLDITALIC),
	);

	// Put my font first (highest priority) for proportional text:
	fonts
		.families
		.entry(egui::FontFamily::Proportional)
		.or_default()
		.insert(0, "JetBrains Mono".to_owned());

	// Put my font as last fallback for monospace:
	fonts
		.families
		.entry(egui::FontFamily::Monospace)
		.or_default()
		.push("JetBrains Mono".to_owned());

	// Tell egui to use these fonts:
	ctx.set_fonts(fonts);
}

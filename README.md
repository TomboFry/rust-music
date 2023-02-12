# rust-music

Yep, someone else's attempt at making a DAW in Rust.

Please read the [progress](./progress/progress.md) log to see how things are
going. There are screenshots and GIFs!

## Goals (not implemented features!)

I would like this DAW to mostly follow my workflow in FL Studio, which includes:

* Audio recording (inc. some audio editing features - volume, trimming, noise
  reduction)
* Playlist/MIDI clip management
* VST synth/effect support
* Parameter automation (as clips in the playlist editor)
* A custom synth, including:
  * Triangle and square waves (inc. duty cycle square)
  * ADSR for pitch, panning, and volume
  * LFOs for pitch, panning, volume, and PWM duty cycle
* Custom effects, including:
  * Hard clipper (with gain and saturation)
  * Bitcrusher
* I've definitely missed something - I suppose the list could be endless!

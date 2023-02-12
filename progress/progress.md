# Development Progress

## 2023-02-11

![](./2023-02-11.png)

Today, I worked on cleaning up the UI a little - mostly non visual, although I
did include [egui_extras_xt](https://github.com/xTibor/egui_extras_xt) for the
fancy knobs and song position readout at the top. I also added a second window
to display available inputs and outputs using CPAL. Currently it just converts
them all to strings, and you can change the String value - no audio devices are
actually switched or connected to yet!

Tomorrow, I'm going to learn how to open audio streams and switch between them
using the Settings window.

## 2023-02-10

![](./2023-02-10.gif)

Set up a project with egui and followed several examples to create a basic
windowing system with a mixer view. The mixer view contains audio channels with
their name, volume, panning, and mute status, all of which are editable.

Currently, the mixer channels' state are completely separate from the Project,
as they are stored in a Box inside the Windows struct. I still don't know what
the project's state management should look like. Really, I should be focusing on
making some noises rather than the GUI!

Next time:
* Generate audio of some kind, running alongside the GUI
  * Explore the possibility of using ASIO for low latency audio
* Pay closer attention to the data structures required to store a music project,
  including all its MIDI and audio data.

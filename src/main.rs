// main.rs

mod audfx;

use vst::plugin::Plugin;
use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};
use audfx::Doubler;

fn main() {
    // Initialize cpal and create an event loop
    let host = cpal::default_host();
    let event_loop = host.event_loop();

    // Choose an audio input and output device
    let input_device = host.default_input_device().expect("No input device available");
    let output_device = host.default_output_device().expect("No output device available");

    // Create a Doubler instance
    let doubler = Arc::new(Doubler::default());

    // Load the plugin and create an instance
    let plugin = vst::plugin::PluginLoader::load(doubler).expect("Failed to load plugin");

    // Connect the input and output streams
    let input_stream_id = event_loop.build_input_stream(&input_device, doubler.clone()).expect("Failed to build input stream");
    let output_stream_id = event_loop.build_output_stream(&output_device, doubler).expect("Failed to build output stream");

    // Start the audio streams
    event_loop.play_stream(input_stream_id);
    event_loop.play_stream(output_stream_id);

    // Run the event loop
    event_loop.run();
}

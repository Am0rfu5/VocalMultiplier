// audfx.rs

use std::sync::Arc;
use vst::plugin::{Info, Plugin, Category};
use vst::buffer::AudioBuffer;
use vst::util::AtomicFloat;
use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};

struct Doubler {
    delay_time: AtomicFloat,
    // Placeholder for parameters
}

impl Default for Doubler {
    fn default() -> Self {
        Doubler {
            delay_time: AtomicFloat::new(0.2), // Default delay time needs to be set here
            // Initialization of parameters
        }
    }
}

impl Plugin for Doubler {
    fn get_info(&self) -> Info {
        Info {
            name: "Doubler Effect".to_string(),
            vendor: "Your Name".to_string(),
            unique_id: 12345,
            category: Category::Effect,
            inputs: 2,
            outputs: 2, 
            parameters: 1,
            ..Default::default()
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let delay_samples = (self.delay_time.get() * 44100.0) as usize; // Sample rate

        for (_, input_buffer, output_buffer) in buffer.zip() {
            for (in_sample, out_sample) in input_buffer.iter().zip(output_buffer.iter_mut()) {
                // Implementation
                // TODO circular buffer
                // TODO Apply delay effect to audio samples
            }
        }
    }


}

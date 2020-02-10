
mod utils;

use wasm_bindgen::prelude::*;
use web_sys::{
    AudioBuffer,
    AudioContext,
};
use rand::prelude::*;

#[wasm_bindgen]
pub struct Data {
    ctx: AudioContext,
}

impl Drop for Data {
    fn drop(&mut self) {
        let _ = self.ctx.close();
    }
}

#[wasm_bindgen]
impl Data {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Data, JsValue> {
        utils::set_panic_hook();

        let ctx = web_sys::AudioContext::new()?;

        log!("Created AudioContext");

        Ok(Data{
            ctx,
        })
    }

    #[wasm_bindgen]
    pub fn generate_white_noise(&self) {
        let buffer: AudioBuffer = self.ctx.create_buffer(
            2,                                   // Channels
            1 * (self.ctx.sample_rate() as u32), // Buffer Size (1 second duration)
            self.ctx.sample_rate(),              // Sample Rate
        ).unwrap();

        log!("Created AudioBuffer");

        // Generate white noise data
        let mut rng = rand::thread_rng();
        for channel in 0..buffer.number_of_channels() {
            let mut channel_data: Vec<f32> = buffer.get_channel_data(channel).unwrap();
            for sample in channel_data.iter_mut() {
                *sample = 2.0 * rng.gen::<f32>() - 1.0;
            }
            buffer.copy_to_channel(&mut channel_data, channel as i32).unwrap();
        }

        // Log the first few samples to confirm data was written
        for channel in 0..buffer.number_of_channels() {
            let channel_data: Vec<f32> = buffer.get_channel_data(channel).unwrap();
            log!("First four samples of channel {}: {} {} {} {}",
                channel,
                channel_data[0],
                channel_data[1],
                channel_data[2],
                channel_data[3],
            )
        }

        log!("Generated sample data");

        // Create an AudioBufferSourceNode.
        // This is the AudioNode to use when we want to play an AudioBuffer
        let source = self.ctx.create_buffer_source().unwrap();

        log!("Created AudioBufferSourceNode");

        // Connect the AudioBufferSourceNode to the
        // destination so we can hear the sound
        source.connect_with_audio_node(&self.ctx.destination()).unwrap();

        log!("Connected AudioBufferSourceNode with destination {:?}",
             self.ctx.destination());

        // Set the buffer in the AudioBufferSourceNode
        source.set_buffer(Some(&buffer));

        log!("Set the AudioBufferSourceNode buffer to {:?}", source.buffer());

        // Start the source playing
        source.start().unwrap();

        log!("Started source playing");
    }
}

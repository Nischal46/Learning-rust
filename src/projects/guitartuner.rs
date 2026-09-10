use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub fn init() {
    println!("Guitar tuner");

    let host = cpal::default_host();

    let mic = host.default_input_device().expect("No microphone found in device");
    let speaker = host.default_output_device().expect("No speaker found in device");

    let decode_mic = mic.default_input_config().unwrap().into();

    let decode_speaker = speaker.default_output_config();

    let audio_storage = Arc::new(Mutex::new(Vec::new()));
    let recording_storage = audio_storage.clone();

    //starting of recording strema

    println!("Starting recording stream...");
    let input_stream = mic.build_input_stream(decode_mic, move |data: &[f32], _: &_| {
        recording_storage
            .lock()
            .unwrap()
            .extend(data.iter().copied());
    }, |err| eprintln!("Error occurred while recording: {}", err), None).unwrap();


}
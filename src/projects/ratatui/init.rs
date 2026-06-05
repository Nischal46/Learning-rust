// use cpal::traits::{DeviceTrait, HostTrait};

// pub fn init() {
//     let host = cpal::default_host();
//
//     let device = host
//         .default_output_device()
//         .expect("no output device detected");
//     println!("Mic: {}", device.description().unwrap());
//
//     let mut supported_configs_range = device
//         .supported_output_configs()
//         .expect("error while querying configs");
//     let supported_config = supported_configs_range
//         .next()
//         .expect("no supported config?!")
//         .with_max_sample_rate();
//
//     println!("Logging of supporting config ----> {:?}", supported_config);
// }
//

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub fn init() {
    let host = cpal::default_host();

    // 1. Get the Mic and Speakers
    let mic = host.default_input_device().expect("No mic found");
    let speaker = host.default_output_device().expect("No speaker found");

    let mic_config = mic.default_input_config().unwrap().into();
    let speaker_config = speaker.default_output_config().unwrap().into();

    // 2. Create a Thread-Safe Storage Buffer
    // Arc allows multiple owners (threads) to share the data.
    // Mutex prevents them from accessing it at the exact same time and crashing.
    let audio_storage = Arc::new(Mutex::new(Vec::new()));

    // Create a clone of the reference to hand to the recording thread
    let recording_storage = audio_storage.clone();

    // -----------------------------------------
    // STEP 1: RECORDING
    // -----------------------------------------
    println!("Recording for 3 seconds...");
    let input_stream = mic
        .build_input_stream(
            &mic_config,
            move |data: &[f32], _: &_| {
                // Every time the mic hears something, lock the storage and save the data
                recording_storage.lock().unwrap().extend_from_slice(data);
            },
            |err| eprintln!("Mic error: {}", err),
            None,
        )
        .unwrap();

    input_stream.play().unwrap();
    std::thread::sleep(Duration::from_secs(10)); // Let it record for 3 seconds
    drop(input_stream); // Dropping the stream stops the recording
    println!("Recording stopped.");

    // -----------------------------------------
    // STEP 2: PLAYBACK
    // -----------------------------------------
    println!("Playing back...");
    let mut playback_index = 0; // Keep track of where we are in the recording
    let playback_storage = audio_storage.clone();

    let output_stream = speaker
        .build_output_stream(
            &speaker_config,
            move |data: &mut [f32], _: &_| {
                let saved_audio = playback_storage.lock().unwrap();

                // Fill the speaker buffer with our saved audio
                for sample in data.iter_mut() {
                    // Get the sample, or default to 0.0 (silence) if we reached the end
                    *sample = *saved_audio.get(playback_index).unwrap_or(&0.0);
                    playback_index += 1;
                }
            },
            |err| eprintln!("Speaker error: {}", err),
            None,
        )
        .unwrap();

    output_stream.play().unwrap();
    std::thread::sleep(Duration::from_secs(3)); // Wait for playback to finish
}

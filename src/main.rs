use std::{error::Error, io::stdin};

use cpal::{
    traits::{DeviceTrait, StreamTrait},
    Device, Stream,
};

mod audio;
mod midi;

fn main() {
    let result: Result<(Device, Stream), Box<dyn Error>> = match audio::read() {
        Ok((out_device, config)) => {
            println!("\nOpening stream");
            let stream = out_device
                .build_output_stream(
                    &config,
                    move |_data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                        // react to stream events and read or write stream data here.
                    },
                    move |_err| {
                        // react to errors here.
                    },
                )
                .unwrap();
            Ok((out_device, stream))
        }
        Err(err) => return println!("Error opening stream: {}", err),
    };

    let (device, stream) = result.unwrap();
    stream.play().unwrap();

    println!(
        "Connection open, reading input from '{}' (press enter to exit) ...",
        device.name().unwrap()
    );

    match midi::read() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    }

    let mut input = String::new();
    input.clear();

    let _ler = stdin().read_line(&mut input); // wait for next enter key press

    println!("Closing stream");
}

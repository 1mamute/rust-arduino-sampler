use cpal::{
    traits::{DeviceTrait, HostTrait},
    Device, StreamConfig,
};
use std::{
    error::Error,
    io::{stdin, stdout, Write},
};

pub fn read() -> Result<(Device, StreamConfig), Box<dyn Error>> {
    let host = cpal::default_host();

    // Get an output device (read from console if multiple are available)
    let out_devices = host.output_devices();
    let out_devices_to_read: Vec<Device> = host.output_devices().unwrap().collect();
    let out_device = match out_devices_to_read.len() {
        0 => return Err("no output device found".into()),
        1 => {
            println!(
                "Choosing the only available output device: {}",
                out_devices_to_read.last().unwrap().name().unwrap()
            );
            out_devices.unwrap().last().ok_or("invalid output device")
        }
        _ => {
            println!("\nAvailable output devices:");
            for (i, p) in out_devices_to_read.iter().enumerate() {
                println!("{}: {}", i, p.name()?);
            }
            print!("Please select output device: ");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            out_devices
                .unwrap()
                .nth(input.trim().parse::<usize>()? - 1)
                .ok_or("invalidar output device")
        }
    };

    let mut supported_configs_range = out_device
        .as_ref()
        .unwrap()
        .supported_output_configs()
        .expect("error while querying configs");

    let supported_config = supported_configs_range
        .next()
        .expect("no supported config?!")
        .with_max_sample_rate();

    Ok((out_device.unwrap(), supported_config.into()))
}

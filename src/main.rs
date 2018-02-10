#[macro_use] extern crate lazy_static;
extern crate regex;

use std::time::Duration;
use std::thread;
use std::process::Command;
use std::vec::Vec;

use regex::Regex;

fn main() {
    println!("Starting fan control");

    let mut prev_measurements = (0, 0, 0);

    loop {
	let measurements = fetch_temperature();

	if prev_measurements != measurements {
            println!("Current temperature min: {}, max: {}, avg: {}", measurements.0, measurements.1, measurements.2);
	    match measurements.1 {
                    0 ... 40 => set_fan_manual(0),
                    40 ... 55 => set_fan_manual((measurements.1 - 40) as u8),
                    _ => set_fan_automatic()
            };
            prev_measurements = measurements;
        }

	thread::sleep(Duration::from_secs(1));
    }
}

fn fetch_temperature() -> (u32, u32, u32) {
        lazy_static! {
		static ref RE_TEMPERATURE: Regex = Regex::new("\\d+\\.\\d+").unwrap();
	}

	let output = Command::new("sensors")
            .output()
            .expect("Failed to fetch temperature");
        let stdout = String::from_utf8_lossy(&output.stdout);
	let mut measurements = Vec::new();
	for line in stdout.split("\n") {
		let measurement: u32 = match RE_TEMPERATURE.captures(line) {
		    Some(captures) => captures.get(0).unwrap().as_str().parse::<f32>().unwrap() as u32,
		    None => continue
		};
		measurements.push(measurement);
	}
	measurements.sort();
	let avg = measurements.iter().cloned().sum::<u32>() / measurements.len() as u32;
	let min = *measurements.first().unwrap();
	let max = *measurements.last().unwrap();
	return (min, max, avg);
}

fn set_fan_automatic() {
    println!("Setting fan to automatic mode");
    Command::new("ipmitool")
        .arg("raw")
        .arg("0x30")
        .arg("0x30")
        .arg("0x01")
        .arg("0x01")
        .output()
        .expect("Failed to set fan to automatic mode");
}

fn set_fan_manual(speed: u8) {
    println!("Setting fan to manual mode 0x{:X}", speed);
    Command::new("ipmitool")
        .arg("raw")
        .arg("0x30")
        .arg("0x30")
        .arg("0x01")
        .arg("0x00")
        .output()
        .expect("Failed to set fan to manual mode");

    Command::new("ipmitool")
        .arg("raw")
        .arg("0x30")
        .arg("0x30")
        .arg("0x02")
        .arg("0xFF")
        .arg(format!("0x{:X}", speed))
        .output()
        .expect("Failed to set fan speed");
}

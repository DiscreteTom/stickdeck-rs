use anyhow::Result;
use log::{debug, info, warn};
use std::env;
use std::sync::mpsc;
use std::thread::sleep;
use std::time::{Duration, Instant};
use stickdeck_common::*;

mod client;
mod input;

use client::Client;
use input::{init_gamepad, init_mouse};

const RETRY_SECONDS: u64 = 3;
const CHANNEL_CAPACITY: usize = 10;

pub fn main() -> Result<()> {
	env_logger::init();

	let args: Vec<String> = env::args().collect();
	if args.len() < 2 {
		eprintln!("Usage: {} <server_ip>", args[0]);
		std::process::exit(1);
	}

	let server_ip = &args[1];
	
	// Check if user accidentally included port in IP
	if server_ip.contains(':') {
		eprintln!("Error: Please provide only the IP address, not IP:port");
		eprintln!("The port {} will be added automatically", PORT);
		eprintln!("Usage: {} <server_ip>", args[0]);
		eprintln!("Example: {} 192.168.1.100", args[0]);
		std::process::exit(1);
	}
	
	let server_addr = format!("{}:{}", server_ip, PORT);

	info!("StickDeck Linux Client starting...");
	info!("Connecting to server at {}", server_addr);

	let (tx, rx) = mpsc::sync_channel::<Packet<XGamepad>>(CHANNEL_CAPACITY);
	let client = Client::new(server_addr, tx, Duration::from_secs(RETRY_SECONDS));
	client.start();

	info!("Initializing virtual gamepad...");
	let mut update_gamepad = init_gamepad()?;

	info!("Initializing virtual mouse...");
	let mut update_mouse = init_mouse()?;

	info!("Ready! Waiting for inputs from Steam Deck...");

	let mut updates_per_second = 0;
	let mut last_print = Instant::now();

	loop {
		match rx.recv() {
			Ok(packet) => {
				match packet {
					Packet::Timestamp(timestamp) => {
						debug!("Received timestamp: {}", timestamp);
					}
					Packet::Gamepad(gamepad) => {
						perf!("update_gamepad", {
							update_gamepad(&gamepad)
						}, 10)?;
						updates_per_second += 1;
					}
					Packet::Mouse(mouse) => {
						perf!("update_mouse", {
							update_mouse(&mouse)
						}, 10)?;
					}
				}

				if last_print.elapsed() >= Duration::from_secs(1) {
					if updates_per_second > 0 {
						debug!("Updates per second: {}", updates_per_second);
					}
					updates_per_second = 0;
					last_print = Instant::now();
				}
			}
			Err(e) => {
				warn!("Failed to receive packet: {}", e);
				sleep(Duration::from_secs(RETRY_SECONDS));
			}
		}
	}
}
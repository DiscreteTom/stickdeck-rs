use log::{info, warn};
use std::{io::Read, net::TcpStream, sync::mpsc, thread, time::Duration};
use stickdeck_common::{Mouse, Packet, XButtons, XGamepad, PACKET_FRAME_SIZE};

pub struct Client {
	server_addr: String,
	tx: mpsc::SyncSender<Packet<XGamepad>>,
	retry_duration: Duration,
}

impl Client {
	pub fn new(
		server_addr: String,
		tx: mpsc::SyncSender<Packet<XGamepad>>,
		retry_duration: Duration,
	) -> Self {
		Self {
			server_addr,
			tx,
			retry_duration,
		}
	}

	pub fn start(self) {
		thread::spawn(move || {
			loop {
				info!("Connecting to {} ...", self.server_addr);
				match self.connect_and_receive() {
					Ok(_) => {
						info!("Connection closed");
					}
					Err(e) => {
						warn!("Connection error: {}", e);
					}
				}
				info!("Retrying in {} seconds...", self.retry_duration.as_secs());
				thread::sleep(self.retry_duration);
			}
		});
	}

	fn connect_and_receive(&self) -> std::io::Result<()> {
		let mut stream = TcpStream::connect(&self.server_addr)?;
		info!("Connected to {}", self.server_addr);

		let mut buf = [0; PACKET_FRAME_SIZE];
		while stream.read_exact(&mut buf).is_ok() {
			match Packet::deserialize(&buf) {
				Ok(packet) => {
					if self.tx.send(packet).is_err() {
						warn!("Main thread has disconnected");
						break;
					}
				}
				Err(_) => {
					warn!("Invalid packet: {:?}", buf);
				}
			}
		}

		info!("Disconnected from server");
		Ok(())
	}
}

trait DeserializablePacket {
	type Target;
	fn deserialize(buf: &[u8; PACKET_FRAME_SIZE]) -> Result<Self::Target, u8>;
}

impl<Gamepad: DeserializableGamepad<Target = Gamepad>> DeserializablePacket for Packet<Gamepad> {
	type Target = Self;

	fn deserialize(buf: &[u8; PACKET_FRAME_SIZE]) -> Result<Self, u8> {
		match buf[0] {
			0 => {
				let timestamp = u64::from_le_bytes(buf[1..9].try_into().unwrap());
				Ok(Packet::Timestamp(timestamp))
			}
			1 => Ok(Packet::Gamepad(Gamepad::deserialize(&buf[1..]))),
			2 => Ok(Packet::Mouse(Mouse::deserialize(&buf[1..]))),
			_ => Err(buf[0]),
		}
	}
}

// rust-analyzer might throw errors below, but it's fine
// see https://github.com/rust-lang/rust-analyzer/issues/17040
include!("../../snippet/deserialize.rs");

#[cfg(test)]
mod tests {
	use super::*;

	// rust-analyzer might throw errors below, but it's fine
	// see https://github.com/rust-lang/rust-analyzer/issues/17040
	include!("../../snippet/serialize.rs");
	include!("../../snippet/test_serialize_deserialize.rs");
}
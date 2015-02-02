use platform::io;
use core::prelude::{Ok, Copy};
use core::fmt::{Writer, Result};
use core::str::StrExt;
use core::clone::Clone;


// These should really be found from the bios data area
pub const COM1: u16 = 0x3F8;
pub const COM2: u16 = 0x2F8;
pub const COM3: u16 = 0x3E8;
pub const COM4: u16 = 0x2E8;

const DATA: u16 = 0;
const INTERRUPT_ENABLE: u16 = 1;
const INTERRUPT_IDENTIFICATION: u16 = 2;
const LINE_CONTROL: u16 = 3;
const MODEM_CONTROL: u16 = 4;
const LINE_STATUS: u16 = 5;
const MODEM_STATUS: u16 = 6;
const SCRATCH: u16 = 7;

#[derive(Clone)]
pub enum BaudRate {
	b115200 = 1,
	b57600 = 2,
	b38400 = 3,
	b28800 = 4,
	b19200 = 6,
	b14400 = 8,
	b9600 = 12,
	b4800 = 24,
	b2400 = 48,
	b1200 = 96,
	b600 = 192,
	b300 = 384,
	b220 = 524,
	b110 = 1047,
	b50 = 2304,
}

#[derive(Clone)]
pub enum DataBit {
	b5 = 0b00,
	b6 = 0b01,
	b7 = 0b10,
	b8 = 0b11,
}

#[derive(Clone)]
pub enum StopBit {
	b1 = 0b0,
	b2 = 0b1,
}

#[derive(Clone)]
pub enum Parity {
	none 	= 0b0000,
	odd 	= 0b0001,
	even	= 0b0011,
	mark	= 0b0101,
	space	= 0b0111,
}

pub mod interrupts {
	pub const DATA_AVAILABLE: u8 = (0 << 1);
	pub const TRANSMITTER_EMPTY: u8 = (1 << 1);
	pub const BREAK: u8 = (2 << 1);
	pub const STATUS_CHANGE: u8 = (3 << 1);
}

#[derive(Clone)]
pub struct SerialPort {
	port: u16,
	baud_rate: BaudRate,
	data_bits: DataBit,
	stop_bits: StopBit,
	parity: Parity,
	interrupts: u8,
}

pub fn new(port: u16, baud_rate: BaudRate, data_bits: DataBit,
		stop_bits: StopBit, parity: Parity, interrupts: u8) -> SerialPort {
	SerialPort {
		port: port,
		baud_rate: baud_rate,
		data_bits: data_bits,
		stop_bits: stop_bits,
		parity: parity,
		interrupts: interrupts,
	}
}

impl SerialPort {
	// http://wiki.osdev.org/Serial_Ports
	// http://en.wikibooks.org/wiki/Serial_Programming/8250_UART_Programming
	pub fn setup(&self) {
		let br = self.baud_rate.clone() as u16;
		let lc = (self.data_bits.clone() as u16 | ((self.stop_bits.clone() as u16) << 2 )
			| ((self.parity.clone() as u16) << 3 )) as u8;
		unsafe {
			io::outport(self.port + INTERRUPT_ENABLE, 0x00); 		// Disable interrupts
			io::outport(self.port + LINE_CONTROL, 0x80); 			// Enable DLAB
			io::outport(self.port + DATA, (br & 0xFF) as u8); 			// Set divisor low byte
			io::outport(self.port + INTERRUPT_ENABLE, (br << 8) as u8); // set divisor high byte
			io::outport(self.port + LINE_CONTROL, lc);
			io::outport(self.port + INTERRUPT_IDENTIFICATION, 0xC7); // TODO: Control this, and the other stuff.
			io::outport(self.port + MODEM_CONTROL, 0x0B);
		}
	}

	pub fn serial_received(&self) -> bool {
		unsafe {
			io::inport(self.port + LINE_STATUS) & 1 > 0
		}
	}

	pub fn read_serial(&self) -> u8 {
		while !self.serial_received() {}

		unsafe {
			io::inport(self.port)
		}
	}

	pub fn is_transmit_empty(&self) -> bool {
		unsafe {
			io::inport(self.port + LINE_STATUS) & 0x20 > 1
		}
	}

	pub fn write_serial(&self, a: u8) {
		while !self.is_transmit_empty() {}

		unsafe {
			io::outport(self.port, a);
		}
	}
}

impl Writer for SerialPort {
	fn write_str(&mut self, s: &str) -> Result {
		for c in s.chars() {
			self.write_serial(c as u8);
		}
		Ok(())
	}
}


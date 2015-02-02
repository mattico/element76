use platform::io;
use core::prelude::Ok;
use core::fmt::{Writer, Result};
use core::str::StrExt;
use core::clone::Clone;

// These should really be found from the bios data area
#[allow(non_upper_case_globals, non_snake_case)]
pub mod DefaultPort {
	pub const COM1: u16 = 0x3F8;
	pub const COM2: u16 = 0x2F8;
	pub const COM3: u16 = 0x3E8;
	pub const COM4: u16 = 0x2E8;
}

// Register offsets
const DATA: u16 = 0;
const INTERRUPT_ENABLE: u16 = 1;
const INTERRUPT_IDENTIFICATION: u16 = 2;
const LINE_CONTROL: u16 = 3;
const MODEM_CONTROL: u16 = 4;
const LINE_STATUS: u16 = 5;
const MODEM_STATUS: u16 = 6;
const SCRATCH: u16 = 7;

#[derive(Clone, Copy)]
pub enum BaudRate {
	B115200 = 1,
	B57600 = 2,
	B38400 = 3,
	B28800 = 4,
	B19200 = 6,
	B14400 = 8,
	B9600 = 12,
	B4800 = 24,
	B2400 = 48,
	B1200 = 96,
	B600 = 192,
	B300 = 384,
	B220 = 524,
	B110 = 1047,
	B50 = 2304,
}

#[derive(Clone, Copy)]
pub enum DataBit {
	B5 = 0b00,
	B6 = 0b01,
	B7 = 0b10,
	B8 = 0b11,
}

#[derive(Clone, Copy)]
pub enum StopBit {
	B1 = 0b0,
	B2 = 0b1,
}

#[derive(Clone, Copy)]
pub enum Parity {
	None	= 0b0000,
	Odd		= 0b0001,
	Even	= 0b0011,
	Mark	= 0b0101,
	Space	= 0b0111,
}

#[allow(non_upper_case_globals, non_snake_case)]
pub mod Interrupt {
	pub const DataAvailable: u8 = (1 << 0);
	pub const TransmitterEmpty: u8 = (1 << 1);
	pub const RecieverLineStatus: u8 = (1 << 2);
	pub const ModemStatus: u8 = (1 << 3);
	pub const Sleep: u8 = (1 << 4);
	pub const LowPower: u8 = (1 << 5);
}

#[derive(Clone, Copy)]
pub struct SerialPort {
	port: u16,
	baud_rate: BaudRate,
	data_bits: DataBit,
	stop_bits: StopBit,
	parity: Parity,
	interrupts: u8,
}

impl SerialPort {

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
			io::outport(self.port + INTERRUPT_ENABLE, (br >> 8) as u8); // set divisor high byte
			io::outport(self.port + LINE_CONTROL, lc);
			io::outport(self.port + INTERRUPT_IDENTIFICATION, 0xC7); // TODO: Control this, and the other stuff.
			io::outport(self.port + MODEM_CONTROL, 0x0B);
		}
	}

	pub fn received(&self) -> bool {
		unsafe {
			io::inport(self.port + LINE_STATUS) & 1 > 0
		}
	}

	pub fn read(&self) -> u8 {
		while !self.received() {}

		unsafe {
			io::inport(self.port)
		}
	}

	pub fn is_empty(&self) -> bool {
		unsafe {
			io::inport(self.port + LINE_STATUS) & 0x20 > 1
		}
	}

	pub fn write(&self, a: u8) {
		while !self.is_empty() {}

		unsafe {
			io::outport(self.port, a);
		}
	}
}

impl Writer for SerialPort {
	fn write_str(&mut self, s: &str) -> Result {
		for c in s.chars() {
			self.write(c as u8);
		}
		Ok(())
	}
}

pub fn interrupt_handler(serial_port: u8) {

}

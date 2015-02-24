use platform::vga::Color;
use kernel::stdio::StdioWriter;
use core::fmt::Write;
use platform::serial;
use core::prelude::*;

#[no_mangle]
pub fn entry() -> !
{
	::platform::cpu::setup();
	::platform::mmu::setup();
	::platform::cpu::enable_interrupts();
	main();
	loop { ::platform::cpu::idle(); }
}

fn main()
{
	let mut printer = StdioWriter::new();
	printer.bg = Color::Red;
	printer.fg = Color::Yellow;
	printer.clear_screen();
	printer.fg = Color::White;
	printer.go_to(3, 3);
	printer.print_screen("Hello, World!");

	let mut com = serial::SerialPort::new(serial::DefaultPort::COM1, serial::BaudRate::B115200,
		serial::DataBit::B8, serial::StopBit::B2, serial::Parity::None, 0);
	com.setup();
	match com.write_str("Serial Works!") {
		Ok(_) => {},
		Err(_) => {
			printer.go_to(3, 4);
			printer.print_screen("Serial Doesn't work...");
		},
	}
}

#[lang = "panic_fmt"]
extern fn panic_fmt(args: ::core::fmt::Arguments, file: &str, line: u32) -> !
{
	let mut printer = StdioWriter::new();
	printer.bg = Color::Black;
	printer.fg = Color::Red;

	printer.print_screen("RUST FAIL");
	printer.crlf();

	let _ = printer.write_fmt(args);
	printer.crlf();

	printer.print_screen(file);
	printer.print_char(':');
	printer.print_dec(line);

	::platform::cpu::halt();
}

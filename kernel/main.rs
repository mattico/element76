use platform::vga::Color;
// use platform::cpu::cpuid::FeatureEnum;
use kernel::stdio::StdioWriter;
use core::fmt::Write;

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
	printer.go_to(3, 4);
	/*
	if ::platform::cpu::cpuid::get_cpuid_supported() {
		printer.print_screen("CPUID Supported");
	} else {
		printer.print_screen("CPUID Not Supported");
	}
	printer.go_to(3, 5);
	if ::platform::cpu::cpuid::FeatureEDX::SSE.get_feature_supported() {
		printer.print_screen("SSE Supported");
	} else {
		printer.print_screen("SSE Not Supported");
	}
	printer.go_to(3, 6);
	if ::platform::cpu::cpuid::FeatureECX::AVX.get_feature_supported() {
		printer.print_screen("AVX Supported");
	} else {
		printer.print_screen("AVX Not Supported");
	}
	printer.go_to(3, 7);
	printer.print_screen(::platform::cpu::cpuid::get_vendor());
	*/
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

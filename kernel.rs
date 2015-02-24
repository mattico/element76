#![crate_name = "kernel"]
#![crate_type = "staticlib"]
#![no_std]
#![feature(no_std, asm, lang_items)]
#![feature(core)]

#[macro_use] extern crate core;
extern crate rlibc;

#[cfg(target_arch = "x86")]
#[path = "arch/x86/"]
pub mod platform {
	pub mod vga;
	pub mod cpu;
	pub mod mmu;
	mod io;
	pub mod keyboard;
	#[allow(dead_code)]
	pub mod serial;
}

#[cfg(target_arch = "x86_64")]
#[path = "arch/x86_64/"]
pub mod platform {
	pub mod vga;
	pub mod cpu;
	pub mod mmu;
	mod io;
	pub mod keyboard;
}

pub mod kernel {
	pub mod main;
	pub mod interrupts;
	mod stdio;
	mod keyboard;
}

// https://github.com/rust-lang/rust/issues/16803 ?
pub mod std {
	pub use core::*;
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}


use ::platform::serial;

pub fn serial1_irq() {
	serial::interrupt_handler(1);
}

pub fn serial2_irq() {
	serial::interrupt_handler(2);
}
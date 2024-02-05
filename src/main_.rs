
#![no_std]
#![no_main]

use esp32c3_hal::{
	clock::ClockControl,
	peripherals::Peripherals,
	prelude::*,
	uart,
	IO,
};
use esp_backtrace as _;
use esp_println::print;

use embedded_io::Read;

#[entry]
fn main() -> ! {
	// Take Peripherals, Initialize Clocks, and Create a Handle for Each
	let peripherals = Peripherals::take();
	let system = peripherals.SYSTEM.split();
	let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
	let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

	let mut uart0 = {
		use uart:: {
			Uart,
			config::{Config, DataBits, Parity, StopBits},
			TxRxPins,
		};
		
		let config = Config {
			baudrate: 9600,
			data_bits: DataBits::DataBits8,
			parity: Parity::ParityNone,
			stop_bits: StopBits::STOP1,
		};

		let pins = TxRxPins::new_tx_rx(io.pins.gpio1.into_push_pull_output(), io.pins.gpio2.into_floating_input());
		// let botton_pin = io.pins.gpio3.into_floating_input();

		Uart::new_with_config(peripherals.UART0, config, Some(pins), &clocks)
	};

	// Application Loop
	loop {
		let mut buf = [0u8; 1];
		Read::read(&mut uart0, &mut buf).unwrap();
		if let Some(c) = char::from_u32(buf[0] as u32) {
			print!("{}", c);
		}
	}
}

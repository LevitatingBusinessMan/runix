use core::panic::PanicInfo;
use core::fmt::Write;
use crate::vga;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	let mut cover;
	if let Some(conf) = crate::conf::CONFIG.try_read() {
		cover = conf.panic_cover;
	} else {
		cover = crate::conf::Config::default().panic_cover;
	}
	if cover {
		vga::fill(vga::Color::Red);
	}
	let msg = "PANIC";
	vga::print_at(vga::BUFFER_WIDTH / 2 - msg.len()/2, 12, msg.as_bytes(), vga::Color::White, vga::Color::Red);
	let mut printer = vga::ColoredPrinter::new(0,vga::BUFFER_HEIGHT-2,vga::Color::White, vga::Color::Red);
	write!(printer, "{}\n{}", info.location().unwrap(), info.message().unwrap()).unwrap();
	loop {};
}

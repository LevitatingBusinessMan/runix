use core::panic::PanicInfo;
use core::fmt::Write;
use crate::vga;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	let cover = match crate::conf::CONFIG.get() {
		Some(conf) => conf.panic_cover,
		None => true,
	};
	if cover {
		vga::fill(vga::Color::Red);
	}
	let mut printer = vga::ColoredPrinter::new(0,vga::BUFFER_HEIGHT-1,vga::Color::White, vga::Color::Red);
	write!(printer, "{}\n{}", info.location().unwrap(), info.message().unwrap()).unwrap();
	if cover {
		vga::print_at(vga::BUFFER_WIDTH / 2 - 2, 12, "PANIC".as_bytes(), vga::Color::White, vga::Color::Red);
	}
	crate::hlt_loop!();
}

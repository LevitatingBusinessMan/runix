use core::panic::PanicInfo;
use core::fmt::Write;

use crate::qemu::QemuDebug;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let _ = QemuDebug().write_fmt(format_args!("{info:?}\n"));
	// let cover = match crate::conf::CONFIG.get() {
	// 	Some(conf) => conf.panic_cover,
	// 	None => true,
	// };
	// if cover {
	// 	vga::fill(vga::Color::Red);
	// }
	// let mut printer = vga::ColoredPrinter::new(0,vga::BUFFER_HEIGHT-1,vga::Color::White, vga::Color::Red);
	// write!(printer, "{}\n{}", info.location().unwrap(), info.message()).unwrap();
	// if cover {
	// 	vga::print_at(vga::BUFFER_WIDTH / 2 - 2, 12, "PANIC".as_bytes(), vga::Color::White, vga::Color::Red);
	// }
	crate::hlt_loop!();
}

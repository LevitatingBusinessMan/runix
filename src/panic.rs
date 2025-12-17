use core::panic::{Location, PanicInfo};
use core::fmt::Write;

use crate::console::Console;
use crate::gfx::{Color, FramebufferGfxExtension};
use crate::qemu::QemuDebug;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let _ = QemuDebug().write_fmt(format_args!("Panic at {}\n{}\n", info.location().unwrap(), info.message()));
    if let Some(Some(fb)) = crate::FRAMEBUFFER_REQUEST.response().map(|response| response.framebuffers().first()) {
        fb.clear(Color::PANIC_RED);
        let mut console = Console::new(fb);
        //console.set_cursor((0, console.height-1));
        let _ = console.write_fmt(format_args!("Panic at {}\n{}\n", info.location().unwrap(), info.message()));
    }
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

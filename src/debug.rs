use core::slice;

pub fn dump(addr: *const i8, len: usize) {
    let longs = unsafe { slice::from_raw_parts(addr, len * 8) };
    for ls in longs.chunks_exact(8) {
        for l in ls {
            print!("{:02x}", l);
        }
        println!();
    }
}

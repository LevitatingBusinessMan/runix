use core::{ptr::addr_of, sync::atomic::{AtomicU64}};

use spin::{Lazy, mutex::Mutex};
use x86_64::{PhysAddr, VirtAddr};

use crate::limine::{MemMapEntry, MemMapType};

/**
 * NOTES
 * Take a look at early redox: https://github.com/redox-os/kernel/blob/f83f61b51a7dfe4ac546b6fb72c5b0c0d8669696/src/memory/mod.rs
 * 
 * It uses a bump allocator with a recycle allocator on top.
 * 
 * The code for the bump allocator was supposedly partially borrowed from Phil Opp's first edition
 * https://github.com/redox-os/kernel/blob/f83f61b51a7dfe4ac546b6fb72c5b0c0d8669696/src/memory/bump.rs#L2
 * 
 * New Phil Opp edition shows 3 designs
 * https://os.phil-opp.com/allocator-designs/
 * 
 * Blogpost with a simple bump(?) allocator walking the multiboot maps
 * https://anastas.io/osdev/memory/2016/08/08/page-frame-allocator.html
 */


pub fn allocate_frame() -> u64 {
    struct FrameIter {
        areas: &'static[&'static MemMapEntry],
        area_index: usize,
        frames_left_in_area: usize,
        next_frame: u64,
    }
    
    impl Iterator for FrameIter {
        type Item = u64;
    
        fn next(&mut self) -> Option<Self::Item> {
            if self.frames_left_in_area <= 0  {
                self.area_index += 1;
                // find usable area
                while !matches!(self.areas[self.area_index].r#type, MemMapType::Usable) {
                    self.area_index += 1;
                    if self.area_index >= self.areas.len() {
                        panic!("out of memory");
                    }
                }
                // move to next area
                let area = self.areas[self.area_index];
                self.frames_left_in_area = self.areas[self.area_index].length as usize / 4096 as usize;
                self.next_frame = area.base;
            }
            let frame = self.next_frame;
            self.next_frame = frame + 4096;
            self.frames_left_in_area -= 1;
            Some(frame)
        }
    }
    
    static FRAME_ITER: Lazy<Mutex<FrameIter>> = Lazy::new(|| {
        let areas = crate::MEMMAP_REQUEST.response().unwrap().entries();
        Mutex::new(FrameIter {
            areas,
            area_index: 0,
            frames_left_in_area: areas.first().unwrap().length as usize / 4096 as usize,
            next_frame: areas.first().unwrap().base,
        })
    });
    
    FRAME_ITER.lock().next().unwrap()
}

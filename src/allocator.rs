use core::{alloc::GlobalAlloc, ptr::addr_of, sync::atomic::AtomicU64};

use spin::{Lazy, mutex::Mutex};
use x86_64::{PhysAddr, VirtAddr};

use crate::{HHDM_REQUEST, limine::{MemMapEntry, MemMapType}};

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

#[derive(Debug)]
struct Allocator {
    current_frame: u64, 
    depth: usize,
}

struct LockedAllocator {
    inner: Mutex<Allocator>,
}

impl LockedAllocator {
    pub const fn new() -> Self {
        Self { inner: Mutex::new(Allocator {
            current_frame: 0,
            depth: 0,
        }) }
    }
}

unsafe impl GlobalAlloc for LockedAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let mut allocator = self.inner.lock();
        
        // initialize if necessary
        if allocator.current_frame == 0 {
            allocator.current_frame = allocate_frame();
        }
        
        if layout.size() > 4096 {
            panic!("cannot allocate more than a pagesize");
        }
        
        let addr = (allocator.current_frame as usize + allocator.depth).next_multiple_of(layout.align());
                
        if addr + layout.size() > allocator.current_frame as usize + 4096 {
            allocator.current_frame = allocate_frame();
            allocator.depth = 0;
            return allocator.current_frame as *mut u8;
        } else {
            allocator.depth = addr - allocator.current_frame as usize + layout.size();
            return addr as *mut u8;
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        println!("leaking some {} bytes", layout.size())
    }
}

#[global_allocator]
static ALLOCATOR: LockedAllocator = LockedAllocator::new();

 /// simply grab a frame, no deallocation possible
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
    
    /// add this to physical frames
    static OFFSET: Lazy<u64> = Lazy::new(|| HHDM_REQUEST.response().unwrap().offset);
    
    FRAME_ITER.lock().next().unwrap() + *OFFSET
}

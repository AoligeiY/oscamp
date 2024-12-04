#![no_std]

use allocator::{BaseAllocator, ByteAllocator, PageAllocator};
use core::mem::size_of;

/// Early memory allocator
/// Use it before formal bytes-allocator and pages-allocator can work!
/// This is a double-end memory range:
/// - Alloc bytes forward
/// - Alloc pages backward
///
/// [ bytes-used | avail-area | pages-used ]
/// |            | -->    <-- |            |
/// start       b_pos        p_pos       end
///
/// For bytes area, 'count' records number of allocations.
/// When it goes down to ZERO, free bytes-used area.
/// For pages area, it will never be freed!
///
pub struct EarlyAllocator<const PAGE_SIZE: usize>{
    bottom_phd: usize,
    top_phd: usize,
    bused: usize,
    pused: usize,
    count: usize,
}

impl<const PAGE_SIZE: usize> EarlyAllocator<PAGE_SIZE> {
    pub const fn new() -> Self {
        Self {
            bottom_phd: 0,
            top_phd: 0,
            bused: 0,
            pused: 0,
            count: 0,
        }
    }
}

impl<const PAGE_SIZE: usize> BaseAllocator for EarlyAllocator<PAGE_SIZE> {
    fn init(&mut self, start: usize, size: usize) {
        let start = (start + size_of::<usize>() - 1) & (!size_of::<usize>() + 1);
        let mut end = start + size;
        end &= !size_of::<usize>() + 1;
        assert!(start <= end);
        self.bottom_phd = start;
        self.top_phd = end;
        self.count = end - start;
    }

    fn add_memory(&mut self, start: usize, size: usize) -> allocator::AllocResult {
        let aligned_start = (start + size_of::<usize>() - 1) & (!size_of::<usize>() + 1);
        let mut end = aligned_start + size;
        end &= !size_of::<usize>() + 1;

        if self.top_phd == aligned_start {
            self.top_phd = end;
            self.count += end - aligned_start;
            Ok(())
        }  else {
            Err(allocator::AllocError::MemoryOverlap)
        }
    }
}

impl<const PAGE_SIZE: usize> ByteAllocator for EarlyAllocator<PAGE_SIZE> {
    fn alloc(&mut self, layout: core::alloc::Layout) -> allocator::AllocResult<core::ptr::NonNull<u8>> {
        if self.available_bytes() < layout.size() {
            return Err(allocator::AllocError::NoMemory);
        }

        let result = core::ptr::NonNull::new(self.bottom_phd as *mut u8);
        if let Some(result) = result{
            self.bused += layout.size();
            self.bottom_phd += layout.size();
            return Ok(result);
        } else {
            panic!("");
        }
    }

    fn dealloc(&mut self, pos: core::ptr::NonNull<u8>, layout: core::alloc::Layout) {
        let ret = pos.as_ptr() as usize + layout.size();
        if ret == self.bottom_phd {
            self.bottom_phd -= layout.size();
            self.bused -= layout.size();
        }
    }

    fn total_bytes(&self) -> usize {
        self.count
    }

    fn used_bytes(&self) -> usize {
        self.bused
    }

    fn available_bytes(&self) -> usize {
        self.count - self.bused - self.pused
    }
}

impl<const PAGE_SIZE: usize> PageAllocator for EarlyAllocator<PAGE_SIZE> {
    const PAGE_SIZE: usize = PAGE_SIZE;

    fn alloc_pages(&mut self, num_pages: usize, align_pow2: usize) -> allocator::AllocResult<usize> {
        if align_pow2 % PAGE_SIZE != 0{
            return Err(allocator::AllocError::InvalidParam);
        }
        let align_pow2 = align_pow2 / PAGE_SIZE;
        if !align_pow2.is_power_of_two(){
            return Err(allocator::AllocError::InvalidParam);
        }
        let _align_log2 = align_pow2.trailing_zeros() as usize;

        self.pused += num_pages * PAGE_SIZE;
        Ok(self.top_phd - num_pages * PAGE_SIZE)
    }

    fn dealloc_pages(&mut self, pos: usize, num_pages: usize) {
        self.pused -= num_pages * PAGE_SIZE;
    }
    fn total_pages(&self) -> usize {
        self.count/PAGE_SIZE
    }
    fn used_pages(&self) -> usize {
        self.pused/PAGE_SIZE
    }
    fn available_pages(&self) -> usize {
        ( self.count -self.bused - self.pused)/PAGE_SIZE
    }
}

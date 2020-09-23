use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

#[global_allocator]
pub static GLOBAL: TrackAllocator<System> = TrackAllocator {
    allocator: System,
    counter: AtomicUsize::new(0)
};


pub struct TrackAllocator<A> where A: GlobalAlloc {
    allocator: A,
    counter: AtomicUsize
}

impl<A> TrackAllocator<A> where A: GlobalAlloc {
    pub fn reset(&self) {
        self.counter.store(0, Ordering::SeqCst)
    }

    pub fn get(&self) -> usize {
        self.counter.load(Ordering::SeqCst)
    }
}

unsafe impl<A> GlobalAlloc for TrackAllocator<A> where A: GlobalAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret = self.allocator.alloc(layout);
        if !ret.is_null() {
            self.counter.fetch_add(layout.size(), Ordering::SeqCst);
        }
        ret
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.allocator.dealloc(ptr, layout);
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        self.counter.fetch_add(layout.size(), Ordering::SeqCst);
        self.allocator.alloc_zeroed(layout)
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        if new_size > layout.size() {
            self.counter.fetch_add(new_size - layout.size(), Ordering::SeqCst);
        } else if new_size < layout.size() {
            self.counter.fetch_sub(layout.size() - new_size, Ordering::SeqCst);
        }
        self.allocator.realloc(ptr, layout, new_size)
    }
}
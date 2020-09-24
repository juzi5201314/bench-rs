use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

#[global_allocator]
pub static GLOBAL: TrackAllocator<System> = TrackAllocator {
    allocator: System,
    counter: AtomicUsize::new(0),
    peak: AtomicUsize::new(0)
};


pub struct TrackAllocator<A> where A: GlobalAlloc {
    allocator: A,
    counter: AtomicUsize,
    peak: AtomicUsize
}

impl<A> TrackAllocator<A> where A: GlobalAlloc {
    #[allow(dead_code)]
    pub fn get(&self) -> usize {
        self.counter.load(Ordering::SeqCst)
    }

    pub fn counter(&'static self) -> &'static AtomicUsize {
        &self.counter
    }

    pub fn peak(&'static self) -> &'static AtomicUsize {
        &self.peak
    }

    #[inline]
    fn add(&self, u: usize) {
        self.counter.fetch_add(u, Ordering::SeqCst);
        self.check_peak();
    }

    #[inline]
    fn sub(&self, u: usize) {
        self.counter.fetch_sub(u, Ordering::SeqCst);
    }

    #[inline]
    fn check_peak(&self) {
        self.peak.fetch_max(self.get(), Ordering::SeqCst);
    }
}

unsafe impl<A> GlobalAlloc for TrackAllocator<A> where A: GlobalAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret = self.allocator.alloc(layout);
        if !ret.is_null() {
            self.add(layout.size());
        }
        ret
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if self.get() > layout.size() {
            self.sub(layout.size());
        }
        self.allocator.dealloc(ptr, layout);
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        self.add(layout.size());
        self.allocator.alloc_zeroed(layout)
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        if new_size > layout.size() {
            self.add(new_size - layout.size());
        } else if new_size < layout.size() {
            self.sub(layout.size() - new_size);
        }
        self.allocator.realloc(ptr, layout, new_size)
    }
}
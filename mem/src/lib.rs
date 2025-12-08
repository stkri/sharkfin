#![no_std]

use core::alloc::{self, GlobalAlloc, Layout};
use spin::Mutex;

unsafe extern "C" {
    static __heap_start: u8;
}

#[global_allocator]
pub static KALLLOCATOR: KAllocator = KAllocator;

pub static BALLOCATOR: BuddyAllocator = BuddyAllocator {
    inner: Mutex::new(BuddyAllocatorInner {
        free_lists: [FreeList { head: None }; MAX_ORDER + 1],
    }),
};

pub static SALLOCATOR: SlabAllocator = SlabAllocator {
    inner: Mutex::new(SlabAllocatorInner {
        free_lists: [FreeList { head: None }; 9],
    }),
};

pub fn init_allocator() {
    {
        let mut binner = BALLOCATOR.inner.lock();
        *binner = BuddyAllocatorInner::new();
    } // DO NOT REMOVE THESE: PREVENTS A DEADLOCK
    let mut sinner = SALLOCATOR.inner.lock();
    *sinner = unsafe { SlabAllocatorInner::new() };
}

const HEAP_END: usize = 0x3F000000;

pub fn heap_size() -> usize {
    let heap_start = unsafe { &__heap_start as *const u8 as usize };
    HEAP_END - heap_start
}

const MIN_BLOCK_SIZE: usize = 4096;
const MAX_ORDER: usize = 18;

pub struct BuddyAllocator {
    inner: Mutex<BuddyAllocatorInner>,
}

struct BuddyAllocatorInner {
    free_lists: [FreeList; MAX_ORDER + 1],
}

#[derive(Clone, Copy)]
struct FreeList {
    head: Option<*mut FreeBlock>,
}

#[repr(C)]
struct FreeBlock {
    next: Option<*mut FreeBlock>,
}

unsafe impl Send for FreeList {}

impl BuddyAllocator {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(BuddyAllocatorInner::new()),
        }
    }
}

impl BuddyAllocatorInner {
    pub fn new() -> Self {
        let heap_start = unsafe { &__heap_start as *const u8 as usize };
        let mut order = 0;
        let mut offset: usize = 0;
        let mut free_lists: [FreeList; MAX_ORDER + 1] = [FreeList { head: None }; MAX_ORDER + 1];
        {
            let mut heap_size = heap_size() >> MIN_BLOCK_SIZE.trailing_zeros();
            while heap_size != 0 {
                if heap_size & 1 == 1 {
                    free_lists[order].head = Some((heap_start + offset) as *mut FreeBlock);
                    unsafe {
                        *((heap_start + offset) as *mut FreeBlock) = FreeBlock { next: None }
                    };
                    offset += (1 << order) * MIN_BLOCK_SIZE
                }
                heap_size >>= 1;
                order += 1;
            }
        }

        Self { free_lists }
    }

    fn is_free(&self, addr: usize, order: usize) -> bool {
        let target = addr as *mut FreeBlock;
        let mut current = self.free_lists[order].head;

        while let Some(node) = current {
            if node == target {
                return true;
            }
            current = unsafe { (*node).next };
        }

        false
    }

    fn add_to_free_list(&mut self, addr: usize, order: usize) {
        let block = addr as *mut FreeBlock;
        unsafe {
            (*block).next = self.free_lists[order].head;
            self.free_lists[order].head = Some(block);
        }
    }

    fn remove_from_free_list(&mut self, addr: usize, order: usize) {
        let target = addr as *mut FreeBlock;

        if self.free_lists[order].head == Some(target) {
            self.free_lists[order].head = unsafe { (*target).next };
            return;
        }

        let mut current = self.free_lists[order].head;
        while let Some(node) = current {
            unsafe {
                if (*node).next == Some(target) {
                    (*node).next = (*target).next;
                    return;
                }
                current = (*node).next;
            }
        }
    }
}

unsafe impl GlobalAlloc for BuddyAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let mut allocator = self.inner.lock();
        let size = layout.size().max(MIN_BLOCK_SIZE);
        let size_power = size.next_power_of_two().trailing_zeros();
        let order = (size_power - MIN_BLOCK_SIZE.trailing_zeros()) as usize;
        let mut i = order as usize;
        loop {
            if i > MAX_ORDER {
                return core::ptr::null_mut();
            }
            match allocator.free_lists[i].head {
                None => i += 1,
                Some(p) => {
                    allocator.remove_from_free_list(p as usize, i);
                    let current_block_addr = p as usize;
                    let mut current_order = i;

                    while current_order > order as usize {
                        let block_size = (1 << (current_order - 1)) * MIN_BLOCK_SIZE;
                        let buddy_addr = current_block_addr + block_size;

                        let buddy = buddy_addr as *mut FreeBlock;
                        unsafe {
                            (*buddy).next = allocator.free_lists[current_order - 1].head;
                            allocator.free_lists[current_order - 1].head = Some(buddy);
                        }
                        current_order -= 1;
                    }

                    return current_block_addr as *mut u8;
                }
            }
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        let mut allocator = self.inner.lock();
        let size = layout.size().max(MIN_BLOCK_SIZE);
        let size_power = size.next_power_of_two().trailing_zeros();
        let mut order = (size_power - MIN_BLOCK_SIZE.trailing_zeros()) as usize;
        let mut current_addr = ptr as usize;

        loop {
            if order > MAX_ORDER {
                allocator.add_to_free_list(current_addr, MAX_ORDER);
                break;
            }
            let block_size = (1 << order) * MIN_BLOCK_SIZE;
            let buddy_addr = current_addr ^ block_size;

            if allocator.is_free(buddy_addr, order) {
                allocator.remove_from_free_list(buddy_addr, order);

                current_addr = current_addr.min(buddy_addr);
                order += 1;
            } else {
                allocator.add_to_free_list(current_addr, order);
                break;
            }
        }
    }
}

// 8 16 32 64 128 256 512 1024 2048

pub struct SlabAllocator {
    inner: Mutex<SlabAllocatorInner>,
}

struct SlabAllocatorInner {
    free_lists: [FreeList; 9],
}

impl SlabAllocatorInner {
    pub unsafe fn new() -> Self {
        let mut free_lists = [FreeList { head: None }; 9];
        for i in 3..=11 {
            let first_ptr = unsafe {
                BALLOCATOR.alloc(Layout::from_size_align_unchecked(4096, 4096)) as *mut FreeBlock
            };
            let ptr_size = 1 << i;
            unsafe { *first_ptr = FreeBlock { next: None } };
            for j in 1..(4096 / ptr_size) {
                let ptr = (first_ptr as usize + j * ptr_size) as *mut FreeBlock;
                let next_ptr = (first_ptr as usize + (j - 1) * ptr_size) as *mut FreeBlock;
                unsafe {
                    *ptr = FreeBlock {
                        next: Some(next_ptr),
                    }
                };
            }
            free_lists[i - 3] = FreeList {
                head: Some((first_ptr as usize + 4096 - ptr_size) as *mut FreeBlock),
            }
        }

        Self { free_lists }
    }
}

impl SlabAllocator {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(unsafe { SlabAllocatorInner::new() }),
        }
    }
}

unsafe impl GlobalAlloc for SlabAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.size() > 2048 {
            return core::ptr::null_mut();
        }
        let size = layout.size().max(8).next_power_of_two();
        let size_slab = (size.trailing_zeros() - 3) as usize;
        let mut allocator = self.inner.lock();
        if allocator.free_lists[size_slab].head == None {
            let first_ptr = unsafe {
                BALLOCATOR.alloc(Layout::from_size_align_unchecked(4096, 4096)) as *mut FreeBlock
            };
            unsafe { *first_ptr = FreeBlock { next: None } };
            for j in 1..(4096 / size) {
                let ptr = (first_ptr as usize + j * size) as *mut FreeBlock;
                let next_ptr = (first_ptr as usize + (j - 1) * size) as *mut FreeBlock;
                unsafe {
                    *ptr = FreeBlock {
                        next: Some(next_ptr),
                    }
                };
            }
            allocator.free_lists[size_slab] = FreeList {
                head: Some((first_ptr as usize + 4096 - size) as *mut FreeBlock),
            };
        }
        match allocator.free_lists[size_slab].head {
            None => return core::ptr::null_mut(),
            Some(p) => {
                allocator.free_lists[size_slab].head = unsafe { (*p).next };
                return p as *mut u8;
            }
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let size = layout.size().max(8).min(2048).next_power_of_two();
        let size_slab = (size.trailing_zeros() - 3) as usize;
        let mut allocator = self.inner.lock();
        let ptr = ptr as *mut FreeBlock;
        unsafe { (*ptr).next = allocator.free_lists[size_slab].head }
        allocator.free_lists[size_slab].head = Some(ptr);
    }
}

pub struct KAllocator;

unsafe impl GlobalAlloc for KAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe {
            if layout.size() > 2048 {
                BALLOCATOR.alloc(layout)
            } else {
                SALLOCATOR.alloc(layout)
            }
        }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe {
            if layout.size() > 2048 {
                BALLOCATOR.dealloc(ptr, layout);
            } else {
                SALLOCATOR.dealloc(ptr, layout);
            }
        }
    }
}

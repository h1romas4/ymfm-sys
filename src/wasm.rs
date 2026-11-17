use std::alloc::{Layout, alloc, dealloc};
use std::ffi::c_void;
use std::ptr::null_mut;

const MIN_ALIGNMENT: usize = 16;

#[repr(C)]
struct AllocationHeader {
    size: usize,
    alignment: usize,
    offset: usize,
}

fn allocate(size: usize, alignment: usize) -> *mut c_void {
    if !alignment.is_power_of_two() {
        return null_mut();
    }
    let alignment = alignment.max(MIN_ALIGNMENT);
    let Some(offset) = size_of::<AllocationHeader>()
        .checked_add(alignment - 1)
        .map(|size| size & !(alignment - 1))
    else {
        return null_mut();
    };
    let Some(total) = size.max(1).checked_add(offset) else {
        return null_mut();
    };
    let Ok(layout) = Layout::from_size_align(total, alignment) else {
        return null_mut();
    };
    let allocation = unsafe { alloc(layout) };
    if allocation.is_null() {
        return null_mut();
    }
    let ptr = unsafe { allocation.add(offset) };
    unsafe {
        ptr.sub(size_of::<AllocationHeader>())
            .cast::<AllocationHeader>()
            .write(AllocationHeader {
                size: layout.size(),
                alignment,
                offset,
            });
    }
    ptr.cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn malloc(size: usize) -> *mut c_void {
    allocate(size, MIN_ALIGNMENT)
}

#[unsafe(no_mangle)]
pub extern "C" fn aligned_alloc(alignment: usize, size: usize) -> *mut c_void {
    allocate(size, alignment)
}

/// # Safety
/// `ptr` must be null or a pointer returned by `malloc` or `aligned_alloc`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn free(ptr: *mut c_void) {
    if ptr.is_null() {
        return;
    }

    let ptr = ptr.cast::<u8>();
    let header = unsafe {
        ptr.sub(size_of::<AllocationHeader>())
            .cast::<AllocationHeader>()
            .read()
    };
    let layout = Layout::from_size_align(header.size, header.alignment)
        .expect("allocation stored an invalid layout");
    unsafe { dealloc(ptr.sub(header.offset), layout) };
}

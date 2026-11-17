#[cfg(all(
    target_arch = "wasm32",
    target_os = "unknown",
    target_vendor = "unknown"
))]
fn main() {
    use std::cell::Cell;
    use std::ffi::c_void;
    use std::rc::Rc;
    use ymfm_sys::{InterfaceCallbacks, InterfaceHandler, ffi};

    unsafe extern "C" {
        fn malloc(size: usize) -> *mut c_void;
        fn aligned_alloc(alignment: usize, size: usize) -> *mut c_void;
        fn free(ptr: *mut c_void);
    }

    unsafe {
        free(std::ptr::null_mut());
        for size in [0, 1, 16, 1024] {
            let ptr = malloc(size);
            assert!(!ptr.is_null());
            assert_eq!(ptr as usize % 16, 0);
            if size > 0 {
                ptr.cast::<u8>().write_volatile(42);
                assert_eq!(ptr.cast::<u8>().read_volatile(), 42);
            }
            free(ptr);
        }
        for alignment in [1, 16, 64, 4096] {
            let ptr = aligned_alloc(alignment, alignment.max(64));
            assert!(!ptr.is_null());
            assert_eq!(ptr as usize % alignment.max(16), 0);
            ptr.cast::<u8>().write_volatile(42);
            assert_eq!(ptr.cast::<u8>().read_volatile(), 42);
            free(ptr);
        }
        let malloc_fn = std::hint::black_box(malloc as unsafe extern "C" fn(usize) -> *mut c_void);
        let aligned_alloc_fn = std::hint::black_box(
            aligned_alloc as unsafe extern "C" fn(usize, usize) -> *mut c_void,
        );
        assert!(malloc_fn(usize::MAX).is_null());
        assert!(aligned_alloc_fn(64, usize::MAX - 63).is_null());
    }

    let clocks = Rc::new(Cell::new(0));
    let callback_clocks = Rc::clone(&clocks);
    let callbacks = InterfaceCallbacks::new(InterfaceHandler {
        advance_clock: Some(Box::new(move |elapsed| {
            callback_clocks.set(callback_clocks.get() + elapsed);
            0
        })),
        ..Default::default()
    });
    let mut chip =
        ffi::create_chip_with_callbacks(ffi::ChipType::Ym2612, 7_670_000, Box::new(callbacks));
    chip.pin_mut().reset();
    let saved = chip.pin_mut().save_state();
    assert!(!saved.is_empty());
    chip.pin_mut().generate(&mut [0; 32]);
    assert!(clocks.get() > 0);
    chip.pin_mut().restore_state(&saved);
    assert_eq!(chip.pin_mut().save_state(), saved);
}

#[cfg(not(all(
    target_arch = "wasm32",
    target_os = "unknown",
    target_vendor = "unknown"
)))]
fn main() {}

#[cfg(all(
    target_arch = "wasm32",
    target_os = "unknown",
    target_vendor = "unknown"
))]
#[unsafe(no_mangle)]
pub extern "C" fn test_abort_traps() {
    unsafe extern "C" {
        fn abort() -> !;
    }
    unsafe { abort() }
}

#[cfg(all(
    target_arch = "wasm32",
    target_os = "unknown",
    target_vendor = "unknown"
))]
#[unsafe(no_mangle)]
pub extern "C" fn test_assertion_traps() {
    unsafe extern "C" {
        fn __assert2(file: *const u8, line: i32, function: *const u8, expression: *const u8) -> !;
    }
    unsafe {
        __assert2(
            c"file".as_ptr().cast(),
            1,
            c"function".as_ptr().cast(),
            c"expression".as_ptr().cast(),
        )
    }
}

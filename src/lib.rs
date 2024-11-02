#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;
use core::panic::PanicInfo;
use core::slice::from_raw_parts;
use object::{File, Object, ObjectSegment};

#[panic_handler]
unsafe fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[global_allocator]
static ALLOCATOR: Allocator = Allocator;

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("Allocation error: {:?}", layout);
}

struct Allocator;

static mut MALLOC: Option<extern "C" fn(usize) -> *mut c_void> = None;
static mut FREE: Option<extern "C" fn(*mut c_void)> = None;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        MALLOC.unwrap()(layout.size()) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        FREE.unwrap()(ptr as *mut c_void);
    }
}

#[repr(C)]
pub struct Segment {
    address: u64,
    size: u64,
    data: *const u8,
}

impl Segment {
    pub fn new(address: u64, size: u64, data: *const u8) -> Self {
        Self { address, size, data }
    }
}

#[repr(C)]
pub enum ParseElfError {
    None = 0,
    InvalidElfData,
    FailedToGetSegmentData,
    AllocFunctionNotProvided,
}

#[no_mangle]
pub unsafe extern "C" fn parse_elf(
    elf_data: *const u8,
    elf_size: usize,
    callback: extern "C" fn(segment: Segment),
    malloc: Option<extern "C" fn(usize) -> *mut c_void>,
    free: Option<extern "C" fn(*mut c_void)>,
) -> ParseElfError {
    if malloc.is_none() || free.is_none() {
        return ParseElfError::AllocFunctionNotProvided;
    }

    MALLOC = malloc;
    FREE = free;

    let buffer = from_raw_parts(elf_data, elf_size);

    let binary = match File::parse(buffer) {
        Ok(file) => file,
        Err(_) => return ParseElfError::InvalidElfData,
    };

    for segment in binary.segments() {
        let data = match segment.data() {
            Ok(d) => d,
            Err(_) => return ParseElfError::FailedToGetSegmentData,
        };

        callback(Segment::new(segment.address(), segment.size(), data.as_ptr()));
    }

    ParseElfError::None
}

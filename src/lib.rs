#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::slice::from_raw_parts;

use elf::abi::PT_LOAD;
use elf::endian::NativeEndian;
use elf::ElfBytes;

#[panic_handler]
unsafe fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[repr(C)]
pub struct ElfSegment {
    address: usize,
    size: usize,
    data: *const u8,
}

#[repr(C)]
pub enum ElfParseResult {
    EntryPoint(usize),
    InvalidElfData,
    ElfContainsNoSegments,
    FailedToGetSegmentData,
    AllocFunctionNotProvided,
}

#[no_mangle]
pub unsafe extern "C" fn parse_elf(
    elf_data: *const u8,
    elf_size: usize,
    mapping_callback: extern "C" fn(segment: ElfSegment),
) -> ElfParseResult {
    let buffer = from_raw_parts(elf_data, elf_size);

    let elf_file = match ElfBytes::<NativeEndian>::minimal_parse(buffer) {
        Ok(elf) => elf,
        Err(_) => return ElfParseResult::InvalidElfData,
    };

    let parsing_table = match elf_file.segments() {
        Some(s) => s,
        None => return ElfParseResult::ElfContainsNoSegments,
    };

    for header in parsing_table {
        if header.p_type == PT_LOAD {
            let data = match elf_file.segment_data(&header) {
                Ok(d) => d,
                Err(_) => return ElfParseResult::FailedToGetSegmentData,
            };

            let segment = ElfSegment {
                address: header.p_vaddr as usize,
                size: header.p_memsz as usize,
                data: data.as_ptr(),
            };
            mapping_callback(segment);
        }
    }

    ElfParseResult::EntryPoint(elf_file.ehdr.e_entry as usize)
}

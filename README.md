# libelf-parse

C binding of `object` crate for x86 or x86_64 OS to parse ELF files easily.

## Usage

Download the header file and lib from [releases](https://github.com/plos-clan/libelf-parse/releases/tag/release).

Link the library to your project.

## Build

Build directly to get the two target files:

```bash
cargo build --release
```

The production build will be in `target/release/<target>/` directory.

And use `cbindgen` to generate the header file:

```bash
cargo install cbindgen
cbindgen --output elf_parse.h
```

## Example

```c
#include <stdint.h>
#include "elf_parse.h"

extern void *page_table;

void segment_callback(struct Segment segment) {
    printk("Segment address: 0x%lx\n", segment.address);
    printk("Segment size: %lu bytes\n", segment.size);
    alloc_and_map(page_table, segment.address, segment.size);
    write_to_mapped_address(page_table, segment.data, segment.size, segment.address);
}

void elf_load_example() {
    const uint8_t elf_data[] = {};
    size_t elf_size = sizeof(elf_data);

    struct ElfParseResult result = parse_elf(elf_data, elf_size, segment_callback);

    switch (result.tag) {
        case EntryPoint:
            printk("ELF Entry Point: 0x%lx\n", result.entry_point);
            break;
        case InvalidElfData:
            printk("Invalid ELF data.\n");
            break;
        case ElfContainsNoSegments:
            printk("ELF contains no segments.\n");
            break;
        case FailedToGetSegmentData:
            printk("Failed to get segment data.\n");
            break;
        case AllocFunctionNotProvided:
            printk("Allocation function not provided.\n");
            break;
        default:
            printk("Unknown error.\n");
            break;
    }
}
```

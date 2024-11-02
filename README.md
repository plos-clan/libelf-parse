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

# multiboot2

[![crates.io](https://img.shields.io/crates/v/multiboot2.svg)](https://crates.io/crates/multiboot2)
[![docs](https://docs.rs/multiboot2/badge.svg)](https://docs.rs/multiboot2/)

Convenient and safe parsing of Multiboot2 Boot Information (MBI)
structures and the contained information tags. Usable in `no_std` environments,
such as a kernel. An optional `builder` feature also allows the construction of
the corresponding structures.

It follows the Multiboot 2.0 specification
at https://www.gnu.org/software/grub/manual/multiboot2/multiboot.html and the
ELF 64 specification at http://www.uclibc.org/docs/elf-64-gen.pdf.

## Design

For every Multiboot2 structure, there is an ABI-compatible rusty type. This
enables a zero-copying parsing design while also enabling the creation of these
structures via convenient constructors on the corresponding types.

## Features and `no_std` Compatibility

This library is always `no_std` without `alloc`. However, the default `builder`-
feature requires the `alloc`-crate and an `#[global_allocator]` to be available.
You need the `builder` only if you want to construct new boot information
structures at runtime. For parsing, this is not relevant, and you can
deactivate the default features.

## Background: The Multiboot 2 Information Structure

The Multiboot information structure looks like this:

 Field            | Type
------------------|------------
 total size       | u32
 reserved         | u32
 tags             | variable
 end tag = (0, 8) | (u32, u32)

There are many different types of tags, but they all have the same beginning:

 Field        | Type
--------------|----------
 type         | u32
 size         | u32
 other fields | variable

## MSRV

The MSRV is 1.81.0 stable.

## License & Contribution

See main [README](https://github.com/rust-osdev/multiboot2/blob/main/README.md)
file.

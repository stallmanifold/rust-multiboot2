//! Multiboot v2 library
//!
//! # Additional documentation
//!   * http://nongnu.askapache.com/grub/phcoder/multiboot.pdf
//!
//!

#![no_std]

/// TODO: Add sanity checks for strings, i.e. Check that they are null terminated.
mod tag;
mod util;
mod basic_memory_information;
mod boot_loader_name;
mod boot_command_line;
mod bios_boot_device;
mod apm_table;
mod module;
mod memory_map;
mod elf_sections;
mod end_tag;

pub mod multiboot;

pub use multiboot::load;

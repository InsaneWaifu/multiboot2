//! Parsing the Multiboot information. Glue code for the [`multiboot`] code.

use anyhow::anyhow;
use core::ptr::addr_of_mut;
use core::slice;
use multiboot::information::{MemoryManagement, Multiboot, PAddr, SIGNATURE_EAX};

static mut MEMORY_MANAGEMENT: Mem = Mem;

/// Returns an object to access the fields of the Multiboot information
/// structure.
pub fn get_mbi<'a>(magic: u32, ptr: u32) -> anyhow::Result<Multiboot<'a, 'static>> {
    if magic != SIGNATURE_EAX {
        return Err(anyhow!("Unknown Multiboot signature {magic:x}"));
    }
    let mmgmt: &mut dyn MemoryManagement = unsafe { &mut *addr_of_mut!(MEMORY_MANAGEMENT) };
    unsafe { Multiboot::from_ptr(ptr as u64, mmgmt) }.ok_or(anyhow!(
        "Can't read Multiboot boot information from pointer"
    ))
}

/// Glue object between the global allocator and the multiboot crate.
struct Mem;

impl MemoryManagement for Mem {
    unsafe fn paddr_to_slice(&self, addr: PAddr, size: usize) -> Option<&'static [u8]> {
        let ptr = addr as *const u64 as *const u8;
        Some(unsafe { slice::from_raw_parts(ptr, size) })
    }

    // If you only want to read fields, you can simply return `None`.
    unsafe fn allocate(&mut self, _length: usize) -> Option<(PAddr, &mut [u8])> {
        None
    }

    unsafe fn deallocate(&mut self, addr: PAddr) {
        if addr != 0 {
            unimplemented!()
        }
    }
}

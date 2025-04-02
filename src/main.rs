#![no_std]
#![no_main]

use core::fmt::Write;
use core::panic::PanicInfo;
use uefi::boot::{MemoryDescriptor, MemoryType};
use uefi::mem::memory_map::{MemoryMap, MemoryMapOwned};
use uefi::prelude::*;
use uefi::table::runtime::ResetType;

#[entry]
fn efi_main(_image: Handle, mut st: SystemTable<Boot>) -> Status {
    // logging, memory allocationの初期化
    uefi::helpers::init().unwrap();

    st.stdout().reset(false).unwrap();

    writeln!(st.stdout(), "Hello, World!").unwrap();
    writeln!(st.stdout(), "Memory map:").unwrap();

    writeln!(st.stdout(), "{:>20} {:>20} {:>20} {:>20} {:>20}", "Type", "Size", "Key", "DescSize", "DescVersion").unwrap();
    writeln!(st.stdout(), "{:>20} {:>20} {:>20} {:>20} {:>20}", "----", "----", "----", "----", "----").unwrap();

    let boot_services_code = get_memory_map(&st, MemoryType::BOOT_SERVICES_CODE).unwrap();
    let boot_services_data = get_memory_map(&st, MemoryType::BOOT_SERVICES_DATA).unwrap();
    let runtime_services_code = get_memory_map(&st, MemoryType::RUNTIME_SERVICES_CODE).unwrap();
    let runtime_services_data = get_memory_map(&st, MemoryType::RUNTIME_SERVICES_DATA).unwrap();
    let reserved = get_memory_map(&st, MemoryType::RESERVED).unwrap();
    let loader_code = get_memory_map(&st, MemoryType::LOADER_CODE).unwrap();
    let loader_data = get_memory_map(&st, MemoryType::LOADER_DATA).unwrap();

    writeln!(st.stdout(), "{:>20} {:>20} {:>?} {:>20} {:>20}", "BOOT_SERVICES_CODE", boot_services_code.len(), boot_services_code.meta().map_key, boot_services_code.meta().desc_size, boot_services_code.meta().desc_version).unwrap();
    writeln!(st.stdout(), "{:>20} {:>20} {:>?} {:>20} {:>20}", "BOOT_SERVICES_DATA", boot_services_data.len(), boot_services_data.meta().map_key, boot_services_data.meta().desc_size, boot_services_data.meta().desc_version).unwrap();
    writeln!(st.stdout(), "{:>20} {:>20} {:>?} {:>20} {:>20}", "RUNTIME_SERVICES_CODE", runtime_services_code.len(), runtime_services_code.meta().map_key, runtime_services_code.meta().desc_size, runtime_services_code.meta().desc_version).unwrap();
    writeln!(st.stdout(), "{:>20} {:>20} {:>?} {:>20} {:>20}", "RUNTIME_SERVICES_DATA", runtime_services_data.len(), runtime_services_data.meta().map_key, runtime_services_data.meta().desc_size, runtime_services_data.meta().desc_version).unwrap();
    writeln!(st.stdout(), "{:>20} {:>20} {:>?} {:>20} {:>20}", "RESERVED", reserved.len(), reserved.meta().map_key, reserved.meta().desc_size, reserved.meta().desc_version).unwrap();
    writeln!(st.stdout(), "{:>20} {:>20} {:>?} {:>20} {:>20}", "LOADER_CODE", loader_code.len(), loader_code.meta().map_key, loader_code.meta().desc_size, loader_code.meta().desc_version).unwrap();
    writeln!(st.stdout(), "{:>20} {:>20} {:>?} {:>20} {:>20}", "LOADER_DATA", loader_data.len(), loader_data.meta().map_key, loader_data.meta().desc_size, loader_data.meta().desc_version).unwrap();

    writeln!(st.stdout(), "Memory map descriptors:").unwrap();
    writeln!(st.stdout(), "BOOT_SERVICES_CODE:").unwrap();
    for i in 0..boot_services_code.len() {
        let descriptor = boot_services_code.get(i).unwrap();
        writeln!(st.stdout(), "{:>6} {:>10} {:>6} {:>6} {:>?}", descriptor.ty.0, descriptor.phys_start, descriptor.virt_start, descriptor.page_count, descriptor.att).unwrap();
    }

    st.boot_services().stall(15_000_000);

    st.stdout().reset(false).unwrap();

    st.runtime_services()
        .reset(ResetType::SHUTDOWN, Status::SUCCESS, None);
}

fn get_memory_map(st: &SystemTable<Boot>, t: MemoryType) -> uefi::Result<MemoryMapOwned> {
    let map = st.boot_services()
        .memory_map(t)
        .map_err(|e| e)?;

    Ok(map)
}

fn get_memory_descriptor(st: &SystemTable<Boot>, t: MemoryType) -> uefi::Result<MemoryDescriptor> {
    let map = self::get_memory_map(st, t).unwrap();
    let descriptor = unsafe {
        let buffer = map.buffer();
        let ptr = buffer.as_ptr() as *const MemoryDescriptor;
        *ptr
    };

    Ok(descriptor)
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop{}
}

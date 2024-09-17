#![no_std]
#![no_main]

use core::fmt::Write;
use core::panic::PanicInfo;
use uefi::prelude::*;
use uefi::table::runtime::ResetType;

#[entry]
fn efi_main(_image: Handle, mut st: SystemTable<Boot>) -> Status {
    // logging, memory allocationの初期化
    uefi::helpers::init().unwrap();

    st.stdout().reset(false).unwrap();

    writeln!(st.stdout(), "Hello, World!").unwrap();

    st.boot_services().stall(3_000_000);

    st.stdout().reset(false).unwrap();

    st.runtime_services()
        .reset(ResetType::SHUTDOWN, Status::SUCCESS, None);
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop{}
}

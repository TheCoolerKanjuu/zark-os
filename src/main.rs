#![feature(alloc_error_handler)]
#![no_std]
#![no_main]

use uefi::entry;

use log::info;
use uefi::prelude::*;

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    info!("Hello world!");
    system_table.boot_services().stall(10_000_000);
    Status::SUCCESS
}

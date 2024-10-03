// Copyright (C) 2024 Bellande Architecture Mechanism Research Innovation Center, Ronaldson Bellande

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

#![no_std]
#![no_main]
#![feature(asm)]
#![feature(global_asm)]
#![feature(alloc_error_handler)]

extern crate alloc;

mod acpi;
mod bootloader;
mod cpu;
mod graphics;
mod interrupts;
mod memory;
mod pci;

use crate::acpi::init_acpi;
use crate::bootloader::load_bootloader;
use crate::cpu::init_cpu;
use crate::graphics::init_graphics;
use crate::interrupts::{init_gdt, init_idt, init_interrupt_controller};
use crate::memory::init_memory;
use crate::pci::enumerate_pci_devices;
use core::panic::PanicInfo;

global_asm!(include_str!("boot.s"));

#[no_mangle]
pub extern "C" fn bellande_boot_main() -> ! {
    init_cpu();
    init_memory();
    init_gdt();
    init_idt();
    init_interrupt_controller();
    init_acpi();
    enumerate_pci_devices();
    init_graphics();
    load_bootloader();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

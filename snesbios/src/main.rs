#![no_main]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![no_std]
#![feature(asm)]
#![feature(rust_intrinsics)]
#![feature(c_variadic)]
#![no_mangle]

extern crate snesdev;

pub mod syscall;

#[link_section = ".text.bios.startup"]
pub unsafe extern"C" fn __reset_bios_startup() -> !{

}

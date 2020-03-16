#![no_std]
#![feature(never_type)]
#![feature(asm)]
#![feature(core_intrinsics)]
#![feature(maybe_uninit_ref)]
#![feature(arbitrary_enum_discriminant)]

extern crate snesdev;

extern"C"{
    #[no_mangle]
    pub fn __shutdown() -> !;
    #[no_mangle]
    pub fn __restart() -> !;

    #[no_mangle]
    pub fn __user_call(proc: UserPtr<extern"C" fn()>);
}

use snesdev::volatile::*;
use snesdev::interrupts;
use crate::ptr::UserPtr;

mod user;
mod proc;
mod errors;
mod file;
mod syscall;
mod ptr;
mod map;
mod signals;
mod future;
mod set;

pub unsafe fn __kernel_init(kargc: usize,kargv: *& str) -> !{
    let kargs = core::slice::from_raw_parts(kargv,kargc);
    let kproc = proc::__kernel_proc();
    let root = kargs[1];

    loop{
       interrupts::wait_interrupt();
    }
}

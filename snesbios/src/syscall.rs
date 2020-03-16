use core::ffi::VaList;
use snesdev::volatile::AtomicCell;

#[no_mangle]
#[link_section=".bss.biossyms"]
pub static _SyscallAddr: AtomicCell<Option<*unsafe extern"C" fn(VaList)->i16>> = unsafe{core::mem::zeroed()};

extern{
    static _PtrefBase: *();
    static _PageRef: u16;
}

#[no_mangle]
#[link_section=".text.bioscalls"]
pub unsafe extern"C" fn __syscall(list: VaList) -> i16{
    *(_PtrefBase as *mut u16) = 0;
    let ret =
    if let Some(syscall) = _SyscallAddr.load(){
        syscall(list)
    }else{
        0i16
    };
    *(_PtrefBase as *mut u16) = _PageRef;
    ret
}
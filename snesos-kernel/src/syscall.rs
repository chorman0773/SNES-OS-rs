use crate::file;
use crate::ptr::{MutUserPtr, UserPtr};


#[repr(C,u16)]
#[derive(Copy,Clone)]
pub enum Syscall{
    Read(file::Fd,MutUserPtr<u8>,usize) = 0,
    Write(file::Fd,UserPtr<u8>,usize) = 1,
    Exit(i16) = 60
}

pub unsafe fn handle_syscall(call: Syscall) -> i16{
    match call{
        Syscall::Exit(code) =>{
            asm!("CLI"::::"volatile");
            asm!("WAI"::::"volatile");
            core::intrinsics::unreachable()
        }
        _ => {
            -1
        }
    }
}
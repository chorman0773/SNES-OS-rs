use crate::{file, proc};
use crate::ptr::{MutUserPtr, UserPtr};
use crate::errors::Errno;


#[repr(C,u16)]
#[derive(Copy,Clone)]
pub enum Syscall{
    Read(file::Fd,MutUserPtr<u8>,usize) = 0,
    Write(file::Fd,UserPtr<u8>,usize) = 1,
    Exit(i16) = 60
}

pub unsafe fn handle_syscall(call: Syscall) -> Result<u16,Errno>{
    match call{
        Syscall::Exit(code) =>{
            proc::current().close();
            asm!("CLI"::::"volatile");
            asm!("WAI"::::"volatile");
            core::intrinsics::unreachable()
        }
        _ => {
            Err(Errno::EBADSYS)
        }
    }
}
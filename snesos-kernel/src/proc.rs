use crate::user;
use crate::errors::Errno;
use snesdev::volatile::AtomicCell;
use std::mem::MaybeUninit;

#[derive(Copy,Clone,Eq,PartialEq)]
#[repr(transparent)]
pub struct Pid(NonZeroU16);

impl Pid{
    pub fn get_pid_value(&self) -> NonZeroU16{
        return self.0
    }

    pub const unsafe fn create_unchecked(id: u16) -> Self{
        Self(NonZeroU16::new_unchecked(id))
    }
}

use crate::set::*;


unit_enum!{
    pub unit enum ProcCap{
        CAP_AUDIT_CONTROL,
        CAP_AUDIT_READ,
        CAP_AUDIT_WRITE,
        CAP_BLOCK_SUSPEND,
        CAP_CHOWN,
        CAP_DAC_OVERRIDE,
        CAP_DAC_READ_SEARCH,
        CAP_FOWNER,
        CAP_FSETID,
        CAP_IPC_LOCK,
        CAP_IPC_OWNER,
        CAP_KILL,
        CAP_LEASE,
        CAP_LINUX_IMMUTABLE,
        CAP_MAC_OVERRIDE,
        CAP_MKNOD,
        CAP_NET_ADMIN,
        CAP_NET_BIND_SERVICE,
        CAP_NET_BROADCAST,
        CAP_NET_RAW,
        CAP_SETGID,
        CAP_SETFCAP,
        CAP_SETPCAP,
        CAP_SETUID,
        CAP_SYS_ADMIN,
        CAP_SYS_BOOT,
        CAP_SYS_CHROOT,
        CAP_SYS_MODULE,
        CAP_SYS_NICE,
        CAP_SYS_PACCT,
        CAP_SYS_PTRACE,
        CAP_SYS_RAWIO,
        CAP_SYS_RESOURCE,
        CAP_SYS_TIME,
        CAP_SYS_TTY_CONFIG,
        CAP_SYSLONG,
        CAP_WAKE_ALARM,
        // Enter LC Reserved Capabilities
        // These are inhabited for when we do lcnix and have caps for that
        CAP_LC_RES0,
        CAP_LC_RES1,
        CAP_LC_RES2,
        CAP_LC_RES3,
        CAP_LC_RES4,
        CAP_LC_RES5,
        CAP_LC_RES6,
        CAP_LC_RES7,
        CAP_LC_RES8,
        CAP_LC_RES9,
        CAP_LC_RES10,
        CAP_LC_RES11,
        CAP_LC_RES12,
        CAP_LC_RES13,
        CAP_LC_RES14,
        CAP_LC_RES15,
        // Enter SNES-OS Specific Capabilities
        CAP_SNESOS_OPEN_MAP,
        CAP_SNESOS_PWR_RESET,
        CAP_SNESOS_CTRL_BUS_ACCESS,
        CAP_SNESOS_RES3,
        CAP_SNESOS_RES4,
        CAP_SNESOS_RES5,
        CAP_SNESOS_RES6,
        CAP_SNESOS_RES7
    }
}

impl ProcCap{
    pub fn get_inherit_capabilities(user: user::Uid) -> EnumSet<ProcCap>{
        if user.is_root(){
            EnumSet::all()
        }else{
            EnumSet::none()
        }
    }
}

pub use ProcCap::*;


static INIT_PID: Pid = unsafe{Pid::create_unchecked(1)};




use crate::user;
use crate::errors::Errno;
use snesdev::volatile::AtomicCell;
use std::mem::MaybeUninit;

#[derive(Copy,Clone,Eq,PartialEq,Default)]
#[repr(transparent)]
pub struct Pid(u16);

impl Pid{
    pub fn get_pid_value(&self) -> u16{
        return self.0
    }
}


static KERNEL_PID: Pid = Pid(0);
static INIT_PID: Pid = Pid(1);

#[repr(C)]
#[derive(Copy,Clone,Default)]
pub struct ProcPermissions{
    uid: user::Uid,
    gid: user::Gid,
    euid: user::Uid,
    egid: user::Gid,
    suid: user::Uid,
    sgid: user::Gid
}

impl ProcPermissions{
    pub fn is_root(&self) -> bool{
        return euid.is_root()
    }
    pub fn get_uid(&self) -> user::Uid{
        return uid
    }
    pub fn get_gid(&self) -> user::Gid{
        return gid
    }
    pub fn get_euid(&self) -> user::Uid{
        return euid
    }
    pub fn get_egid(&self) -> user::Gid{
        return egid
    }
    pub fn set_uid(&mut self,n_uid: user::Uid){
        self.suid = self.euid;
        self.euid = n_uid;
    }
    pub fn set_gid(&mut self,n_gid: user::Gid){
        self.sgid = self.egid;
        self.egid = n_gid;
    }
}

#[repr(C)]
#[derive(Default)]
struct ProcDescriptor{
    permissions: ProcPermissions,
    pid: Pid,
    ppid: Pid,
    pgid: Pid,
    pthead_pg: usize,
    pexec: FileHandle
}

impl ProcDescriptor{
    pub fn get_permissions(&self) -> &ProcPermissions{
        return &self.permissions
    }
    pub fn get_pid(&self) -> Pid{
        return self.pid
    }
    pub fn get_parent(&self) -> &ProcDescriptor{
        unsafe { PROCS[self.ppid].get_ref()}
    }
}

static mut PROCS: [Option<ProcDescriptor>;1024] = [None;1024];
static mut CUR_PID: Pid = Pid(0);

pub fn current() ->&'static mut ProcDescriptor{
    return unsafe{PROCS[CUR_PID.0].as_mut().unwrap()}
}

pub fn __kernel_proc() -> &'static mut ProcDescriptor{
    return unsafe { PROCS }[0].get_or_insert(Default::default());
}


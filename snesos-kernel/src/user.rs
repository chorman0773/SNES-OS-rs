
#[derive(Copy,Clone,Eq,PartialEq,Default)]
#[repr(transparent)]
pub struct Uid(pub u16);

#[derive(Copy,Clone,Eq,PartialEq,Default)]
#[repr(transparent)]
pub struct Gid(pub u16);

static ROOT_UID: Uid = Uid(0);
static ROOT_GID: Gid = Gid(0);


impl Uid{
    pub fn is_root(&self) -> bool{
        return self==ROOT_UID
    }
}

impl Gid{
    pub fn is_root(&self) -> bool{
        return self==ROOT_GID
    }
}


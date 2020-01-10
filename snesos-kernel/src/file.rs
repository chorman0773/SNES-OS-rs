
#[repr(transparent)]
#[derive(Copy,Clone,Eq,PartialEq)]
pub struct Fd(pub u16);

pub static INVALID: Fd = Fd(0xFFFF);
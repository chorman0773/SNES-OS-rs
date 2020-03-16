

use snesdev::pointer::PackedPtr;

#[repr(transparent)]
#[derive(Copy,Clone)]
pub struct UserPtr<T>(pub *T);

impl<T> UserPtr<T>{
    pub fn cast<U>(self) -> UserPtr<U>{
        return UserPtr(self.0.cast::<U>())
    }
    pub fn page(&self) -> u16{
        let packed = PackedPtr.from(self.0);
        let bank = packed.1;
        let addr = packed.0;
        return (bank as u16)<<4 | (addr&0xF000)>>12;
    }
}

#[derive(Copy,Clone)]
pub struct MutUserPtr<T>(pub *mut T);

impl<T> MutUserPtr<T>{
    pub fn cast<U>(self) -> UserMutPtr<U>{
        return UserMutPtr(self.0.cast::<U>())
    }
    pub fn page(&self) -> u16{
        let packed = PackedPtr.from(self.0);
        let bank = packed.1;
        let addr = packed.0;
        return (bank as u16)<<4 | (addr&0xF000)>>12;
    }
}

impl<T> From<MutUserPtr<T>> for UserPtr<T>{
    fn from(val: MutUserPtr<T>) -> Self {
        Self(val.0)
    }
}

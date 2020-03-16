use snesdev::volatile::AtomicCell;
use core::future::Future;
use core::task::{Context, Poll};
use core::pin::Pin;

struct FutureEvent{
    complete: AtomicCell<bool>
}
impl FutureEvent{
    fn complete(&self){
        self.complete.store(true)
    }
    fn wait_for(&self){
        while self.complete.load(){
            unsafe{ asm!("WAI"::::"volatile");}
        }
    }
}

impl Future for FutureEvent{
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.complete.load(){
            return Poll::Ready(())
        }else{
            Poll::Pending
        }
    }
}

use std::{
    future::Future,
    sync::{Arc, Condvar, Mutex},
    task::{Context, Poll, Wake, Waker, RawWaker, RawWakerVTable},
    pin::Pin,
};

async fn demo() {
    println!("hello");
}

struct Demo;

fn block_on<F: Future + std::marker::Unpin>(future: F) -> F::Output {
    let mut fut: Pin<&mut F> = std::pin::Pin::new(&mut future);
    let waker: Waker = dummy_waker();
    let mut cx: Context<'_> = Context::from_waker(&waker);
    loop {
        if let Poll::Ready(output) = fut.as_mut().poll(&mut cx) {
            return output;
        }
    }
}

impl Future for Demo {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        println!("poll");
        std::task::Poll::Ready(())
    }
}

fn dummy_waker() -> Waker {
    static DATA: () = ();
    unsafe {
        Waker::from_raw(RawWaker::new(&DATA as *const _, &VTABLE))
    }
}

static VTABLE: RawWakerVTable = RawWakerVTable::new(vtable_clone, vtable_wake, vtable_wake_by_ref, vtable_drop);

unsafe fn vtable_clone(_p: *const ()) -> RawWaker {
    RawWaker::new(_p, &VTABLE)
}

unsafe fn vtable_wake(_p: *const ()) {}

unsafe fn vtable_wake_by_ref(_p: *const ()) {}

unsafe fn vtable_drop(_p: *const ()) {}

fn main() {
    block_on(demo());
}

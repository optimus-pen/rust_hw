use std::{
    future::Future,
    sync::{Arc, Condvar, Mutex},
    task::{Context, Poll, Wake, Waker, RawWaker, RawWakerVTable},
    pin::Pin,
};
use std::time::Duration;

struct Signal{
    state:Mutex<State>,
    cond:Condvar,
}

enum State{
    Empty,
    Waiting,
    Notified,
}

fn wait(&self){
    let mut state = self.state.lock().unwrap();
    match *state{
        State::Notified => *state=State::Empty,
        State::Waiting => {
            panic!("multiple wait");
        }
        State::Empty=>{
            *state=State::Waiting;
            while let State::Waiting=*state{
                state=self.cond.wait(state).unwrap();
            }
        }
    }
}

fn notify(&self){
    let mut state = self.state.lock().unwrap();
    match *state{
        State::Notified=>{}
        State::Empty=>*state=State::Notified,
        State::Waiting=>{
            *state = State::Empty;
            self.cond.notify_one();// call one thread
        }
    }
}

impl Wake for Signal{
    fn wake(self:Arc<Self>){
        self.notify();
    }
}

async fn demo(){
    let (tx,rx) = async_channel::bounded::<()>(1);
    std::thread::spawn(move||{
        std::thread::sleep(Duration::from_secs(20));
        tx.send_blocking(())
    });
    let _=rx.recv().await;
    println!("hello");
}

impl<W:Wake+Send+Sync+'static> From<Arc<W>> for Waker{
    fn from(waker:Arc<W>)->Waker{
        unsafe{Waker::from_raw(raw_waker(waker))}
    }
}

struct Demo;

fn block_on<F:Future>(future:F)->F::Output{
    let mut fut:Pin<&mut F> = std::pin::pin!(future);
    let signal = Arc::new(Signal::new());
    let waker = Waker::from(signal.clone());
    //let waker:Waker = dummy_waker();
    let mut cx:Context<'_>=Context::from_waker(&waker);
    loop{
        if let Poll::Ready(output) = fut.as_mut().poll(&mut cx){
            return output;
        }
        signal.wait();
    }
}

impl Future for Demo{
    type Output=();
    fn poll(self:std::pin::Pin<&mut Self>, _cx:&mut std::task::Context<'_>)->std::task::Poll<Self::Output> {
        println!("poll");
        std::task::Poll::Ready(())
    }
}

fn dummy_waker() ->Waker{
    static DATA:()=();
    unsafe{
        Waker::from_raw(RawWaker::new(&DATA as *const _, &VTABLE))
    }
}

const VTABLE:RawWakerVTable = RawWakerVTable::new(vtable_clone,vtable_wake,vtable_wake_by_ref,vtable_drop);

unsafe fn vtable_clone(_p:*const())->RawWaker{
    RawWaker::new(_p,&VTABLE)
}

unsafe fn vtable_wake(_p:*const()){}

unsafe fn vtable_wake_by_ref(_p:*const()){}

unsafe fn vtable_drop(_p:*const()){}


fn main() {
    block_on(demo());
}

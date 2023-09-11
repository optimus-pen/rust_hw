use async_std::task::spawn;
use futures::executor::block_on;
mod executor;
mod reactor;

async fn demo() {
    let (tx, rx) = async_channel::bounded(1);
    spawn(demo2(tx));
    println!("hello world1!");
    let _ = rx.recv().await;
}

async fn demo2(tx: async_channel::Sender<()>) {
    println!("hello world2!");
    let _ = tx.send(()).await;
}

fn main() {
    block_on(demo());
}
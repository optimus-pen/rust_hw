#![feature(impl_trait_in_assoc_type)]
use volo_example::{LogLayer, FilterLayer};
use std::{net::SocketAddr, collections::HashMap, sync::{Arc, Mutex}, string};
use volo_example::{KV};
#[volo::main]
async fn main() {
    let addr: SocketAddr = "[::]:8080".parse().unwrap();
    let addr = volo::net::Address::from(addr);
    volo_gen::volo::example::ItemServiceServer::new(KV{ map: Mutex::new(HashMap::new())})
        .layer_front(LogLayer)
        .layer_front(FilterLayer)
        .run(addr)
        .await
        .unwrap();
}
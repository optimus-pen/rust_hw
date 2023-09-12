use lazy_static::lazy_static;
use pilota::FastStr;
use tracing_subscriber::layer::Filter;
use volo_gen::volo::example::{RCommand, GetItemResponse};
use std::net::SocketAddr;
use std::io::{self, BufRead};
use std::string;
use volo_example::{LogLayer, FilterLayer};
lazy_static! {
    static ref CLIENT: volo_gen::volo::example::ItemServiceClient = {
        let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        volo_gen::volo::example::ItemServiceClientBuilder::new("my_redis")
            .layer_outer(LogLayer)
            .layer_outer(FilterLayer)
            .address(addr)
            .build()
    };
}
async fn handle(request: &str) {
    let args = request.split_whitespace().collect::<Vec<&str>>();
    let cmd = args[0];
    let args = &args[1..];
    let req = volo_gen::volo::example::GetItemRequest {
        cmd: match cmd {
            "Get" => RCommand::Get,
            "Set" => RCommand::Set,
            "Ping" => RCommand::Ping,
            "Del" => RCommand::Del,
            _ => RCommand::Unkonwn,
        },
        args: Some(args.iter().map(|s| FastStr::from(s.to_string())).collect()),
    };
    let resp = CLIENT.get_item(req).await.unwrap();
    match resp {
        GetItemResponse { ok, data } => {
            if ok {
                println!("{:?}", data.unwrap());
            } else {
                println!("Error: {:?}", data);
            }
        }
    }
}

#[volo::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let stdin = io::stdin();
    for lines in stdin.lock().lines() {
        let request = lines.unwrap();
        handle(&request).await;
    }
}

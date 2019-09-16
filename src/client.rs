mod protos;

use std::env;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::thread;
use std::time;
use std::vec::Vec;

use futures::{Future, Sink};
use grpcio::{ChannelBuilder, EnvBuilder, WriteFlags};

fn generate_data(size: usize) -> Vec<u8> {
    (0..size).map(|_| 0u8).collect()
}

static COUNT: AtomicU32 = AtomicU32::new(0);

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let addr = &args[1];
    let num = args[2].to_string().parse().unwrap();
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env)
        .stream_initial_window_size(20 * 1024 * 1024)
        .keepalive_time(time::Duration::from_secs(100 * 3600))
        .keepalive_timeout(time::Duration::from_secs(300 * 3600))
        .default_compression_algorithm(grpcio::CompressionAlgorithms::None)
        .tcp_max_read_chunk_size(1024)
        .connect(addr);
    println!("connected to server {}", addr);
    let client = Arc::new(protos::StreamServiceClient::new(ch));
    for i in 0..num {
        let client = client.clone();
        thread::spawn(move || {
            let (sender, receiver) = client.open_stream().unwrap();
            let mut tx = sender.wait();
            println!("stream opened {}", i);
            loop {
                let bytes = generate_data(1024 * 1024);
                let mut msg = protos::ClientStream::new();
                msg.set_data(bytes);
                println!("send data in {}", i);
                tx.send((msg, WriteFlags::default())).unwrap();
                tx.flush().unwrap();
                COUNT.fetch_add(1, Ordering::SeqCst);
                println!("done {} MB", COUNT.load(Ordering::SeqCst));
            }
        });
    }
    // thread::spawn(move || loop {
    //     let mut req = protos::HeartbeatRequest::new();
    //     req.set_greeting(String::from("PING"));
    //     let resp = client.heartbeat(&req).unwrap();
    //     println!("heartbeat {}", resp.get_status());
    //     thread::sleep(time::Duration::from_millis(100));
    // });
    thread::sleep(time::Duration::from_secs(100 * 3600));
}

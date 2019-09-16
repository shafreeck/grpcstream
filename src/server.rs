mod protos;

use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time;

use futures::{Future, Stream};
use futures_cpupool::CpuPool;
use grpcio::*;

enum Task {
    Recv {
        stream: RequestStream<protos::ClientStream>,
        sink: ClientStreamingSink<protos::Response>,
    },
}

fn run_task(t: Task, pool: Arc<CpuPool>) {
    match t {
        Task::Recv { stream, sink } => {
            println!("run task");
            let s = stream
                .for_each(|_| {
                    println!("receiving data");
                    return Ok(());
                })
                .and_then(|_| sink.success(protos::Response::new()));
            pool.spawn(s).forget();
            println!("task spawned");
        }
    }
}
fn sleep_task(t: Task, pool: Arc<CpuPool>) {
    thread::spawn(move || {
        thread::sleep(time::Duration::from_secs(100 * 3600));
        run_task(t, pool);
    });
}

#[derive(Clone)]
struct StreamServer {
    tx: mpsc::Sender<Task>,
}

impl protos::StreamService for StreamServer {
    fn open_stream(
        &mut self,
        ctx: RpcContext,
        stream: RequestStream<protos::ClientStream>,
        sink: ClientStreamingSink<protos::Response>,
    ) {
        println!("open stream");
        let t = Task::Recv { stream, sink };
        self.tx.send(t).unwrap();
    }
    fn heartbeat(
        &mut self,
        ctx: RpcContext,
        req: protos::HeartbeatRequest,
        sink: UnarySink<protos::Response>,
    ) {
        println!("heartbeat {}", req.get_greeting());
        let mut resp = protos::Response::new();
        resp.set_status(String::from("PONG"));
        sink.success(resp);
    }
}

fn main() {
    let env = Arc::new(Environment::new(1));
    let (tx, rx) = mpsc::channel();
    let service = protos::create_stream_service(StreamServer { tx: tx });
    let ch_args = ChannelBuilder::new(env.clone())
        //.http2_bdp_probe(false)
        .stream_initial_window_size(2 * 1024 * 1024)
        .max_concurrent_stream(1024)
        .max_send_message_len(10 * 1024 * 1024)
        .max_receive_message_len(-1)
        .build_args();
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("0.0.0.0", 8804)
        .channel_args(ch_args)
        .build()
        .unwrap();
    server.start();
    let pool = Arc::new(CpuPool::new(4));
    thread::spawn(move || {
        let mut i = 0;
        for task in rx.iter() {
            let pool = pool.clone();
            println!("received task");
            if i == 1000000 {
                run_task(task, pool);
            } else {
                sleep_task(task, pool);
            }
            i += 1;
        }
    });
    thread::sleep(time::Duration::from_secs(100 * 3600));
    server.shutdown().wait();
}

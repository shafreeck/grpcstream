// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_STREAM_SERVICE_OPEN_STREAM: ::grpcio::Method<super::stream::ClientStream, super::stream::Response> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ClientStreaming,
    name: "/stream.StreamService/OpenStream",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_STREAM_SERVICE_HEARTBEAT: ::grpcio::Method<super::stream::HeartbeatRequest, super::stream::Response> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/stream.StreamService/Heartbeat",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct StreamServiceClient {
    client: ::grpcio::Client,
}

impl StreamServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        StreamServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn open_stream_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::stream::ClientStream>, ::grpcio::ClientCStreamReceiver<super::stream::Response>)> {
        self.client.client_streaming(&METHOD_STREAM_SERVICE_OPEN_STREAM, opt)
    }

    pub fn open_stream(&self) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::stream::ClientStream>, ::grpcio::ClientCStreamReceiver<super::stream::Response>)> {
        self.open_stream_opt(::grpcio::CallOption::default())
    }

    pub fn heartbeat_opt(&self, req: &super::stream::HeartbeatRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::stream::Response> {
        self.client.unary_call(&METHOD_STREAM_SERVICE_HEARTBEAT, req, opt)
    }

    pub fn heartbeat(&self, req: &super::stream::HeartbeatRequest) -> ::grpcio::Result<super::stream::Response> {
        self.heartbeat_opt(req, ::grpcio::CallOption::default())
    }

    pub fn heartbeat_async_opt(&self, req: &super::stream::HeartbeatRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::stream::Response>> {
        self.client.unary_call_async(&METHOD_STREAM_SERVICE_HEARTBEAT, req, opt)
    }

    pub fn heartbeat_async(&self, req: &super::stream::HeartbeatRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::stream::Response>> {
        self.heartbeat_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait StreamService {
    fn open_stream(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::stream::ClientStream>, sink: ::grpcio::ClientStreamingSink<super::stream::Response>);
    fn heartbeat(&mut self, ctx: ::grpcio::RpcContext, req: super::stream::HeartbeatRequest, sink: ::grpcio::UnarySink<super::stream::Response>);
}

pub fn create_stream_service<S: StreamService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_client_streaming_handler(&METHOD_STREAM_SERVICE_OPEN_STREAM, move |ctx, req, resp| {
        instance.open_stream(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_STREAM_SERVICE_HEARTBEAT, move |ctx, req, resp| {
        instance.heartbeat(ctx, req, resp)
    });
    builder.build()
}

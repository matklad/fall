// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

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


// interface

pub trait Editor {
    fn event(&self, o: ::grpc::RequestOptions, p: super::editor::InputEvent) -> ::grpc::SingleResponse<super::editor::EventReply>;

    fn updates(&self, o: ::grpc::RequestOptions, p: super::editor::ViewStateRequest) -> ::grpc::StreamingResponse<super::editor::ViewStateReply>;
}

// client

pub struct EditorClient {
    grpc_client: ::grpc::Client,
    method_event: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::editor::InputEvent, super::editor::EventReply>>,
    method_updates: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::editor::ViewStateRequest, super::editor::ViewStateReply>>,
}

impl EditorClient {
    pub fn with_client(grpc_client: ::grpc::Client) -> Self {
        EditorClient {
            grpc_client: grpc_client,
            method_event: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/editor.Editor/event".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_updates: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/editor.Editor/updates".to_string(),
                streaming: ::grpc::method::GrpcStreaming::ServerStreaming,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new(host: &str, port: u16, tls: bool, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new(host, port, tls, conf).map(|c| {
            EditorClient::with_client(c)
        })
    }
}

impl Editor for EditorClient {
    fn event(&self, o: ::grpc::RequestOptions, p: super::editor::InputEvent) -> ::grpc::SingleResponse<super::editor::EventReply> {
        self.grpc_client.call_unary(o, p, self.method_event.clone())
    }

    fn updates(&self, o: ::grpc::RequestOptions, p: super::editor::ViewStateRequest) -> ::grpc::StreamingResponse<super::editor::ViewStateReply> {
        self.grpc_client.call_server_streaming(o, p, self.method_updates.clone())
    }
}

// server

pub struct EditorServer {
    pub grpc_server: ::grpc::Server,
}

impl ::std::ops::Deref for EditorServer {
    type Target = ::grpc::Server;

    fn deref(&self) -> &Self::Target {
        &self.grpc_server
    }
}

impl EditorServer {
    pub fn new<A : ::std::net::ToSocketAddrs, H : Editor + 'static + Sync + Send + 'static>(addr: A, conf: ::grpc::ServerConf, h: H) -> Self {
        let service_definition = EditorServer::new_service_def(h);
        EditorServer {
            grpc_server: ::grpc::Server::new_plain(addr, conf, service_definition),
        }
    }

    pub fn new_pool<A : ::std::net::ToSocketAddrs, H : Editor + 'static + Sync + Send + 'static>(addr: A, conf: ::grpc::ServerConf, h: H, cpu_pool: ::futures_cpupool::CpuPool) -> Self {
        let service_definition = EditorServer::new_service_def(h);
        EditorServer {
            grpc_server: ::grpc::Server::new_plain_pool(addr, conf, service_definition, cpu_pool),
        }
    }

    pub fn new_service_def<H : Editor + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::server::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::server::ServerServiceDefinition::new(
            vec![
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/editor.Editor/event".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.event(o, p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/editor.Editor/updates".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::ServerStreaming,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerServerStreaming::new(move |o, p| handler_copy.updates(o, p))
                    },
                ),
            ],
        )
    }
}

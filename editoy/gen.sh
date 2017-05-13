#!/bin/sh
protoc --rust_out=rs/proto proto/*.proto
protoc --rust-grpc_out=rs/proto proto/*.proto

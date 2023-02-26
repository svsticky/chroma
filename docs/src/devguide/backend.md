# Backend 

## Rust

Chroma's backend is written in [Rust](https://rust-lang.org).
While the language is not as mainstream yet as say, JavaScript, it is
still approachable to most developers.

To get started you should probably install the [Rust compiler](https://www.rust-lang.org/tools/install).

## Protocol buffers

The 'protoocol specification' is written using [Protocol buffers](https://protobuf.dev/).
I've chosen protobuf to allow easy, language-asgnostic sharing of the datatypes used by the API.
It's also mostly format independent. Chroma supports `application/protobuf`, `application/json` and `text/xml`. Both for requests, as well as 
for responses. This can be specified with the `Content-Type` and `Accept` headers.

Note: The parsing of those headers isn't completely up to spec. E.g. character sets will 
be ignored. By default `JSON` is used.

You will need `protoc` to compile the protobuf files to Rust code. It can be found [here](https://github.com/protocolbuffers/protobuf/releases/latest). It must be available on your path to compile the API.
The compilation of the protobuf files is done automatically.

## Compiling and running

After installing both tools, the API can then be build with `cargo build`, or run directly with `cargo run`.
To control log output, use the `RUST_LOG` environmental variable. More information [here](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/struct.EnvFilter.html#directives)

## Codebase structure

The server is split up in three mayor components:
- `dal/`
- `proto/`
- `chroma/`

All three are seperate Rust crates.

### dal

This is the so-called "**D**ata **A**ccess **L**ayer". This layer
is responsible for 'translating' Rust objects and function calls to SQL queries and statements.

It is also responsible for abstracting over the S3 SDK.

While it might seem out of place, `dal` also has implementations to convert `dal` types to `proto` types.
E.g. to convert `dal::Photo` to `proto::Photo`.

### proto

The `proto` crate holds all protobuf files, in the `protos/` subdirectory.
It contains only 1 line of code:
```rust,no_run,noplayground
{{#include ../../../server/proto/src/lib.rs}}
```
This line includes the auto generated Rust code, generated from the protobuf files, at compile time.
You might've noticed there's also a file `build.rs`. This is a program compiled and executed at compile time.
It is responsible for actually compiling the protobuf files to Rust code. This file is automatically executed by Cargo.

### chroma

The `chroma/` directory is the primary crate of the backend. It is also the entrypont of the application (In `main.rs`).
It contains all HTTP routes and handles them.

## Style

Chroma uses `rustfmt` to format its code. Rustfmt is standard formatter for Rust.
All code should be formatted using `rustfmt -A` before commiting.
It is installed by the rust installer by default.

## Linting

Chroma is linted using `clippy`. Clippy is the standard linter for Rust.
You should preferably lint your code with clippy before commiting.
While it is not a requirment yet, it might become so in the future.

> Note: As Chroma is still under active development, the output will contain *a lot* of warnings or error

## Testing

No unit tests are written as of yet (February 2023), but the goal is to test most of the backend,
using a mix of unit and integration tests, as well as benchmarks.
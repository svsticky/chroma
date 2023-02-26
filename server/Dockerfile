FROM rust:1-slim AS BUILDER
RUN apt update -qq && apt install -y -qq --no-install-recommends \
    gcc \
    musl-tools \
    cmake \
    clang \
    make \
    protobuf-compiler

RUN rustup set profile minimal
RUN rustup default nightly
RUN rustup target add x86_64-unknown-linux-musl

COPY ./chroma /app/chroma/chroma
COPY ./dal /app/chroma/dal
COPY ./proto /app/chroma/proto
COPY ./Cargo.toml /app/chroma/
COPY ./Cargo.lock /app/chroma/

WORKDIR /app/chroma

RUN CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse cargo build --release --target x86_64-unknown-linux-musl --bin chroma

FROM alpine
RUN apk add --no-cache ca-certificates
COPY --from=BUILDER /app/chroma/target/x86_64-unknown-linux-musl/release/chroma /usr/local/bin/chroma

RUN chmod a+rx /usr/local/bin/*
RUN adduser chroma -s /bin/false -D -H
USER chroma

EXPOSE 8000
WORKDIR /usr/local/bin
ENTRYPOINT [ "/usr/local/bin/chroma" ]
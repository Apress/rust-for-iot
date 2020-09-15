#FROM ekidd/rust-musl-builder:1.34.2 as build
#FROM clux/muslrust:1.34.2-stable as build
FROM clux/muslrust:stable as build

# <1>
RUN apt-get update && apt-get install -y capnproto

COPY ./ ./

RUN cargo build --release

FROM scratch

COPY --from=build /volume/target/x86_64-unknown-linux-musl/release/retrieval_svc .

EXPOSE 3000

CMD ["/retrieval_svc"]
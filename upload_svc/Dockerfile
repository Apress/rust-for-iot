#FROM rust:1.34.0 as build
FROM ekidd/rust-musl-builder:1.34.2 as build

COPY ./ ./

# Fix permissions on source code.
RUN sudo chown -R rust:rust /home/rust

#RUN cargo build --target x86_64-unknown-linux-musl --release
RUN cargo build --release

FROM scratch

COPY --from=build /home/rust/src/target/x86_64-unknown-linux-musl/release/upload_svc .

EXPOSE 3001

CMD ["/upload_svc"]
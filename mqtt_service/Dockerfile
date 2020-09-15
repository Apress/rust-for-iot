## MUSL with OpenSSL
# <1>
FROM ekidd/rust-musl-builder:1.34.2-openssl11 as build

# Fix permissions on source code.
# Also on where we are installing CAPNP
RUN sudo chown -R rust:rust /home/rust

# need to instapp capnp
# Install capnproto 0.6.1
# <2>
RUN echo "Intalling Cap'n Proto" && \
  cd /tmp && \
  CAPN_PROTO_VERSION=0.7.0 && \
  curl -O "https://capnproto.org/capnproto-c++-$CAPN_PROTO_VERSION.tar.gz" && \
  tar zxf "capnproto-c++-$CAPN_PROTO_VERSION.tar.gz" && \
  cd "capnproto-c++-$CAPN_PROTO_VERSION" && \
  ./configure && \
  make -j6 check && \
  sudo make install && \
  rm -r /tmp/*

# add the source code for the build
COPY ./ ./

RUN cargo build --release

FROM scratch

COPY --from=build /home/rust/src/target/x86_64-unknown-linux-musl/release/mqtt_service .

EXPOSE 8883

CMD ["/mqtt_service"]
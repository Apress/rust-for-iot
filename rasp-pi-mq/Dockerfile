FROM nusairat/cross:arm7-gnu-ubuntu-18

ENV CAPN_PROTO_VERSION 0.7.0
ENV FILE_NAME capnproto-c++-$CAPN_PROTO_VERSION

RUN curl -O https://capnproto.org/capnproto-c++-$CAPN_PROTO_VERSION.tar.gz && \
  tar zxf capnproto-c++-$CAPN_PROTO_VERSION.tar.gz

WORKDIR capnproto-c++-0.7.0
RUN  ./configure

RUN make -j6 check

RUN make install

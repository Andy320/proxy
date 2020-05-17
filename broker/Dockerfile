ARG BASE_IMAGE=registry.cn-beijing.aliyuncs.com/andy320/dev-ubuntu18.04:1.0.0

FROM ${BASE_IMAGE} AS builder

ADD . /root/rust/src

RUN cd /root/rust/src && cargo build --release

FROM alpine:latest
RUN apk --no-cache add ca-certificates

COPY --from=builder \
    /root/rust/src/target/x86_64-unknown-linux-musl/release/broker \
    /usr/local/bin/

EXPOSE 9000

ENTRYPOINT ["broker"]
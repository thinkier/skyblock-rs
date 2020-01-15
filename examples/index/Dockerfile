FROM rust:latest

RUN apt-get update;\
    apt-get install libssl-dev;\
    mkdir /tmp/build

COPY ../../ /tmp/build/

WORKDIR /tmp/build/
RUN cargo build;\
    mv /tmp/build/index /root/executable

WORKDIR /root/

CMD ["./executable"]

FROM rust:latest

COPY . /tmp/build/

WORKDIR /tmp/build/

RUN cargo build --release;

RUN mv /tmp/build/target/release/index /root/executable
RUN rm -rf /tmp/build

WORKDIR /root/

CMD ["./executable"]

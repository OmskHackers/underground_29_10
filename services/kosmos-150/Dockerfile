FROM rust:1.73-buster

WORKDIR /app

COPY . .

RUN cargo build --release

CMD [ "./target/release/kosmos150" ]
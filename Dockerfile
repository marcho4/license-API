FROM rust:latest AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM ubuntu:latest

WORKDIR /app

COPY --from=builder /app/target/release/license_microservice /app/license_microservice

EXPOSE 8001

CMD ["./license_microservice"]
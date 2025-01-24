FROM rust:1.83 AS build

WORKDIR /app

RUN apt update 
RUN apt install -y build-essential pkg-config libssl-dev cmake 

COPY ./registry/backend/Cargo.toml ./registry/backend/Cargo.toml
COPY ./registry/backend/Cargo.lock ./registry/backend/Cargo.lock
COPY ./registry/backend/src ./registry/backend/src
COPY ./codegen ./codegen

WORKDIR /app/registry/backend

RUN cargo build --release

FROM rust:1.83-slim
COPY --from=build /app/registry/backend/target/release/telchar-backend /app/bin/telchar-backend
COPY ./registry/data /app/data

ENV ROCKET_ADDRESS=0.0.0.0

WORKDIR /app/bin

CMD ["./telchar-backend"]
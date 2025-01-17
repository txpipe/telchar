FROM rust:1.83 AS build

WORKDIR /app

RUN apt update 
RUN apt install -y build-essential pkg-config libssl-dev cmake 

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src ./src

RUN cargo build --release

FROM rust:1.83-slim
COPY --from=build /app/target/release/telchar-backend .

ENV ROCKET_ADDRESS=0.0.0.0

CMD ["./telchar-backend"]
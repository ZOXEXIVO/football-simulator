FROM rust:1.58.1 as build
WORKDIR /src

COPY ./ ./

RUN cargo test -p core

RUN cargo build --release

FROM rust:1.58.1-slim
WORKDIR /app

COPY --from=build /src/target/release/football_simulator .

ENTRYPOINT ["./football_simulator"]
FROM rust:1.59 as build
WORKDIR /src

COPY ./ ./

RUN cargo test -p core

RUN cargo build --release

FROM rust:1.59-slim
WORKDIR /app

COPY --from=build /src/target/release/football_simulator .

ENTRYPOINT ["./football_simulator"]
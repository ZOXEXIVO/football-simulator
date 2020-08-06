FROM rust:1.45 as build
WORKDIR /src

COPY ./ ./

RUN cargo test

RUN cargo build --release

FROM rust:1.45-slim
WORKDIR /app
COPY --from=build /src/target/release .

ENTRYPOINT ["./football"]
FROM rust:1.43 as build
WORKDIR /src

COPY ./ ./

RUN cargo test

RUN cargo build --release

FROM rust:1.43-slim
WORKDIR /app
COPY --from=build /src/target/release .

ENTRYPOINT ["./football"]
# BUILD FRONTEND
FROM node:19-alpine3.16 AS build-frontend

WORKDIR /app

COPY ./ui/package.json .

RUN npm install --legacy-peer-deps

COPY ./ui/ .

RUN npm run publish 

FROM rust:1.71 as build-backend
WORKDIR /src

COPY ./ ./

RUN cargo test -p core

RUN cargo build --release

FROM rust:1.71-slim
WORKDIR /app

COPY --from=build-backend /src/target/release/football_simulator .
COPY --from=build-frontend /app/dist ./dist

ENTRYPOINT ["./football_simulator"]
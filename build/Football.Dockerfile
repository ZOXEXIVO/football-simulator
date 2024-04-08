# BUILD FRONTEND
FROM node:19-alpine3.18 AS build-frontend

WORKDIR /app

COPY ./ui/package.json .

RUN npm install --legacy-peer-deps

COPY ./ui/ .

RUN npm run publish 

# BUILD BACKEND

FROM rust:1.77 as build-backend
WORKDIR /src

COPY ./ ./

RUN cargo test -p core

RUN cargo build --release

FROM rust:1.77-slim
WORKDIR /app

COPY --from=build-backend /src/target/release/football_simulator .
COPY --from=build-frontend /app/dist ./dist

ENTRYPOINT ["./football_simulator"]
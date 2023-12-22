# Builder: Downloads dependencies, compiles the project,
# passes on the executable
FROM node:latest as frontend-build

WORKDIR /app
COPY frontend .

RUN npm install
RUN npm run build

FROM rust:alpine as backend-build

WORKDIR /app
COPY backend .

RUN apk add --no-cache musl-dev
RUN cargo build --release

# User: Get the executable and run it
FROM alpine:latest

WORKDIR /app
COPY config.yml defaults/config.yml
COPY --chmod=711 entrypoint.sh .
COPY --from=backend-build --chmod=711 /app/target/release/wol .
COPY --from=frontend-build --chmod=711 /app/build ./frontend

ENV RUST_LOG="wol"
ENV CONFIG_FILE="/app/config.yml"
ENV FRONTEND_PATH="/app/frontend"
ENV ADDRESS="0.0.0.0:3000"

ENTRYPOINT ["./entrypoint.sh"]
CMD ["./wol"]
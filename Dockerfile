# Build stage — compiles the Yew app to WASM with Trunk
FROM rust:1-bookworm AS builder
WORKDIR /app

RUN rustup target add wasm32-unknown-unknown \
    && cargo install --locked trunk

COPY Cargo.toml Cargo.lock ./
COPY index.html index.scss favicon.ico ./
COPY src ./src

RUN trunk build --release

# Runtime stage — serves the static bundle behind nginx
FROM nginx:1.27-alpine

COPY --from=builder /app/dist /usr/share/nginx/html
COPY nginx/nginx.conf /etc/nginx/conf.d/default.conf
COPY nginx/docker-entrypoint.sh /docker-entrypoint.d/40-greeniem-env.sh
RUN chmod +x /docker-entrypoint.d/40-greeniem-env.sh

EXPOSE 80

FROM rust:1.88.0-slim-bookworm AS base

FROM base AS builder
WORKDIR /app  
COPY ./auth_service ./auth_service
COPY ./shared ./shared
COPY ./.sqlx ./auth_service/.sqlx

WORKDIR /app/auth_service  

ENV SQLX_OFFLINE=true
RUN cargo build --release --no-default-features --bin auth_service --features postgres 
RUN strip ./target/release/auth_service -o /auth_service

FROM debian:bookworm-slim
COPY --from=builder /auth_service /
COPY ./auth_service/migrations ./auth_service/migrations

RUN addgroup --system appgroup && adduser --system --ingroup appgroup appuser
RUN chown appuser:appgroup /auth_service -R

EXPOSE $APP_PORT

USER appuser

CMD [ "/auth_service" ]
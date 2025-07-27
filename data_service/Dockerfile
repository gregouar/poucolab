FROM rust:1.88.0-slim-bookworm AS base

FROM base AS builder
WORKDIR /app  
COPY ./data_service ./data_service
COPY ./shared ./shared
COPY ./.sqlx ./data_service/.sqlx

WORKDIR /app/data_service  

ENV SQLX_OFFLINE=true
RUN cargo build --release --no-default-features --bin data_service --features postgres 
RUN strip ./target/release/data_service -o /data_service

FROM debian:bookworm-slim
COPY --from=builder /data_service /
COPY ./data_service/migrations ./data_service/migrations

RUN addgroup --system appgroup && adduser --system --ingroup appgroup appuser
RUN chown appuser:appgroup /data_service -R

EXPOSE 4201

USER appuser

CMD [ "/data_service" ]
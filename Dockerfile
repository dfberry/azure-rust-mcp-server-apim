FROM rust AS builder

RUN apt update && apt install -y libpq-dev

WORKDIR /app

COPY . .
RUN ls -la

RUN cargo build --release
#---------------------------------------------
FROM rust AS server

RUN apt update && apt install -y libpq-dev

WORKDIR /app

# Copy the built application from the first stage
COPY --from=builder /app/target/release/server /app/server
COPY --from=builder /app/Cargo.toml /app/Cargo.toml

RUN ls -la

ENV PORT=3000
EXPOSE 3000

CMD ["/app/server"]
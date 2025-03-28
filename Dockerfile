# Etapa de compilación
FROM rust:1.65 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Etapa final: imagen ligera
FROM debian:buster-slim
WORKDIR /app
# Copia el binario compilado; asegúrate que el nombre coincida con el definido en Cargo.toml
COPY --from=builder /app/target/release/la_liga_backend .
# Copia el archivo .env
COPY .env .
EXPOSE 8080
CMD ["./la_liga_backend"]

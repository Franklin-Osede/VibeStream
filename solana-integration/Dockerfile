# Build stage
FROM rust:1.82-slim as builder

WORKDIR /usr/src/app

# Instalar dependencias necesarias
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# Copiar solo el Cargo.toml primero
COPY Cargo.toml .

# Crear un src vacío para cachear las dependencias
RUN mkdir src && \
    echo "fn main() {println!(\"Hello\");}" > src/main.rs && \
    echo "pub fn add(a: i32, b: i32) -> i32 { a + b }" > src/lib.rs

# Compilar las dependencias
RUN cargo build --release

# Eliminar los archivos temporales
RUN rm -rf src

# Copiar el código real
COPY src ./src

# Compilar el proyecto real y mostrar los archivos generados
RUN cargo build --release && \
    echo "Contenido del directorio target/release:" && \
    ls -la /usr/src/app/target/release/

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copiar el binario compilado (usaremos el nombre correcto después de ver la salida)
COPY --from=builder /usr/src/app/target/release/solana-integration ./solana-integration

ENV RUST_LOG=debug

EXPOSE 3001

CMD ["./solana-integration"] 
# Étape de construction
FROM rust:1.75 as builder

# Installer les dépendances système
RUN apt-get update && \
    apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copier les fichiers de dépendances
COPY Cargo.toml Cargo.lock ./
COPY backend ./backend

# Construire les dépendances (optimisation du cache)
RUN mkdir -p backend/src && \
    echo "fn main() {}" > backend/src/main.rs && \
    cargo build --release --manifest-path=backend/Cargo.toml && \
    rm -rf backend/src

# Construire l'application réelle
COPY . .
RUN cargo build --release --manifest-path=backend/Cargo.toml

# Étape d'exécution
FROM debian:bullseye-slim

# Dépendances d'exécution
RUN apt-get update && \
    apt-get install -y \
    libssl1.1 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copier le binaire construit
COPY --from=builder /app/target/release/backend /app/backend

# Copier le frontend (si nécessaire)
COPY frontend ./frontend

# Variables d'environnement
ENV RUST_LOG=info
ENV PORT=8000

EXPOSE ${PORT}

CMD ["/app/backend"]
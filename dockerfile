# Étape 1 : Utiliser une image Rust pour la construction
FROM rust:latest AS builder

# Installer les dépendances nécessaires à la construction
RUN apt-get update && apt-get install -y --no-install-recommends \
    apt-transport-https \
    ca-certificates \
    build-essential \
    libgtk-3-dev \
    pkg-config \
    libjavascriptcoregtk-4.0-dev \
    javascriptcoregtk-4.1\
    libsoup2.4-dev \
    libsoup-3.0-dev \
    webkit2gtk-4.1-dev\
    && rm -rf /var/lib/apt/lists/*

# Configurer le chemin pour pkg-config
ENV PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig

# Définir le répertoire de travail
WORKDIR /usr/src/app

# Copier les fichiers Cargo.toml et Cargo.lock pour le cache des dépendances
COPY Cargo.toml Cargo.lock ./

# Télécharger les dépendances
RUN cargo fetch

# Copier les fichiers restants
COPY . .

# Construire en mode release
RUN cargo build --release

# Étape 2 : Utiliser une image légère pour l'exécution
FROM debian:bullseye-slim

# Installer les dépendances nécessaires à l'exécution
RUN apt-get update && apt-get install -y --no-install-recommends \
    libgtk-3-0 \
    libjavascriptcoregtk-4.0-18 \
    libsoup2.4-1 \
    && rm -rf /var/lib/apt/lists/*

# Copier l'exécutable depuis l'étape de construction
COPY --from=builder /usr/src/app/target/release/rust_framework /usr/local/bin/rust_framework

# Définir le point d'entrée
ENTRYPOINT ["rust_framework"]

# Exposer le port utilisé par l'application
EXPOSE 8080

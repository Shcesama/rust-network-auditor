# 1. Aşama: Derleme (Builder)
FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo build --release

# 2. Aşama: Çalıştırma (Runner)
FROM debian:bookworm-slim
WORKDIR /app

# İşletim sistemi için temel sertifikaları yükle
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Derlenen çalıştırılabilir dosyayı builder aşamasından kopyala
COPY --from=builder /app/target/release/rust-service-port-auditor /usr/local/bin/

# Konteyner başlatıldığında çalışacak ana komut
ENTRYPOINT ["rust-service-port-auditor"]
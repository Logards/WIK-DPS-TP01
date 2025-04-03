FROM rust:slim
WORKDIR /app
COPY src/ ./src/
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release
RUN groupadd -g 1000 rustgroup && \
    useradd -u 1000 -g rustgroup -s /bin/bash -m rustuser && \
    chown -R rustuser:rustgroup /app
USER rustuser
ENV PING_LISTEN_PORT="8080"
CMD ["./target/release/WIK-DPS-TP01"]
FROM rust:1.75-slim

RUN apt-get update && apt-get install -y \
    libpcap-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .

CMD ["cargo", "run"]
FROM rust:latest as builder
WORKDIR /usr/src/moose23967-ru
COPY . .
RUN cargo install --path .

FROM balenalib/armv7hf-debian:latest
RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/moose23967-ru /usr/local/bin/moose23967-ru
CMD ["moose23967-ru"]

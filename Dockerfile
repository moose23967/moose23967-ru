FROM rust:latest as builder
WORKDIR /usr/src/moose23967-ru
COPY . .
RUN cargo install --path .

FROM balenalib/armv7hf-debian:latest
COPY --from=builder /usr/local/cargo/bin/moose23967-ru /usr/local/bin/moose23967-ru

EXPOSE 80
CMD ["moose23967-ru"]

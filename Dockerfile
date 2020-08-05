FROM rust:alpine as builder
RUN apk update && \
    apk add build-base && \
    rm -rf /var/cache/apk/*

WORKDIR /usr/src/rustscan
COPY . .
RUN cargo install --path .

FROM alpine
LABEL author="Hydragyrum <https://github.com/Hydragyrum>"
RUN ulimit -n 100000 && \
    apk update && \
    apk add \
    nmap nmap-scripts wget \
    && \
    rm -rf /var/cache/apk/*
COPY --from=builder /usr/local/cargo/bin/rustscan /usr/local/bin/rustscan
RUN addgroup -S rustscan && \
    adduser -S -G rustscan rustscan
USER rustscan
ENTRYPOINT ["/usr/local/bin/rustscan"]

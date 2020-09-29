FROM rust:alpine as builder
LABEL maintainer="RustScan <https://github.com/RustScan>"
RUN apk add --no-cache build-base

# Encourage some layer caching here rather then copying entire directory that includes docs to builder container ~CMN
WORKDIR /usr/src/rustscan
COPY Cargo.toml Cargo.lock ./
COPY src/ src/
RUN cargo install --path .

FROM alpine:3.12
LABEL author="Hydragyrum <https://github.com/Hydragyrum>"
RUN addgroup -S rustscan && \
    adduser -S -G rustscan rustscan && \
    ulimit -n 100000 && \
    apk add --no-cache nmap nmap-scripts wget
COPY --from=builder /usr/local/cargo/bin/rustscan /usr/local/bin/rustscan
USER rustscan
ENTRYPOINT [ "/usr/local/bin/rustscan" ]

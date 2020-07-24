FROM rust:latest

RUN apt update -y && apt install git -y
RUN git clone https://github.com/brandonskerritt/RustScan
WORKDIR "/RustScan"
RUN git pull --force
RUN cargo install cargo-deb

COPY ./entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]

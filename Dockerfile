FROM ubuntu:18.04

# Install build tools
RUN apt-get update && apt-get install -y gcc make git curl binutils libc6-dev

# Install Rust
RUN curl https://sh.rustup.rs -o rustup-init.sh && chmod +x rustup-init.sh && ./rustup-init.sh -y 

WORKDIR /9cc.rust

COPY . /9cc.rust

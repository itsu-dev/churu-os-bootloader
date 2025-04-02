FROM rustlang/rust:nightly

RUN apt-get update && \
    apt-get install -y \
    qemu-utils \
    nasm \
    dosfstools \
    binutils \
    mtools \
    parted \ 
    udev

RUN rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
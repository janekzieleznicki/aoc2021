FROM clearlinux:base

RUN swupd bundle-add curl sudo os-core-search c-basic
RUN groupadd --gid 1000 node \
    && useradd --uid 1000 --gid node --shell /bin/bash --create-home node

USER 1000:1000
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly-x86_64-unknown-linux-gnu

ENTRYPOINT ["/home/node/.cargo/bin/cargo"]
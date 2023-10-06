FROM rustlang/rust:nightly-slim
LABEL maintainer="Deniz Hofmeister"
LABEL description="Robotics Deployment Command-Line Interface"

RUN apt-get update && apt-get install -y --no-install-recommends \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get autoremove -y \
    && apt-get clean

COPY rdcli /rdcli
WORKDIR /rdcli

RUN cargo build --release && \
    cp target/release/rdcli /usr/local/bin/

ENV CARGO_UNSTABLE_SPARSE_REGISTRY=true
ENV CARGO_UNSTABLE_REGISTRY_AUTH=true

RUN rdcli --version

CMD ["rdcli", "--help"]
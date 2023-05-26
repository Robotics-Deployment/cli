FROM ros:humble-ros-core-jammy
LABEL authors="Deniz Hofmeister"

# Install dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    curl \
    python3-colcon-common-extensions \
    git \
    build-essential \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

COPY rdcli /project/src/rdcli

WORKDIR /project

CMD ["/bin/bash"]

FROM ros:humble-ros-core-jammy
LABEL authors="Deniz Hofmeister"

# Install dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    nano \
    git \
    wget \
    python3-dev \
    python3-pip \
    python3-setuptools \
    python3-wheel \
    python3-colcon-common-extensions \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# Update python
RUN python3 -m pip install --upgrade --no-cache-dir pip setuptools wheel && \
    python3 -m pip install --no-cache-dir hatch

COPY rdcli /src/rdcli

# Install toml CLI
RUN cd /src/rdcli && pip3 install -e .

CMD ["/bin/bash"]

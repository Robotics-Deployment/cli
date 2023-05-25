FROM ros:humble-ros-core-jammy
LABEL authors="Deniz Hofmeister"

# Install dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    python3-pip \
    python3-colcon-common-extensions \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# Update python
RUN python3 -m pip install --upgrade --no-cache-dir pip && \
    python3 -m pip install --no-cache-dir hatch

COPY rdcli /src/rdcli

# Install toml CLI
RUN cd /src/rdcli && python3 -m pip install -e .

WORKDIR /project

CMD ["/bin/bash"]

FROM ubuntu:jammy
LABEL authors="Deniz Hofmeister"

# Install dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    nano \
    git \
    python3-dev \
    python3-pip \
    python3-setuptools \
    python3-wheel \
    wget \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# Update python
RUN python3 -m pip install --upgrade --no-cache-dir pip setuptools wheel && \
    python3 -m pip install hatch

CMD ["/bin/bash"]

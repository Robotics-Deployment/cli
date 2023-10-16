ARG IMAGE=rust:latest

FROM ${IMAGE}

ARG DEBIAN_FRONTEND=noninteractive

LABEL maintainer="Deniz Hofmeister"
LABEL description="Robotics Deployment Command Line Interface"
LABEL org.opencontainers.image.source="https://github.com/Robotics-Deployment/cli"

COPY rdcli /opt/rdcli
WORKDIR /opt/rdcli

CMD ["bash"]
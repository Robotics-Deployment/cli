ARG IMAGE=rust:latest

FROM ${IMAGE}

ARG DEBIAN_FRONTEND=noninteractive

LABEL maintainer="Deniz Hofmeister"
LABEL description="Robotics Deployment Command Line Interface"

COPY rdcli /opt/rdcli
WORKDIR /opt/rdcli

CMD ["bash"]
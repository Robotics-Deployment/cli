FROM ros:humble-ros-core-jammy
LABEL authors="Deniz Hofmeister"

# Install dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    curl \
    python3-colcon-common-extensions \
    git \
    build-essential \
    gdb \
    openssh-server \
    gdbserver \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"
COPY client /client/src/client
WORKDIR /client/src/client

RUN cargo build

COPY entrypoint.sh /client/src/entrypoint.sh
RUN mkdir /var/run/sshd

RUN echo 'root:password' | chpasswd
RUN sed -i 's/#PermitRootLogin prohibit-password/PermitRootLogin yes/' /etc/ssh/sshd_config
RUN sed 's@session\s*required\s*pam_loginuid.so@session optional pam_loginuid.so@g' -i /etc/pam.d/sshd

ENTRYPOINT ["/client/src/entrypoint.sh"]

CMD ["sleep", "infinity"]

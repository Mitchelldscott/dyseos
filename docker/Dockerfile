
FROM ubuntu:20.04

ENV PROJECT_ROOT=/home/dev/dyseos
ENV DEBIAN_FRONTEND=noninteractive

#
#   Update packages
#
RUN apt update && apt upgrade -y             && \
    apt install -y --no-install-recommends      \
    build-essential                             \
    ca-certificates                             \
    pkg-config                                  \
    python3-venv                                \
    python3                                     \
    locales                                     \
    ruby-dev                                    \
    ruby                                        \
    graphviz                                    \
    curl                                        \
    wget                                        \
    git

#
#   Install LLVM
#
RUN apt install -y --no-install-recommends lsb-release wget software-properties-common gnupg

RUN wget https://apt.llvm.org/llvm.sh   && \
    chmod +x llvm.sh                    && \
    ./llvm.sh 18                        && \
    rm llvm.sh

# Set lldb version independant alias
# RUN echo 'alias lldb="lldb-18"' >> ~/.bashrc
RUN ln -s $(which lldb-18) /usr/bin/lldb

#
#   QEMU build dependencies
#
RUN apt install -y --no-install-recommends  \
    libglib2.0-dev                          \
    libfdt-dev                              \
    libpixman-1-dev                         \
    zlib1g-dev                              \
    ninja-build

#
#   Apt bad, Build QEMU for great success
#
RUN git clone https://gitlab.com/qemu-project/qemu.git       && \
    cd qemu                                                  && \
    git checkout tags/v8.2.2                                 && \
    ./configure --target-list=aarch64-softmmu --enable-modules  \
    --enable-tcg-interpreter --enable-debug-tcg                 \
    --python=/usr/bin/python3                                && \
    make -j10                                                && \
    make install                                             && \
    cd ..                                                    && \
    rm -rf qemu                                               

#
#   Install Rust
#
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -sSf | sh -s -- -y

#
#   Install Cargo-binutils & llvm-tools, Download the Teensy4.1 & RaspberryPi targets
#
ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup update                                       && \
    cargo install cargo-binutils                        && \
    cargo install cargo-modules                         && \
    rustup component add llvm-tools                     && \
    rustup target add thumbv7em-none-eabihf             && \
    rustup target add aarch64-unknown-none-softfloat

#
#   Setup locale
#
RUN locale-gen en_US.UTF-8

ENV LANG=en_US.UTF-8   \
    LANGUAGE=en_US:en  \
    LC_ALL=en_US.UTF-8

# 
#   Clean apt
#
RUN apt autoremove -q -y        && \
    apt clean -q -y             && \
    rm -rf /var/lib/apt/lists/*

#
#   Setup workspace
#
ARG TARGET
ARG PROFILE
ARG MACHINE
ARG BIN

ENV TARGET=${TARGET}
ENV PROFILE=${PROFILE}
ENV MACHINE=${MACHINE}
ENV BIN=${BIN}

COPY ./Cargo* ${PROJECT_ROOT}/
COPY ./src ${PROJECT_ROOT}/src/
COPY ./.cargo/ ${PROJECT_ROOT}/.cargo/
COPY ./docker/entrypoint.bash ${PROJECT_ROOT}/../entrypoint.bash

WORKDIR ${PROJECT_ROOT}
ENTRYPOINT ${PROJECT_ROOT}/../entrypoint.bash $0 $@
CMD -d bash
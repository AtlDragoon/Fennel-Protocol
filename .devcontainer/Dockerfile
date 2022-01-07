FROM mcr.microsoft.com/vscode/devcontainers/rust:0-1

# [Optional] Uncomment this section to install additional packages.
# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
#     && apt-get -y install --no-install-recommends <your-package-list-here>

RUN DEBIAN_FRONTEND=noninteractive \
    apt-get update -y && \
#    ln -fs /usr/share/zoneinfo/America/New_York /etc/localtime && \
#    apt-get install -y tzdata && \
#    dpkg-reconfigure --frontend noninteractive tzdata && \
    apt-get install unzip curl build-essential protobuf-compiler -y && \
    apt-get install clang libclang-dev libclang1 llvm llvm-dev clang-tools cmake -y && \
    apt-get upgrade -y

#RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y

RUN rustup default nightly && \
	rustup target add wasm32-unknown-unknown --toolchain nightly

EXPOSE 9944

RUN git config --global submodule.recurse true

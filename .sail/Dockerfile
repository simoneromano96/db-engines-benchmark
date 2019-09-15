FROM codercom/ubuntu-dev:latest

# Access other docker containers or start new ones with automatic port forwarding
# Share the host's docker socket with the Sail project so that you can
# access it using the docker client.
LABEL share.docker_sock "/var/run/docker.sock:/var/run/docker.sock"

# Follow the instructions for installing docker on ubuntu here:
# https://docs.docker.com/install/linux/docker-ce/ubuntu/
RUN sudo apt-get update && \
   sudo apt-get upgrade -y && \
   sudo apt-get install -y apt-utils

RUN sudo apt-get install -y \
   apt-transport-https \
   ca-certificates \
   curl \
   gnupg-agent \
   software-properties-common

RUN curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo apt-key add -

RUN sudo apt-key fingerprint 0EBFCD88

RUN sudo add-apt-repository \
   "deb [arch=amd64] https://download.docker.com/linux/ubuntu \
   $(lsb_release -cs) \
   stable"

# Only install the client since we're using the docker daemon system running on the host.
RUN sudo apt-get install -y docker-ce-cli

# Install docker-compose
RUN sudo curl -L "https://github.com/docker/compose/releases/download/1.24.1/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose

# Make docker-compose executable
RUN sudo chmod +x /usr/local/bin/docker-compose


# Install miniconda
ENV LANG=C.UTF-8 LC_ALL=C.UTF-8
ENV PATH /opt/conda/bin:$PATH

RUN sudo apt-get update --fix-missing && \
    sudo apt-get install -y wget bzip2 ca-certificates curl git && \
    sudo apt-get clean && \
    sudo rm -rf /var/lib/apt/lists/*

RUN wget --quiet https://repo.anaconda.com/miniconda/Miniconda3-4.5.11-Linux-x86_64.sh -O ~/miniconda.sh && \
    sudo /bin/bash ~/miniconda.sh -b -p /opt/conda && \
    rm ~/miniconda.sh && \
    sudo /opt/conda/bin/conda clean -tipsy && \
    sudo ln -s /opt/conda/etc/profile.d/conda.sh /etc/profile.d/conda.sh && \
    echo ". /opt/conda/etc/profile.d/conda.sh" >> ~/.bashrc && \
    echo "conda activate base" >> ~/.bashrc

# Rust setup
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

ENV PATH=/home/user/.cargo/bin:$PATH

RUN rustup component add rls 

# VSCode extensions
RUN installext rust-lang.rust
RUN installext serayuzgur.crates
RUN installext bungcip.better-toml


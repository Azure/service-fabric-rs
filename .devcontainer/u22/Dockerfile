FROM mcr.microsoft.com/mirror/docker/library/ubuntu:22.04 as base

# Base tools for SF + dev (combined into one RUN so the
# /var/lib/apt/lists/* cleanup actually shrinks the layer; same
# reason --no-install-recommends is set — strips ~100 MB of
# transitively-pulled docs/recommendations we never use).
RUN apt-get update -y \
    && apt-get upgrade -y --no-install-recommends \
    && apt-get install -y --no-install-recommends \
        apt-transport-https curl lsb-release wget gnupg2 \
        software-properties-common debconf-utils \
        git socat sudo python3-pip protobuf-compiler \
        net-tools locales \
        build-essential pkg-config \
    && locale-gen en_US.UTF-8 \
    && update-locale LANG=en_US.UTF-8 \
    && rm -rf /var/lib/apt/lists/*

# install cmake + sfctl via pip (kept in one RUN so the pip cache
# isn't preserved across layers).
RUN python3 -m pip install --upgrade pip \
    && pip install --upgrade cmake \
    && pip3 install -I sfctl==11.1.0
ENV PATH="${PATH}:~/.local/bin"

ENV LANG en_US.UTF-8
ENV LANGUAGE en_US:en
ENV LC_ALL en_US.UTF-8

# install sf
RUN wget -q https://packages.microsoft.com/config/ubuntu/$(lsb_release -rs)/packages-microsoft-prod.deb \
    && dpkg -i packages-microsoft-prod.deb \
    && rm packages-microsoft-prod.deb \
    && echo "servicefabric servicefabric/accepted-eula-ga select true" | debconf-set-selections \
    && echo "servicefabricsdkcommon servicefabricsdkcommon/accepted-eula-ga select true" | debconf-set-selections

ENV SF_VERSION=11.5.111.4
RUN wget -q https://download.microsoft.com/download/3/1/F/31F3FEEB-F073-4E27-A98B-8E691FF74F40/ServiceFabric.U22.${SF_VERSION}.deb \
    && apt-get update \
    && apt-get install -y --no-install-recommends ./ServiceFabric.U22.${SF_VERSION}.deb \
    && rm ServiceFabric.U22.${SF_VERSION}.deb \
    && rm -rf /var/lib/apt/lists/*

# expose sf shared libs
ENV LD_LIBRARY_PATH "$LD_LIBRARY_PATH:/opt/microsoft/servicefabric/bin/Fabric/Fabric.Code:"

FROM base AS onebox

COPY ./onebox/ClusterDeployer.sh /opt/microsoft/servicefabric/ClusterDeployer/ClusterDeployer.sh
COPY ./onebox/ClusterManifest.SingleMachineFSS.xml /opt/microsoft/servicefabric/ClusterDeployer/ClusterManifest.SingleMachineFSS.xml
RUN chmod +x /opt/microsoft/servicefabric/ClusterDeployer/ClusterDeployer.sh

WORKDIR /opt/microsoft/servicefabric/ClusterDeployer
ENTRYPOINT ["/opt/microsoft/servicefabric/ClusterDeployer/ClusterDeployer.sh"]

FROM base as repo

# Arguments for user creation
ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=1000
ARG RUST_VERSION=stable

# Install docker CLI so the container can talk to the host docker
# daemon via a mounted /var/run/docker.sock. Used for things like
# `docker logs onebox` from inside the devcontainer.
RUN apt-get update \
    && apt-get install -y --no-install-recommends docker.io \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN groupadd --gid ${USER_GID} ${USERNAME} \
    && useradd --uid ${USER_UID} --gid ${USER_GID} -m ${USERNAME} -s /bin/bash \
    && echo ${USERNAME} ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/${USERNAME} \
    && chmod 0440 /etc/sudoers.d/${USERNAME}

# Switch to vscode user for installing rust
USER ${USERNAME}

# install rust
WORKDIR /home/${USERNAME}
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o rustup.sh \
    && chmod u+x rustup.sh \
    && ./rustup.sh -y --default-toolchain ${RUST_VERSION} \
    && rm rustup.sh

# Forward onebox ports to host
COPY ./port_forward.sh /usr/local/bin/port_forward.sh
RUN sudo chmod +x /usr/local/bin/port_forward.sh

ENTRYPOINT ["/usr/local/bin/port_forward.sh"]
WORKDIR /home/${USERNAME}/repo
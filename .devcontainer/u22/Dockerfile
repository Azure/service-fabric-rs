FROM mcr.microsoft.com/mirror/docker/library/ubuntu:22.04 as base

RUN apt-get update -y && apt-get upgrade -y

# install tools for sf
RUN apt-get install -y apt-transport-https curl lsb-release wget gnupg2 software-properties-common debconf-utils \
    git socat sudo python3-pip

# more tools for dev
RUN pip install cmake --upgrade

#install sfctl and its dependencies.
RUN python3 -m pip install --upgrade pip
RUN pip3 install -I sfctl==11.1.0
ENV PATH="${PATH}:~/.local/bin"

RUN apt install -y net-tools locales \
 && locale-gen en_US.UTF-8 \
 && update-locale LANG=en_US.UTF-8

ENV LANG en_US.UTF-8
ENV LANGUAGE en_US:en
ENV LC_ALL en_US.UTF-8

# install sf
RUN wget -q https://packages.microsoft.com/config/ubuntu/$(lsb_release -rs)/packages-microsoft-prod.deb \
    && dpkg -i packages-microsoft-prod.deb \
    && apt-get update

RUN echo "servicefabric servicefabric/accepted-eula-ga select true" | debconf-set-selections \
  && echo "servicefabricsdkcommon servicefabricsdkcommon/accepted-eula-ga select true" | debconf-set-selections
# RUN apt-get install -y servicefabricsdkcommon

ENV SF_VERSION=11.1.208.4
RUN wget https://download.microsoft.com/download/3/1/F/31F3FEEB-F073-4E27-A98B-8E691FF74F40/ServiceFabric.U22.${SF_VERSION}.deb
RUN apt-get install -y ./ServiceFabric.U22.${SF_VERSION}.deb
RUN rm ServiceFabric.U22.${SF_VERSION}.deb

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
ARG RUST_VERSION=1.93.0

# Create a non-root user
RUN groupadd --gid ${USER_GID} ${USERNAME} \
    && useradd --uid ${USER_UID} --gid ${USER_GID} -m ${USERNAME} -s /bin/bash \
    && echo ${USERNAME} ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/${USERNAME} \
    && chmod 0440 /etc/sudoers.d/${USERNAME}

# Switch to vscode user for installing rust
USER ${USERNAME}

# install rust
WORKDIR /home/${USERNAME}
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup.sh
RUN chmod u+x rustup.sh && ./rustup.sh -y --default-toolchain ${RUST_VERSION}

# Forward onebox ports to host
COPY ./port_forward.sh /usr/local/bin/port_forward.sh
RUN sudo chmod +x /usr/local/bin/port_forward.sh

ENTRYPOINT ["/usr/local/bin/port_forward.sh"]
WORKDIR /home/${USERNAME}/repo
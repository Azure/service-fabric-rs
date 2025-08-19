FROM mcr.microsoft.com/mirror/docker/library/ubuntu:20.04 as base

RUN apt-get update -y && apt-get upgrade -y

# install tools for sf
RUN apt-get install -y apt-transport-https curl lsb-release wget gnupg2 software-properties-common debconf-utils

# install sf
RUN wget -q https://packages.microsoft.com/config/ubuntu/$(lsb_release -rs)/packages-microsoft-prod.deb \
    && dpkg -i packages-microsoft-prod.deb \
    && apt-get update

RUN echo "servicefabric servicefabric/accepted-eula-ga select true" | debconf-set-selections \
  && echo "servicefabricsdkcommon servicefabricsdkcommon/accepted-eula-ga select true" | debconf-set-selections

ENV SF_VERSION=11.1.208.3
RUN wget https://download.microsoft.com/download/3/1/F/31F3FEEB-F073-4E27-A98B-8E691FF74F40/ServiceFabric.U20.${SF_VERSION}.deb
RUN apt-get install -y ./ServiceFabric.U20.${SF_VERSION}.deb
RUN rm ServiceFabric.U20.${SF_VERSION}.deb

RUN apt install -y net-tools locales \
 && locale-gen en_US.UTF-8 \
 && update-locale LANG=en_US.UTF-8

ENV LANG en_US.UTF-8
ENV LANGUAGE en_US:en
ENV LC_ALL en_US.UTF-8

#install sfctl and its dependencies.
RUN apt-get update && apt-get install python3-pip -y
RUN python3 -m pip install --upgrade pip
RUN pip3 install -I sfctl==11.1.0
ENV PATH="${PATH}:~/.local/bin"

# expose sf shared libs
ENV LD_LIBRARY_PATH "$LD_LIBRARY_PATH:/opt/microsoft/servicefabric/bin/Fabric/Fabric.Code:"

FROM base AS onebox

COPY ./onebox/ClusterDeployer.sh /opt/microsoft/servicefabric/ClusterDeployer/ClusterDeployer.sh
COPY ./onebox/ClusterManifest.SingleMachineFSS.xml /opt/microsoft/servicefabric/ClusterDeployer/ClusterManifest.SingleMachineFSS.xml
RUN chmod +x /opt/microsoft/servicefabric/ClusterDeployer/ClusterDeployer.sh

WORKDIR /opt/microsoft/servicefabric/ClusterDeployer
ENTRYPOINT ["/opt/microsoft/servicefabric/ClusterDeployer/ClusterDeployer.sh"]

FROM base as repo
# install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > ./rustup.sh
RUN chmod u+x ./rustup.sh && ./rustup.sh -y

# more tools for dev
RUN apt-get install git socat -y
RUN pip install cmake --upgrade

# Forward onebox ports to host
COPY ./port_forward.sh /usr/local/bin/port_forward.sh
RUN chmod +x /usr/local/bin/port_forward.sh
ENTRYPOINT ["/usr/local/bin/port_forward.sh"]
WORKDIR /workspace/repo
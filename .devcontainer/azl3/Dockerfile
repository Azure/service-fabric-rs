FROM mcr.microsoft.com/azurelinux/base/core:3.0 as base

RUN tdnf install --noplugins --skipsignature -y \
    tar unzip ca-certificates gcc glibc-devel binutils \
    make python3-pip git socat cmake curl wget \
    awk sudo net-tools

RUN mkdir -p /var/opt/microsoft/servicefabric/ && echo "Accepted" | tee /var/opt/microsoft/servicefabric/accepted-eula-ga

ENV SF_VERSION=11.2.298.5
RUN wget https://download.microsoft.com/download/3/1/F/31F3FEEB-F073-4E27-A98B-8E691FF74F40/ServiceFabric.AL.${SF_VERSION}.rpm
RUN tdnf install -y ./ServiceFabric.AL.${SF_VERSION}.rpm
RUN rm ServiceFabric.AL.${SF_VERSION}.rpm

RUN pip3 install -I sfctl==11.1.0 setuptools
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

# Forward onebox ports to host
COPY ./port_forward.sh /usr/local/bin/port_forward.sh
RUN chmod +x /usr/local/bin/port_forward.sh
ENTRYPOINT ["/usr/local/bin/port_forward.sh"]
WORKDIR /workspace/repo
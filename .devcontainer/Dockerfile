FROM ubuntu:20.04

RUN apt-get update && apt-get upgrade

# install sf
RUN apt-get install apt-transport-https curl lsb-release wget gnupg2 software-properties-common debconf-utils -y

RUN wget -q https://packages.microsoft.com/config/ubuntu/$(lsb_release -rs)/packages-microsoft-prod.deb \
    && dpkg -i packages-microsoft-prod.deb \
    && curl -fsSL https://packages.microsoft.com/keys/msopentech.asc | apt-key add - \
    && curl -fsSL https://download.docker.com/linux/ubuntu/gpg | apt-key add - \
    && add-apt-repository "deb [arch=amd64] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable" \
    && apt-get update

RUN echo "servicefabric servicefabric/accepted-eula-ga select true" | debconf-set-selections \
  && echo "servicefabricsdkcommon servicefabricsdkcommon/accepted-eula-ga select true" | debconf-set-selections \
  && apt-get install servicefabricsdkcommon -y

# install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > ./rustup.sh
RUN chmod u+x ./rustup.sh && ./rustup.sh -y

# more tools
RUN apt-get install cmake git -y
RUN apt-get install clang lld -y

# expose sf shared libs
ENV LD_LIBRARY_PATH "$LD_LIBRARY_PATH:/opt/microsoft/servicefabric/bin/Fabric/Fabric.Code:/workspaces/service-fabric-rs/bintemp"
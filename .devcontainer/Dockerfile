FROM ubuntu:18.04

RUN apt-get update && apt-get upgrade

# install tools for sf
RUN apt-get install apt-transport-https curl lsb-release wget gnupg2 software-properties-common debconf-utils -y

# install sf
RUN wget -q https://packages.microsoft.com/config/ubuntu/$(lsb_release -rs)/packages-microsoft-prod.deb \
    && dpkg -i packages-microsoft-prod.deb \
    && curl -fsSL https://packages.microsoft.com/keys/msopentech.asc | apt-key add - \
    && curl -fsSL https://download.docker.com/linux/ubuntu/gpg | apt-key add - \
    && add-apt-repository "deb [arch=amd64] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable" \
    && apt-get update

RUN echo "servicefabric servicefabric/accepted-eula-ga select true" | debconf-set-selections \
  && echo "servicefabricsdkcommon servicefabricsdkcommon/accepted-eula-ga select true" | debconf-set-selections \
  && apt-get install servicefabricsdkcommon -y

#install sfctl and its dependencies.
RUN apt-get update && apt-get install python3-pip -y
RUN python3 -m pip install --upgrade pip
RUN pip3 install -I sfctl==11.1.0
ENV PATH="${PATH}:~/.local/bin"

# install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > ./rustup.sh
RUN chmod u+x ./rustup.sh && ./rustup.sh -y

# more tools for dev
RUN apt-get install git clang lld -y
RUN pip install cmake --upgrade

# expose sf shared libs
ENV LD_LIBRARY_PATH "$LD_LIBRARY_PATH:/opt/microsoft/servicefabric/bin/Fabric/Fabric.Code:"
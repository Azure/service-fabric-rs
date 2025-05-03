FROM mcr.microsoft.com/mirror/docker/library/ubuntu:20.04

RUN apt-get update && apt-get upgrade

# install tools for sf
RUN apt-get install apt-transport-https curl lsb-release wget gnupg2 software-properties-common debconf-utils -y

# install sf
RUN wget -q https://packages.microsoft.com/config/ubuntu/$(lsb_release -rs)/packages-microsoft-prod.deb \
    && dpkg -i packages-microsoft-prod.deb \
    && apt-get update

RUN echo "servicefabric servicefabric/accepted-eula-ga select true" | debconf-set-selections \
  && echo "servicefabricsdkcommon servicefabricsdkcommon/accepted-eula-ga select true" | debconf-set-selections

RUN wget https://download.microsoft.com/download/3/1/F/31F3FEEB-F073-4E27-A98B-8E691FF74F40/ServiceFabric.U20.10.1.2319.1.deb
RUN apt-get install ./ServiceFabric.U20.10.1.2319.1.deb -y
RUN rm ServiceFabric.U20.10.1.2319.1.deb

#install sfctl and its dependencies.
RUN apt-get update && apt-get install python3-pip -y
RUN python3 -m pip install --upgrade pip
RUN pip3 install -I sfctl==11.1.0
ENV PATH="${PATH}:~/.local/bin"

# install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > ./rustup.sh
RUN chmod u+x ./rustup.sh && ./rustup.sh -y

# more tools for dev
RUN apt-get install git clang -y
RUN pip install cmake --upgrade

# expose sf shared libs
ENV LD_LIBRARY_PATH "$LD_LIBRARY_PATH:/opt/microsoft/servicefabric/bin/Fabric/Fabric.Code:"
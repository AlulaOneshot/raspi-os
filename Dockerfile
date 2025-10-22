FROM ubuntu:questing

RUN apt-get update
RUN apt-get install bc bison flex libssl-dev make build-essential libc6-dev libncurses5-dev cpio unzip rsync bzip2 xz-utils crossbuild-essential-arm64 git wget file -y
WORKDIR /
RUN git clone --depth 1 https://github.com/AlulaOneshot/raspi-os.git
WORKDIR /raspi-os

CMD ["bash", "/raspi-os/setup.sh"]
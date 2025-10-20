git clone https://github.com/raspberrypi/linux --depth 1 -b rpi-6.12.y
cd linux

export KERNEL_DIR=$PWD
export ARCH=arm64
export CROSS_COMPILE=aarch64-linux-gnu-
make bcm2711_defconfig
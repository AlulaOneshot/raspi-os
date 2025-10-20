git clone https://github.com/raspberrypi/linux --depth 1 -b rpi-6.12.y

cd linux
KERNEL=kernel8
make ARCH=arm64 CROSS_COMPILE=aarch64-linux-gnu- bcm2711_defconfig
cp ../.config .config
make ARCH=arm64 CROSS_COMPILE=aarch64-linux-gnu- Image modules dtbs
set -e

wget https://buildroot.org/downloads/buildroot-2025.08.tar.xz
tar -xf buildroot-2025.08.tar.xz
cd buildroot-2025.08
cp ../shiota_devkit_defconfig configs/shiota_devkit_defconfig
make shiota_devkit_defconfig
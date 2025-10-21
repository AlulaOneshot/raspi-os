set -e

wget https://buildroot.org/downloads/buildroot-2025.08.tar.xz
tar -xf buildroot-2025.08.tar.xz
cd buildroot-2025.08
make BR2_EXTERNAL=../.config
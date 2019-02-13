#!/bin/sh

set -ex

curl http://releases.llvm.org/8.0.0/clang+llvm-8.0.0-x86_64-linux-gnu-ubuntu-14.04.tar.xz | \
  tar xJf -
export PATH=`pwd`/clang+llvm-8.0.0-x86_64-linux-gnu-ubuntu-14.04/bin:$PATH

# FIXME: uncomment this line and remove the next
#git clone https://github.com/cranestation/reference-sysroot-wasi
git clone /tmp/wut reference-sysroot-wasi

cd reference-sysroot-wasi
git reset --hard d5a609fe63926533e1054e539ba5f2693d51bdf5
make -j$(nproc) INSTALL_DIR=/wasm32-unknown-wasi
make install INSTALL_DIR=/wasm32-unknown-wasi

cd ..
rm -rf reference-sysroot-wasi
rm -rf clang+llvm*

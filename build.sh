#!/bin/bash

export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabihf-gcc \
  CC_ARMV7_UNKNOWN_LINUX_GNUEABIHF=arm-linux-gnueabihf-gcc \
  TARGET_CC=arm-linux-gnueabihf-gcc \
  TARGET_CXX=arm-linux-gnueabihf-g++ \

cargo build --target=armv7-unknown-linux-gnueabihf --release

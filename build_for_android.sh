#!/bin/bash

echo -e "\n*****************    cargo build --target aarch64-linux-android $*     ******************\n"

cargo build --target aarch64-linux-android $*

echo -e "\n*****************     cargo build --target x86_64-linux-android $*    ******************\n"

cargo build --target x86_64-linux-android $*

echo -e "\n*****************    cargo build --target armv7-linux-androideabi $*   ******************\n"

cargo build --target armv7-linux-androideabi $*
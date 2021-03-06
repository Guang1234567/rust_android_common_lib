#!/bin/bash

echo -e "\n*****************    cargo +nightly build --target aarch64-linux-android $*     ******************\n"

cargo +nightly build --target aarch64-linux-android $*

echo -e "\n*****************     cargo +nightly build --target x86_64-linux-android $*    ******************\n"

cargo +nightly build --target x86_64-linux-android $*

echo -e "\n*****************    cargo +nightly build --target armv7-linux-androideabi $*   ******************\n"

cargo +nightly build --target armv7-linux-androideabi $*
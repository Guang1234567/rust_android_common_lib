#!/bin/bash

#/${HOME}/dev_kit/sdk/android_sdk/ndk-bundle/prebuilt/darwin-x86_64/bin/gdb

APP_ID=$1
PORT=1337
PID=$2

adb push gdbserver/x86_64/gdbserver /data/local/tmp

# adb push gdbserver/arm64-v8a/gdbserver /data/local/tmp

adb shell "chmod 777 /data/local/tmp/gdbserver"

adb push start_gdb_server_x86_64.sh /data/local/tmp

adb shell "chmod 777 /data/local/tmp/start_gdb_server_x86_64.sh"

adb root

adb forward tcp:${PORT} tcp:${PORT}

echo -e "/data/local/tmp/start_gdb_server_x86_64.sh ${APP_ID} ${PORT} ${PID}"

adb shell "/data/local/tmp/start_gdb_server_x86_64.sh ${APP_ID} ${PORT} ${PID}"



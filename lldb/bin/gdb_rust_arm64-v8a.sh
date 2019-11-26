#!/bin/bash

# http://ian-ni-lewis.blogspot.com/2011/05/ndk-debugging-without-root-access.html

APP_ID=$1
PORT=1337

#adb push gdbserver/x86_64/gdbserver /data/local/tmp

adb push gdbserver/arm64-v8a/gdbserver /data/local/tmp

PID=`adb shell "ps -A | grep ${APP_ID}" | awk '{ print $2 }'`
echo "${APP_ID} 's pid = ${PID}"

adb shell "run-as ${APP_ID} sh -c 'mkdir -p /data/data/${APP_ID}/files'"

adb shell "cat /data/local/tmp/gdbserver | run-as ${APP_ID} sh -c 'cat > /data/data/${APP_ID}/files/gdbserver && chmod 700 /data/data/${APP_ID}/files/gdbserver'"

adb push start_gdb_server_arm64-v8a.sh /data/local/tmp

adb shell "cat /data/local/tmp/start_gdb_server_arm64-v8a.sh | run-as ${APP_ID} sh -c 'cat > /data/data/${APP_ID}/files/start_gdb_server_arm64-v8a.sh && chmod 700 /data/data/${APP_ID}/files/start_gdb_server_arm64-v8a.sh'"

adb root

adb forward tcp:${PORT} tcp:${PORT}

echo -e "/data/data/${APP_ID}/files/start_gdb_server_arm64-v8a.sh ${APP_ID} ${PORT} ${PID}"

adb shell "run-as ${APP_ID} sh -c '/data/data/${APP_ID}/files/start_gdb_server_arm64-v8a.sh ${APP_ID} ${PORT} ${PID}'"



#!/bin/bash

APP_ID=com.rust.example.android.debug
PORT=1337
PID=3529

adb push gdbserver/x86_64/gdbserver /data/local/tmp

adb shell "cat /data/local/tmp/gdbserver | run-as ${APP_ID} sh -c 'cat > /data/data/${APP_ID}/gdbserver && chmod 700 /data/data/${APP_ID}/gdbserver'"

adb push start_gdb_server_2.sh /data/local/tmp

adb shell "cat /data/local/tmp/start_gdb_server_2.sh | run-as ${APP_ID} sh -c 'cat > /data/data/${APP_ID}/start_gdb_server_2.sh && chmod 700 /data/data/${APP_ID}/start_gdb_server_2.sh'"

adb root

adb forward tcp:${PORT} tcp:${PORT}

echo -e "/data/data/${APP_ID}/start_gdb_server_2.sh ${APP_ID} ${PORT} ${PID}"

adb shell "run-as ${APP_ID} sh -c '/data/data/${APP_ID}/start_gdb_server_2.sh ${APP_ID} ${PORT} ${PID}'"



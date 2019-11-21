#!/bin/bash

APP_ID=$1
PKG=$2

adb push lldb-server/arm64-v8a/lldb-server /data/local/tmp

adb push lldb-server/start_lldb_server.sh /data/local/tmp

#adb shell am start -n "${APP_ID}/${PKG}.MainActivity" -a android.intent.action.MAIN -c android.intent.category.LAUNCHER -D

PID=`adb shell "ps -A | grep ${APP_ID}" | awk '{ print $2 }'`
echo "${APP_ID} 's pid = ${PID}"

adb shell "run-as ${APP_ID} sh -c 'mkdir -p /data/data/${APP_ID}/lldb/bin && mkdir -p /data/data/${APP_ID}/lldb/tmp && mkdir -p /data/data/${APP_ID}/lldb/log'"

adb shell "cat /data/local/tmp/lldb-server | run-as ${APP_ID} sh -c 'cat > /data/data/${APP_ID}/lldb/bin/lldb-server && chmod 700 /data/data/${APP_ID}/lldb/bin/lldb-server'"

adb shell "cat /data/local/tmp/start_lldb_server.sh | run-as ${APP_ID} sh -c 'cat > /data/data/${APP_ID}/lldb/bin/start_lldb_server.sh && chmod 700 /data/data/${APP_ID}/lldb/bin/start_lldb_server.sh'"

echo "Starting LLDB server:"

echo -e "adb shell run-as ${APP_ID} sh -c '/data/data/${APP_ID}/lldb/bin/start_lldb_server.sh /data/data/${APP_ID}/lldb unix-abstract /${APP_ID}-0 platform-debug.sock \"lldb process:gdb-remote packets\"'"

adb shell "run-as ${APP_ID} sh -c '/data/data/${APP_ID}/lldb/bin/start_lldb_server.sh /data/data/${APP_ID}/lldb unix-abstract /${APP_ID}-0 platform-debug.sock \"lldb process:gdb-remote packets\"'"





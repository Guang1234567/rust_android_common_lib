#!/bin/bash

APP_ID=$1

PID=`adb shell "ps -A | grep ${APP_ID}" | awk '{ print $2 }'`
echo "${APP_ID} 's pid = ${PID}"



jdb -connect com.sun.jdi.SocketAttach:hostname=127.0.0.1,port=8604
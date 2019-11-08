#!/system/bin/sh

umask 0002

APP_ID=$1
PORT=$2
PID=$3

#su

#set enforce 0

echo -e "/data/data/${APP_ID}/gdbserver :${PORT} --attach ${PID}"

/data/data/${APP_ID}/gdbserver :${PORT} --attach ${PID}


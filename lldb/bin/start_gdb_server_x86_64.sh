#!/system/bin/sh

umask 0002

APP_ID=$1
PORT=$2
PID=$3

#su

#set enforce 0

echo -e "/data/local/tmp/gdbserver :${PORT} --attach ${PID}"

/data/local/tmp/gdbserver :${PORT} --attach ${PID}


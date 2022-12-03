#!/bin/bash
set -e -x

if [ -z "$1" ]; then
	echo "must pass day"
	exit 1;
fi

PLATFORMIO_BUILD_FLAGS=-DDAY=$1 \
	~/.platformio/penv/bin/platformio \
	run \
	-t upload

sleep 2s

stty -F /dev/ttyACM0 cs8 9600 ignbrk -brkint -imaxbel -opost -onlcr -isig -icanon -iexten -echo -echoe -echok -echoctl -echoke noflsh -ixon -crtscts

exec 3<> /dev/ttyACM0
cat "../input/2022/day$1.txt" <(printf '\0') >&3 &
head -n1 -z <&3
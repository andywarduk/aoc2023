#!/bin/bash

if [ $1 -lt 1 -o $1 -gt 25 ]; then
	echo "Must give day number"
	exit 1
fi

day=$1
daypad="$(printf %02d $day)"

token="$(cat token.txt)"
if [ "x$token" == "x" ]; then
	echo "No token"
	exit 2
fi

curl -sS --cookie "session=$token" https://adventofcode.com/2023/day/$day/input -o inputs/day$daypad.txt

if [ $? -ne 0 ]; then
	echo "Failed to download"
fi

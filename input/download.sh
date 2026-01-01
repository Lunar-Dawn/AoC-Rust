#!/usr/bin/env sh

if [ $# -ne 1 ]; then
    echo "Wrong arguments arguments"
    exit 0
fi


for i in $(seq 1 "$1"); do
	file="day$i.txt"
	if [ ! -f "$file" ]; then
		echo "$file"
		curl -A "Luna's downloader, lunarrequiem42@gmail.com" -b "$(cat cookie)" "https://adventofcode.com/2025/day/$i/input" > "$file"
	fi
done

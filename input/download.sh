#!/usr/bin/env sh

if [ $# -ne 2 ]; then
    echo "Wrong number of arguments"
    exit 0
fi

year=$1
day=$2

if [ ! -d "$year" ]; then
  mkdir "$year"
fi

for i in $(seq "$day"); do
	file=$(printf "$year/day%02g.txt" "$i")
	if [ ! -f "$file" ]; then
		echo "$file"
		curl -A "Luna's downloader, lunarrequiem42@gmail.com" -b "$(cat cookie)" "https://adventofcode.com/$year/day/$i/input" > "$file"
	fi
done

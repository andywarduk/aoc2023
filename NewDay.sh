#!/bin/bash

if [ "x$1" == "x" ]; then
	echo "Must give a day number"
	exit 1
fi

if [ "$1" -lt 1 -o "$1" -gt 25 ]; then
	echo "Day number invalid"
	exit 1
fi

day="$1"
daypad="$(printf %02d $day)"
dir=day$daypad

if [ -d "$dir" ]; then
    echo "$dir already exists"
    exit 2
fi

cp -R template "$dir"

find "$dir" -type f -print | while read file
do
    case $(uname) in
    Darwin)
        sed -i '' "s/\$day/$day/g" "$file"
        sed -i '' "s/\$daypad/$daypad/g" "$file"
        sed -i '' "s/\$dir/$dir/g" "$file"
        ;;
    *)
        sed -i "s/\$day/$day/g" "$file"
        sed -i "s/\$daypad/$daypad/g" "$file"
        sed -i "s/\$dir/$dir/g" "$file"
        ;;
    esac
done

echo "!!! Add to main Cargo.toml !!!"
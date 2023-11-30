#!/bin/bash

if [ "x$1" == "x" ]; then
	echo "Must give a day number"
	exit 1
fi

if [ "$1" -lt 1 -o "$1" -gt 25 ]; then
	echo "Day number invalid"
	exit 1
fi

daypad="$(printf %02d $1)"

cargo build --release --bin day$daypad --quiet

if [ $? -ne 0 ]
then
	echo "Build failed"
	exit 2
fi

case "x$(uname)" in
"xLinux")
	flags="-v"
	outadd="linux"
	uname=$(uname -srvmpio)
	;;
"xDarwin")
	flags="-l"
	outadd="macos"
	uname=$(uname -mprsv)
	;;
*)
	echo "Unrecognised arch"
	exit 3
esac

outfile=stats/day$daypad-$outadd.txt
\time $flags target/release/day$daypad 2>&1 | tee "$outfile"
echo "------------------------------------------" >> "$outfile"
echo $uname >> "$outfile"
rustc -Vv >> "$outfile"

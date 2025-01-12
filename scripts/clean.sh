#!/bin/bash

CDIR="$(dirname "$(readlink -f "$0")")"
TOTAL_REMOVED=0

DIR="$CDIR/../aoc/data"

DIR_SIZE_KB=$(du -sk "$DIR" | awk '{print $1}')
DIR_SIZE_BYTES=$((DIR_SIZE_KB * 1024))

TOTAL_REMOVED=$((TOTAL_REMOVED + DIR_SIZE_BYTES))
cargo clean
# if [ -d "$DIR" ]; then
#   rm -rf "$DIR"
# fi
echo $DIR_SIZE_KB

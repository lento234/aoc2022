#!/bin/bash

DAY=$1
DAYXX=$(printf "%02d" $DAY)
NEWDAY=day$DAYXX

# Ensure we are in git root directory
ROOTDIR=$(git rev-parse --show-toplevel)
echo "Changing to $ROOTDIR"
cd $ROOTDIR

echo "Downloading the puzzle input for day $DAYXX"
curl -s -H "Cookie: session=$(cat .cookie)" https://adventofcode.com/2022/day/${DAY#0}/input > $NEWDAY/input.txt
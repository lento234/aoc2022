#!/bin/bash

DAY=$1
DAYXX=$(printf "%02d" $DAY)
NEWDAY=day$DAYXX

# Ensure we are in git root directory
ROOTDIR=$(git rev-parse --show-toplevel)
cd $ROOTDIR

# Downloading input
curl -s -H "Cookie: session=$(cat .cookie)" https://adventofcode.com/2022/day/${DAY#0}/input

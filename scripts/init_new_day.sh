#!/bin/bash

# Ensure we are in git root directory
ROOTDIR=$(git rev-parse --show-toplevel)
echo "Changing to $ROOTDIR"
cd $ROOTDIR

# New day directory
DAY=$1
DAYXX=$(printf "%02d" $DAY)
NEWDAY=day$DAYXX

echo "DAY: $DAYXX"
# Abort if already exists
if [[ -d $NEWDAY ]]
then
    echo "Aborting: directory already exists!!"
    exit 1
fi

# Initializing new directory using cargo
echo "Initializing new day: day $DAYXX"
cargo new $NEWDAY --vcs=none

# Generate README for new day
echo -e "# Day $DAYXX\n" > $NEWDAY/README.md

# Fetch the challenge of new day
echo "Downloading the puzzle description for day $DAYXX"
./scripts/fetch_description.sh $DAY > $NEWDAY/README.md

# Fetch the input
echo "Downloading the puzzle input for day $DAYXX"
./scripts/fetch_input.sh $DAY > $NEWDAY/input.txt

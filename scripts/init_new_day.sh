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

echo "Initializing new day: day $DAYXX"

# Make new directory for new day
mkdir -v $NEWDAY

# Generate README for new day
echo -e "# Day $DAYXX\n" > $NEWDAY/README.md

# Fetch the challenge of new day
./scripts/fetch_challenge.sh $DAY

# Fetch the input
./scripts/fetch_input.sh $DAY
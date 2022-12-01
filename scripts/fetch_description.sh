#!/bin/bash
DAY=$1
DAYXX=$(printf "%02d" $DAY)
NEWDAY=day$DAYXX

# Ensure we are in git root directory
ROOTDIR=$(git rev-parse --show-toplevel)
cd $ROOTDIR

# Fetch day $DAY using curl and cookie
# convert from HTML to pandoc
# delete all unwanted lines and HTML tags containing {}
# write to README.md
curl -s -H "Cookie: session=$(cat .cookie)" https://adventofcode.com/2022/day/${DAY#0} \
    | pandoc -f html -t markdown \
    | sed -e '/<div>/,/::: {role="main"}/d' -e '/Both parts of this puzzle are complete!/,//d' -e 's/{[^{}]*}//g'
#!/bin/bash

LINES=$(cat savetest)

for LINE in $LINES
do
	echo "$LINE"
	TEST_FILE="$LINE" make functionaltest || exit 1
done

#!/usr/bin/env bash

OUTDIR="tests/expected"
[[ ! -d "$OUTDIR" ]] && mkdir -p "$OUTDIR"
# Example of cyclomatic complxity, which is to say all the ways the input can be combined
# even with the same resulting text
echo "Hello there" > $OUTDIR/hello1.txt
echo "Hello"  "there" > $OUTDIR/hello2.txt
echo -n "Hello  there" > $OUTDIR/hello1.n.txt
echo -n "Hello"  "there" > $OUTDIR/hello2.n.txt

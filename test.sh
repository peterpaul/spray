#!/usr/bin/env bash
set -e

echo "Cleaning up before test"
for i in {1..3}; do
    rm -f $i.txt
done

set -x

cat testdata.in | target/debug/spray "^(?P<x>\d{1}):.*" "\$x.txt"

grep fiets 1.txt
grep hond 2.txt
grep zadel 1.txt
grep konijn 2.txt
grep rust 3.txt
grep emacs 3.txt

echo "All tests passed!"

set +x
echo "Cleaning up after test"

for i in {1..3}; do
    rm -f $i.txt
done

#!/usr/bin/env bash

tempfiles=$(for i in {1..3}; do echo $i.txt; done; echo stdout.txt)
cleanup() {
    for i in $tempfiles;
    do
	rm -f $i
    done
}
trap cleanup 0

error() {
    local parent_lineno="$1"
    local message="$2"
    local code="${3:-1}"
    tput bold
    tput setaf 1
    if [[ -n "$message" ]] ; then
	echo "Error on or near line ${parent_lineno}: ${message}; exiting with status ${code}"
    else
	echo "Error on or near line ${parent_lineno}; exiting with status ${code}"
    fi
    tput sgr0
    exit "${code}"
}
trap 'error ${LINENO}' ERR

echo "Compiling"
cargo build

echo "Cleaning up before test"
cleanup

echo "Running tests"
cat <<HERE | target/debug/spray "^(?P<x>\d{1}):.*" "\$x.txt" > stdout.txt
1: first
2: second
1: third
2: fourth
3: fifth
3: sixth
seventh
HERE

echo "Validating results"
grep "first$" 1.txt >> /dev/null
grep "second$" 2.txt >> /dev/null
grep "third$" 1.txt >> /dev/null
grep "fourth$" 2.txt >> /dev/null
grep "fifth$" 3.txt >> /dev/null
grep "sixth$" 3.txt >> /dev/null
grep "seventh$" stdout.txt >> /dev/null

tput bold
tput setaf 2
echo "All tests passed!"
tput sgr0

#!/bin/sh

for D in `find . -maxdepth 1 -mindepth 1 -type d`;
do
    (cd $D ; echo "I am in `pwd`" ; cargo publish --dry-run)
done
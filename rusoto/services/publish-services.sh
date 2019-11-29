#!/bin/sh

for D in `find . -maxdepth 1 -mindepth 1 -type d | sort`;
do
    (cd $D ; echo "I am in `pwd`" ; cargo publish)
done
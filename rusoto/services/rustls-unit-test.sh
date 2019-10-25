#!/bin/sh

for D in `find . -maxdepth 1 -mindepth 1 -type d`;
do
    (cd $D ; cargo +$1 test --no-default-features --features=rustls )
done
#!/bin/sh

for D in `find . -maxdepth 1 -mindepth 1 -type d | sort`;
do
    # Limit parallelization of compilation to avoid hitting crates.io rate limit.
    # See https://github.com/rusoto/rusoto/issues/1610 .
    (cd $D ; echo "I am in `pwd`" ; cargo publish --jobs 4)
done
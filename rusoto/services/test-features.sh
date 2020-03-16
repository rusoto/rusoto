#!/bin/sh

for D in `find . -maxdepth 1 -mindepth 1 -type d | sort`;
do
    (cd $D ; cargo check --features serialize_structs,deserialize_structs )
done

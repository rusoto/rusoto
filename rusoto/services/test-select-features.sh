#!/bin/sh

for D in cloudwatch cloudfront lambda kinesis ec2;
do
    cd $D && cargo check --features serialize_structs,deserialize_structs && cd .. || exit $?
done

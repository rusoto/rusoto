#!/bin/bash

export AWS_ACCESS_KEY_ID=ANTN35UAENTS5UIAEATD
export AWS_SECRET_ACCESS_KEY=TtnuieannGt2rGuie2t8Tt7urarg5nauedRndrur
export S3_ENDPOINT='http://localhost:9000'

./docker_test_run.py \
    --docker-image="minio/minio" \
    --docker-image="minio/minio:edge" \
    --port=9000 \
    --run-opt=-p=9000:9000 \
    --run-opt=--env=MINIO_ACCESS_KEY=$AWS_ACCESS_KEY_ID \
    --run-opt=--env=MINIO_SECRET_KEY=$AWS_SECRET_ACCESS_KEY \
    --run-opt=--env=MINIO_DOMAIN=localhost \
    --run-arg=server \
    --run-arg=/home/shared \
    -- cargo test --features s3,disable_minio_unsupported

docker rmi minio/minio
docker rmi minio/minio:edge
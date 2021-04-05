#!/usr/bin/env bash
set -Eeu

# Test Ceph with S3 virtual-hosted-style addressing.

export AWS_ACCESS_KEY_ID=ANTN35UAENTS5UIAEATD
export AWS_SECRET_ACCESS_KEY=TtnuieannGt2rGuie2t8Tt7urarg5nauedRndrur
export DOMAIN=localhost
export S3_ENDPOINT="http://$DOMAIN:9000"
export S3_PROXY=http://127.0.0.1:9000

GIT_ROOT=$(git rev-parse --show-toplevel)
cd "$GIT_ROOT/integration_tests"

# Ceph releases supported by upstream:
# http://docs.ceph.com/docs/master/releases/schedule/

./docker_test_run.py \
    --docker-image "ceph/daemon" \
    --port=9000 \
    --run-opt=-p=9000:8080 \
    --run-opt=--env=CEPH_DEMO_ACCESS_KEY=$AWS_ACCESS_KEY_ID \
    --run-opt=--env=CEPH_DEMO_SECRET_KEY=$AWS_SECRET_ACCESS_KEY \
    --run-opt=--env=CEPH_DEMO_UID=demo_uid \
    --run-opt=--env=MON_IP=127.0.0.1 \
    --run-opt=--env=CEPH_PUBLIC_NETWORK=0.0.0.0/0 \
    --run-arg=DEMO \
    --run-opt=--hostname=localhost \
    "--run-opt=-v=$GIT_ROOT/ci/ceph.conf:/etc/ceph.conf:ro" \
    -- cargo test --features s3,disable_ceph_unsupported ---- --test-threads 1

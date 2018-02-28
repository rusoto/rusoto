#!/bin/bash
set -Eeu

# Free some disk space.
docker system prune --all -f

#!/bin/bash
set -Eeu

# Additional directories to cache
# https://semaphoreci.com/docs/caching-between-builds.html#additional-dir-caching
cached_dirs=(
    ~/.cargo
)

for dir in "${cached_dirs[@]}"; do
    dir=$(realpath -s -- "$dir")
    link_target="$SEMAPHORE_CACHE_DIR$dir"

    mkdir -p -- "$dir"
    echo "Caching directory \"$dir\" in \"$link_target\"."
    ln -s -- "$link_target" "$dir"
done

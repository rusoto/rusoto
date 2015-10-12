#!/bin/sh

echo "Requires ghp-import in PATH.  pip install ghp-import if you don't have it.\n"

echo "Generating docs via cargo."
cargo doc --no-deps
echo '<meta http-equiv=refresh content=0;url=rusoto/index.html>' > target/doc/index.html

echo "Firing off ghp-import."
ghp-import -n target/doc -m "Doc push." -p

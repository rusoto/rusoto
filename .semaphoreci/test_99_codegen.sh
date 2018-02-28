#!/bin/bash
set -Eeu

cd "$GIT_ROOT/service_crategen"
git submodule update --init
cargo +nightly run -- generate -c ./services.json -o ../rusoto/services
diff=$(git diff)
if [ -n "$diff" ]; then
    echo
    echo "Differences after regenerating:"
    echo
    echo "$diff"
    echo
    echo -en "\\e[31m"
    echo "ERROR: Generated files differ after regenerating them. Make sure you check in changes"
    echo "       in generated code. Details can be found in service_crategen/README.md."
    echo
    echo "INFO: Have look at the (possibly very large) diff above to see how the generated files"
    echo "      differ. Hint, search for generated.rs in this output."
    echo -en "\\e[0m"
    echo
    exit 1
fi

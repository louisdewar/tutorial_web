# This script takes care of testing your crate

set -ex

# All we care about is whether this compiles right now
main() {
    cross build --target $TARGET
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi

#!/bin/bash
set -ev
if [[ "${TRAVIS_COMMIT_MESSAGE}" = *"Release"* ]]; then
    cargo login $CARGO_LOGIN_KEY
    cargo publish
fi

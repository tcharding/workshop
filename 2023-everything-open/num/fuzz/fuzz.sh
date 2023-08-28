#!/bin/bash

set -e

# For hfuzz options see https://github.com/google/honggfuzz/blob/master/docs/USAGE.md

# We only have one fuzz target.
TARGET=fuzz

# fuzz for 10 seconds
HFUZZ_RUN_ARGS='--run_time 10' chrt -i 0 cargo hfuzz run $TARGET

exit 0

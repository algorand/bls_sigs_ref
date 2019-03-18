#!/bin/bash
#
# (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

[ -z "$1" ] && { echo "Usage: $0 <apps_dir>"; exit 1; }

set -e
set -o pipefail

. "$(dirname "$0")/bench_setup"

rm -f "$1"/bench.out

for app in "${TESTS[@]}"; do
    app_tests="TESTS_${app}[@]"
    for test in ${!app_tests}; do
        "$1/apps/$app" -${test} -n ${NUM_RUNS} <<< "benchmark_input" >> "$1"/bench.out
    done
done

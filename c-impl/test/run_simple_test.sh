#!/bin/bash
#
# (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

set -e
set -o pipefail

if [ ! -z "$2" ]; then
    NRUNS=$2
else
    NRUNS=8192
fi

exec dd if=/dev/urandom bs=1M count=1 status=none | "$1" -n ${NRUNS} "-Tq"

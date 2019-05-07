#!/bin/bash
#
# (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

set -e
set -o pipefail

if [ ! -z "$4" ]; then
    NRUNS=$4
else
    NRUNS=1024
fi

exec dd if=/dev/urandom bs=1M count=1 status=none | "$1" -n ${NRUNS} "-t" | "$2" "$3"

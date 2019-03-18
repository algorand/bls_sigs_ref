#!/bin/bash
#
# (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

[ "$#" != "5" ] && { echo "Usage: $0 <bench_file> <hac_t> <hac_t_fo> <h2ac_t> <h2ac_t_fo>"; exit 1; }

set -e
set -o pipefail

. "$(dirname "$0")/bench_setup"
DIV_VAL=$((1000 * ${NUM_RUNS}))
RND_VAL=$((${DIV_VAL} / 2))

mapfile bench1_vals < "$1"

if [ "${#bench1_vals[@]}" != 34 ] ; then
    echo "ERROR: input files must have 34 lines"
    exit 1
fi

div_round() {
    echo $(($(($1 + ${RND_VAL})) / ${DIV_VAL}))
}

show_q_fq_cq() {
    [ "$#" != "2" ] && { echo "bad args"; exit 1; }

    if [ "$1" = "1" ]; then
        div_round ${bench1_vals[$idx]}
        let idx++ 1
    else
        echo "---"
    fi

    if [ "$2" = "1" ]; then
        echo "\\\\"
    else
        echo "&"
    fi
}

idx=0

get_avg_timing() {
    perl -e '
@timings = <>;
@timings = sort {$b <=> $a} @timings;
$len = $#timings; $tot = 0;
for ($i = 0; $i < $len / 10; $i++) {
    $tot += $timings[$i];
}
$avg = int($tot * 10 / $len / 10 / 1000 + 0.5);
print "$avg\n";
' $1
    echo "&"
}

echo "\$\\bm{G_1}\$ & Hash-and-check & --- &"
show_q_fq_cq 1 0
show_q_fq_cq 1 0
show_q_fq_cq 0 1
echo "%"

echo "& (worst 10\\%) & --- &"
get_avg_timing $2
get_avg_timing $3
echo "---\\\\"
echo "\\cmidrule{2-6}"

echo "& Construction \\#1 & \\S\\ref{sec:blsmap} &"
show_q_fq_cq 1 0
show_q_fq_cq 1 0
show_q_fq_cq 1 1
echo "%"

echo "&& \\S\\ref{sec:blsmap2} &"
show_q_fq_cq 0 0
show_q_fq_cq 1 0
show_q_fq_cq 1 1
echo "\\cmidrule{2-6}"

echo "& Construction \\#2 & \\S\\ref{sec:blsmap} &"
show_q_fq_cq 1 0
show_q_fq_cq 1 0
show_q_fq_cq 1 1
echo "%"

echo "&& \\S\\ref{sec:blsmap2} &"
show_q_fq_cq 0 0
show_q_fq_cq 1 0
show_q_fq_cq 1 1
echo "\\cmidrule{2-6}"

echo "& Construction \\#3 & \\S\\ref{sec:blsmap} &"
show_q_fq_cq 1 0
show_q_fq_cq 1 0
show_q_fq_cq 1 1
echo "%"

echo "&& \\S\\ref{sec:blsmap2} &"
show_q_fq_cq 0 0
show_q_fq_cq 1 0
show_q_fq_cq 1 1
echo "\\midrule[\\heavyrulewidth]"

echo "\$\\bm{G_2}\$ & Hash-and-check & --- &"
show_q_fq_cq 1 0
show_q_fq_cq 1 0
show_q_fq_cq 0 1
echo "%"

echo "& (worst 10\%) & --- &"
get_avg_timing $4
get_avg_timing $5
echo "---\\\\"

echo "\\cmidrule{2-6}"

echo "& Construction \\#4 & \\S\\ref{sec:blsmap} &"
show_q_fq_cq 1 0
show_q_fq_cq 1 0
show_q_fq_cq 1 1
echo "%"

echo "&& \\S\\ref{sec:blsmap2} &"
show_q_fq_cq 0 0
show_q_fq_cq 1 0
show_q_fq_cq 1 1
echo "\\cmidrule{2-6}"

echo "& Construction \\#5 & \\S\\ref{sec:blsmap} &"
show_q_fq_cq 1 0
show_q_fq_cq 1 0
show_q_fq_cq 1 1
echo "%"

echo "&& \\S\\ref{sec:blsmap2} &"
show_q_fq_cq 0 0
show_q_fq_cq 1 0
show_q_fq_cq 1 1
echo "\\cmidrule{2-6}"

echo "& Construction \\#6 & \\S\\ref{sec:blsmap} &"
show_q_fq_cq 1 0
show_q_fq_cq 1 0
show_q_fq_cq 1 1
echo "%"

echo "&& \\S\\ref{sec:blsmap2} &"
show_q_fq_cq 0 0
show_q_fq_cq 1 0
show_q_fq_cq 1 1
echo "%"

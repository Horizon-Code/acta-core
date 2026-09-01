#!/usr/bin/env bash
set -eu

if [ "$#" -ne 1 ]; then
    printf '%s\n' 'usage: check-upstream.sh <OmegaClaw-Core checkout>' >&2
    exit 64
fi

upstream=$1
expected_revision=642c53676cf795cb7a0030823b36018c029b1416
actual_revision=$(git -C "$upstream" rev-parse HEAD)

if [ "$actual_revision" != "$expected_revision" ]; then
    printf '%s\n' "unexpected OmegaClaw revision: $actual_revision" >&2
    exit 1
fi

rg -F '(read_file_tail $history_file (maxHistory))' "$upstream/src/memory.metta" >/dev/null
rg -F '(append-file-raw (library OmegaClaw-Core ./memory/history.metta)' \
    "$upstream/src/memory.metta" >/dev/null
rg -F '(catch (let $R (eval $s)' "$upstream/src/loop.metta" >/dev/null

printf '%s\n' "OMEGACLAW_SOURCE_CHECK=PASS revision=$actual_revision"
printf '%s\n' 'observed=plain-text tail read, raw append, and source-level eval interception point'
printf '%s\n' 'scope=static source compatibility check; absence of a matched integrity check is not proved by grep'

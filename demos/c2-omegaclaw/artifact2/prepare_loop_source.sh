#!/usr/bin/env bash
set -euo pipefail

EXPECTED_COMMIT="642c53676cf795cb7a0030823b36018c029b1416"
EXPECTED_BASE_SHA256="f5b0028fa7b24666d3192e857228fb782d4638469b773eb7352a30a841eab45a"
EXPECTED_PATCHED_SHA256="8735e68001dcbd20acea554420d415685443a5c6d94433c4094a693a759ca8b2"

usage() {
    printf 'usage: %s OMEGACLAW_REPOSITORY\n' "$(basename "$0")" >&2
}

if [[ $# -ne 1 ]]; then
    usage
    exit 2
fi

source_repo="$1"
if [[ ! -d "$source_repo/.git" ]]; then
    printf 'error: not an OmegaClaw git checkout: %s\n' "$source_repo" >&2
    exit 2
fi

resolved_commit="$(git -C "$source_repo" rev-parse --verify "${EXPECTED_COMMIT}^{commit}")"
if [[ "$resolved_commit" != "$EXPECTED_COMMIT" ]]; then
    printf 'error: expected commit object %s, resolved %s\n' \
        "$EXPECTED_COMMIT" "$resolved_commit" >&2
    exit 2
fi

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
patch_file="$script_dir/patches/0001-log-raw-eval-before-normalization.patch"
output_root="$(mktemp -d "${TMPDIR:-/tmp}/acta-c2-raw-eval.XXXXXXXX")"
cleanup_on_error() {
    find "$output_root" -depth -delete
}
trap cleanup_on_error ERR INT TERM

mkdir -p "$output_root/src"
git -C "$source_repo" show "${EXPECTED_COMMIT}:src/loop.metta" \
    >"$output_root/src/loop.metta"

actual_base_sha256="$(sha256sum "$output_root/src/loop.metta" | cut -d ' ' -f 1)"
if [[ "$actual_base_sha256" != "$EXPECTED_BASE_SHA256" ]]; then
    printf 'error: pristine src/loop.metta hash mismatch\n' >&2
    exit 2
fi

(
    cd "$output_root"
    git apply --check "$patch_file"
    git apply "$patch_file"
)

actual_patched_sha256="$(sha256sum "$output_root/src/loop.metta" | cut -d ' ' -f 1)"
if [[ "$actual_patched_sha256" != "$EXPECTED_PATCHED_SHA256" ]]; then
    printf 'error: instrumented src/loop.metta hash mismatch\n' >&2
    exit 2
fi

trap - ERR INT TERM
printf '%s\n' "$output_root"

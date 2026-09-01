#!/usr/bin/env bash
set -eu

artifact_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
demo_dir=$(mktemp -d /tmp/acta-c2-omegaclaw-artifact1.XXXXXX)

cleanup() {
    case "$demo_dir" in
        /tmp/acta-c2-omegaclaw-artifact1.*) rm -rf -- "$demo_dir" ;;
        *) printf '%s\n' "refusing to clean unexpected path: $demo_dir" >&2 ;;
    esac
}
trap cleanup EXIT HUP INT TERM

operator_dir="$demo_dir/operator"
external_dir="$demo_dir/external-party"
history="$operator_dir/memory/history.metta"
evidence="$operator_dir/acta-evidence"
witness="$external_dir/epoch-witness.json"

mkdir -p "$operator_dir/memory" "$external_dir"
cp "$artifact_dir/fixtures/history.metta" "$history"

cargo build --offline --quiet --manifest-path "$artifact_dir/Cargo.toml"
artifact1="$artifact_dir/target/debug/acta-c2-omegaclaw-artifact1"

"$artifact1" seal "$history" "$evidence" "$witness"
"$artifact1" local-check "$history"
"$artifact1" verify "$history" "$evidence" "$witness"

printf '%s\n' '--- operator deletes the middle OmegaClaw record ---'
"$artifact1" delete-record "$history" C2-A1-TURN-002
"$artifact1" local-check "$history"

set +e
"$artifact1" verify "$history" "$evidence" "$witness"
verify_status=$?
set -e

if [ "$verify_status" -ne 2 ]; then
    printf '%s\n' "expected ACTA detection exit 2, got $verify_status" >&2
    exit 1
fi
printf '%s\n' 'DEMO_RESULT=PASS (local trace remains readable; external ACTA commitment detects deletion)'

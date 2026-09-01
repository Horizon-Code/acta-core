#!/usr/bin/env python3

from __future__ import annotations

import copy
import json
import unittest
from pathlib import Path

from audit_links import (
    CHAIN_CODE,
    NONDET_CODE,
    SELECTION_DISCLAIMER,
    InputError,
    audit_document,
    harness_transform,
    render_text,
)


HERE = Path(__file__).resolve().parent
REPO = HERE.parents[2]
VECTOR = HERE / "vectors" / "e0-capture3.json"
MEASUREMENTS = REPO / "research" / "e0-logs" / "c3-measurements.json"
PROBE_PATCH = HERE / "patches" / "0001-log-raw-eval-before-normalization.patch"


def load_vector() -> dict:
    return json.loads(VECTOR.read_text(encoding="utf-8"))


class HarnessTransformTests(unittest.TestCase):
    def test_measured_order_escapes_then_keeps_tail(self) -> None:
        harness = load_vector()["harness"]
        harness["maxFeedback"] = 20
        self.assertEqual(
            harness_transform('head\n""x\'tail', harness),
            "te_x_apostrophe_tail",
        )

    def test_rejects_undeclared_transform_order(self) -> None:
        harness = load_vector()["harness"]
        harness["transform_order"].reverse()
        with self.assertRaises(InputError):
            harness_transform("x", harness)

    def test_rejects_ambiguous_string_safe_declaration(self) -> None:
        harness = load_vector()["harness"]
        del harness["string-safe"]["newline"]
        with self.assertRaises(InputError):
            harness_transform("x", harness)


class RawEvalProbeTests(unittest.TestCase):
    def test_patch_only_intercepts_loop_step_five(self) -> None:
        patch = PROBE_PATCH.read_text(encoding="utf-8")
        self.assertEqual(patch.count("diff --git "), 1)
        self.assertIn("diff --git a/src/loop.metta b/src/loop.metta", patch)
        self.assertNotIn("plugins/", patch)

    def test_probe_records_eval_result_before_normalization(self) -> None:
        patch = PROBE_PATCH.read_text(encoding="utf-8")
        eval_position = patch.index("(let $R (eval $s)")
        probe_position = patch.index('(log INFO "e0_capture3" (RAW_EVAL: $s $R))')
        # The diff also contains the removed one-line implementation; rindex selects the
        # normalization call on the added side of the hunk.
        normalize_position = patch.rindex("(py-call (helper.normalize_string $R))")
        self.assertLess(eval_position, probe_position)
        self.assertLess(probe_position, normalize_position)


class LinkAuditTests(unittest.TestCase):
    def test_e0_vector_is_five_of_five_only_with_scope_and_disclaimer(self) -> None:
        report = audit_document(load_vector())
        metric = report["chain_mediation"]
        self.assertEqual((metric["faithful"], metric["free"], metric["total"]), (5, 0, 5))
        self.assertTrue(metric["explicit_byte_copy_instruction"])
        self.assertIn("Techo empírico bajo instrucción explícita de copia", metric["statement"])
        self.assertIn(SELECTION_DISCLAIMER, metric["statement"])
        self.assertEqual(
            {condition["code"] for condition in report["conditions"]},
            {CHAIN_CODE, NONDET_CODE},
        )
        self.assertEqual(
            [link["classification"] for link in report["links"]],
            ["faithful"] * 5,
        )
        self.assertFalse(
            report["verification_context"]["adr004_historical_lockfile_verified"]
        )
        self.assertTrue(
            all(
                link["classification_basis"]
                == "contract-validation-over-synthetic-verification-envelope"
                for link in report["links"]
            )
        )
        self.assertEqual(
            set(report["verification_context"]["lockfile_binding"]["source_hashes"]),
            {"helper_py", "loop_metta_base", "utils_metta", "loop_metta_probe"},
        )

    def test_vector_strings_and_line_provenance_match_e0_measurements(self) -> None:
        vector = load_vector()
        measurements = json.loads(MEASUREMENTS.read_text(encoding="utf-8"))
        measured = {row["task"]: row for row in measurements["rows"]}
        for link in vector["links"]:
            task = link["link_id"].split("-")[2]
            row = measured[task]
            self.assertEqual(link["conclusion"]["serialized"], row["hop1_raw_eval_output"])
            self.assertEqual(link["premise"]["serialized"], row["hop2_carried_premise"])
            self.assertEqual(link["source"]["conclusion_line"], row["hop1_raw_eval_line"])
            self.assertEqual(link["source"]["context_line"], row["context_line"])
            self.assertEqual(link["source"]["premise_call_line"], row["hop2_raw_eval_line"])
            self.assertEqual(
                link["context_evidence"]["last_skill_use_results_exact"],
                row["last_skill_use_results_exact"],
            )

            raw_lines = (REPO / link["source"]["rawlog"]).read_text(
                encoding="utf-8"
            ).splitlines()
            conclusion_line = raw_lines[link["source"]["conclusion_line"] - 1]
            context_line = raw_lines[link["source"]["context_line"] - 1]
            premise_line = raw_lines[link["source"]["premise_call_line"] - 1]
            self.assertIn(link["conclusion"]["serialized"], conclusion_line)
            self.assertIn(
                link["context_evidence"]["last_skill_use_results_exact"], context_line
            )
            self.assertIn(link["premise"]["serialized"], premise_line)

        for result in audit_document(vector)["links"]:
            strings = result["evidence_strings"]
            self.assertEqual(
                set(strings),
                {
                    "raw_conclusion",
                    "transformed_conclusion_T_C",
                    "last_skill_use_results_exact",
                    "next_premise",
                },
            )

    def test_literal_match_is_free_when_process_differs(self) -> None:
        document = load_vector()
        document["links"] = [copy.deepcopy(document["links"][0])]
        document["links"][0]["premise"]["process_ref"]["process_id"] = (
            "another-process"
        )
        report = audit_document(document)
        link = report["links"][0]
        self.assertEqual(link["classification"], "free")
        self.assertIn("same_process_ref", link["failed_faithful_predicates"])

    def test_process_type_mismatch_is_free(self) -> None:
        document = load_vector()
        document["links"] = [copy.deepcopy(document["links"][0])]
        document["links"][0]["premise"]["process_ref"]["process_type"] = (
            "another.profile"
        )
        report = audit_document(document)
        link = report["links"][0]
        self.assertEqual(link["classification"], "free")
        self.assertIn("same_process_ref", link["failed_faithful_predicates"])

    def test_literal_match_is_free_without_verified_precedence(self) -> None:
        document = load_vector()
        document["links"] = [copy.deepcopy(document["links"][0])]
        document["links"][0]["chronos"]["order_verified"] = False
        report = audit_document(document)
        link = report["links"][0]
        self.assertEqual(link["classification"], "free")
        self.assertIn(
            "conclusion_precedes_premise_in_verified_chronos_order",
            link["failed_faithful_predicates"],
        )

    def test_declared_T_not_raw_equality_controls_classification(self) -> None:
        document = load_vector()
        document["links"] = [copy.deepcopy(document["links"][0])]
        conclusion = "prefix\nO'Reilly"
        transformed = harness_transform(conclusion, document["harness"])
        document["links"][0]["conclusion"]["serialized"] = conclusion
        document["links"][0]["premise"]["serialized"] = transformed
        report = audit_document(document)
        link = report["links"][0]
        self.assertFalse(link["predicates"]["bytes_premise_eq_raw_conclusion"])
        self.assertTrue(link["predicates"]["bytes_premise_eq_T_conclusion"])
        self.assertEqual(link["classification"], "faithful")

    def test_nondeterministic_input_is_distinct_and_only_emitted_when_declared(self) -> None:
        document = load_vector()
        document["links"] = [copy.deepcopy(document["links"][0])]
        document["links"][0]["premise"]["nondeterministic_origin"] = "none"
        report = audit_document(document)
        self.assertEqual([condition["code"] for condition in report["conditions"]], [CHAIN_CODE])
        self.assertNotIn(NONDET_CODE, render_text(report))

    def test_chain_condition_requires_present_mediator_but_nondet_is_separate(self) -> None:
        document = load_vector()
        document["mediator"] = {
            "present": False,
            "nondeterministic": False,
            "kind": "",
        }
        report = audit_document(document)
        self.assertEqual(
            [condition["code"] for condition in report["conditions"]],
            [NONDET_CODE],
        )
        self.assertFalse(report["chain_mediation"]["condition_emitted"])
        self.assertNotIn(CHAIN_CODE, json.dumps(report))

    def test_bool_is_rejected_where_chronos_index_must_be_int(self) -> None:
        document = load_vector()
        document["links"] = [copy.deepcopy(document["links"][0])]
        document["links"][0]["chronos"]["conclusion_index"] = True
        with self.assertRaises(InputError):
            audit_document(document)

    def test_negative_chronos_index_is_rejected(self) -> None:
        document = load_vector()
        document["links"] = [copy.deepcopy(document["links"][0])]
        document["links"][0]["chronos"]["conclusion_index"] = -1
        with self.assertRaisesRegex(InputError, "non-negative"):
            audit_document(document)

    def test_historical_lockfile_cannot_be_self_asserted(self) -> None:
        document = load_vector()
        document["verification_context"]["kind"] = "historical-lockfile"
        document["verification_context"]["historical_lockfile_present"] = True
        with self.assertRaisesRegex(InputError, "ADR-004 verifier"):
            audit_document(document)

    def test_bool_is_rejected_where_max_feedback_must_be_int(self) -> None:
        document = load_vector()
        document["harness"]["maxFeedback"] = True
        with self.assertRaises(InputError):
            audit_document(document)

    def test_harness_config_hash_mismatch_is_rejected(self) -> None:
        document = load_vector()
        document["harness"]["maxHistory"] += 1
        with self.assertRaisesRegex(InputError, "harness_config_sha256 mismatch"):
            audit_document(document)

    def test_declared_transform_hash_is_recomputed(self) -> None:
        document = load_vector()
        document["verification_context"]["lockfile_binding"]["transform_sha256"] = (
            "sha256:" + "0" * 64
        )
        with self.assertRaisesRegex(InputError, "transform_sha256 mismatch"):
            audit_document(document)

    def test_source_closure_binding_mismatch_is_rejected(self) -> None:
        document = load_vector()
        document["verification_context"]["lockfile_binding"]["source_hashes"][
            "utils_metta"
        ] = "sha256:" + "0" * 64
        with self.assertRaisesRegex(InputError, "source_hashes"):
            audit_document(document)


if __name__ == "__main__":
    unittest.main()

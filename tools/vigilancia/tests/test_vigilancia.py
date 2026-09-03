import datetime as dt
import importlib.util
import sys
import tempfile
import unittest
from pathlib import Path


MODULE_PATH = Path(__file__).parents[1] / "main.py"
SPEC = importlib.util.spec_from_file_location("acta_vigilancia", MODULE_PATH)
vigilancia = importlib.util.module_from_spec(SPEC)
assert SPEC.loader is not None
sys.modules[SPEC.name] = vigilancia
SPEC.loader.exec_module(vigilancia)


ATOM = b"""<?xml version='1.0' encoding='utf-8'?>
<feed xmlns='http://www.w3.org/2005/Atom'>
  <entry>
    <id>draft-1</id><title>Agent evidence draft</title>
    <updated>2026-09-01T10:00:00Z</updated>
    <link href='https://example.test/draft-1'/>
  </entry>
</feed>
"""


class FeedTests(unittest.TestCase):
    def test_parses_atom(self):
        self.assertEqual(
            vigilancia.parse_feed(ATOM),
            [{
                "id": "draft-1",
                "title": "Agent evidence draft",
                "link": "https://example.test/draft-1",
                "date": "2026-09-01T10:00:00Z",
            }],
        )

    def test_initial_atom_run_only_sets_baseline(self):
        config = {
            "url": "https://example.test/feed",
            "display_id": "A1",
            "label": "Test",
            "always_attention": True,
        }
        original = vigilancia.fetch_bytes
        vigilancia.fetch_bytes = lambda *_args, **_kwargs: ATOM
        try:
            result = vigilancia.collect_atom(config, {})
        finally:
            vigilancia.fetch_bytes = original
        self.assertEqual(result.observations, [])
        self.assertFalse(result.changed)
        self.assertEqual(result.snapshot["ids"], ["draft-1"])

    def test_new_highlighted_entry_requires_attention(self):
        config = {
            "url": "https://example.test/feed",
            "display_id": "A3",
            "label": "IETF",
            "highlight_terms": ["evidence"],
        }
        original = vigilancia.fetch_bytes
        vigilancia.fetch_bytes = lambda *_args, **_kwargs: ATOM
        try:
            result = vigilancia.collect_atom(config, {"ids": []})
        finally:
            vigilancia.fetch_bytes = original
        self.assertTrue(result.changed)
        self.assertTrue(result.observations[0].attention)
        self.assertTrue(result.observations[0].highlighted)

    def test_source_limit_is_hard(self):
        payload = ATOM.replace(
            b"</feed>",
            b"<entry><id>draft-2</id><title>Second evidence draft</title>"
            b"<updated>2026-09-02T10:00:00Z</updated>"
            b"<link href='https://example.test/draft-2'/></entry></feed>",
        )
        config = {
            "url": "https://example.test/feed",
            "display_id": "A8",
            "label": "arXiv",
            "max_observations": 1,
        }
        original = vigilancia.fetch_bytes
        vigilancia.fetch_bytes = lambda *_args, **_kwargs: payload
        try:
            result = vigilancia.collect_atom(config, {"ids": []})
        finally:
            vigilancia.fetch_bytes = original
        self.assertEqual(len(result.observations), 1)


class LotlTests(unittest.TestCase):
    def test_counts_service_type_identifiers(self):
        payload = b"""<TrustServiceStatusList xmlns='urn:test'>
          <TSLSequenceNumber>391</TSLSequenceNumber>
          <TSPService><ServiceTypeIdentifier>urn:ElectronicLedger</ServiceTypeIdentifier></TSPService>
          <TSPService><ServiceTypeIdentifier>urn:Other</ServiceTypeIdentifier></TSPService>
        </TrustServiceStatusList>"""
        sequence, counts = vigilancia.lotl_counts(payload, ["ElectronicLedger", "ElectronicArchiving"])
        self.assertEqual(sequence, "391")
        self.assertEqual(counts, {"ElectronicLedger": 1, "ElectronicArchiving": 0})

    def test_extracts_unique_https_national_lists(self):
        payload = b"""<TrustServiceStatusList xmlns='urn:test'>
          <OtherTSLPointer><TSLLocation>https://example.test/es.xml</TSLLocation></OtherTSLPointer>
          <OtherTSLPointer><TSLLocation>http://example.test/insecure.xml</TSLLocation></OtherTSLPointer>
          <OtherTSLPointer><TSLLocation>https://example.test/es.xml</TSLLocation></OtherTSLPointer>
        </TrustServiceStatusList>"""
        self.assertEqual(vigilancia.lotl_locations(payload), ["https://example.test/es.xml"])


class ReportTests(unittest.TestCase):
    def test_hard_limit_moves_overflow_to_archive(self):
        observations = [
            vigilancia.Observation("A4", "Source", f"change {index}", "https://example.test", "2026-09-03", True, True)
            for index in range(7)
        ]
        report = vigilancia.render_report(2026, 36, observations, ["A1"], [], 5)
        attention, archive = report.split("## Archivo (sin acción)")
        self.assertEqual(attention.count("https://example.test"), 5)
        self.assertEqual(archive.count("[exceso del límite de atención]"), 2)
        self.assertIn("A1", report)
        self.assertIn("(ninguno)", report)

    def test_state_round_trip(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "state.json"
            vigilancia.save_state(path, {"A1": {"ids": ["one"]}})
            self.assertEqual(vigilancia.load_state(path)["sources"]["A1"]["ids"], ["one"])


if __name__ == "__main__":
    unittest.main()

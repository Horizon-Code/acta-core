#!/usr/bin/env python3
"""Extract the three E0/3 evidence strings and apply both byte predicates."""

from __future__ import annotations

import json
import re
from pathlib import Path


HERE = Path(__file__).resolve().parent
SELECTED = {
    "t2": ("sparrow", "bird", "animal", "living_thing"),
    "t3": ("copper", "metal", "conductor", "useful_material"),
    "t4": ("oak", "tree", "plant", "organism"),
    "t5": ("salmon", "fish", "aquatic_animal", "living_thing"),
    "t7": ("rose", "flower", "plant", "organism"),
}

RAW_RE = re.compile(
    r'\| e0_capture3 \| \(RAW_EVAL: \(metta "(?P<input>.*)"\) '
    r'"\\"(?P<output>.*)\\""\)$'
)


def split_top(text: str) -> list[str]:
    result: list[str] = []
    depth = 0
    start = None
    quoted = False
    escaped = False
    for index, char in enumerate(text):
        if quoted:
            if escaped:
                escaped = False
            elif char == "\\":
                escaped = True
            elif char == '"':
                quoted = False
            continue
        if char == '"':
            quoted = True
        elif char == "(":
            if depth == 0:
                start = index
            depth += 1
        elif char == ")":
            depth -= 1
            if depth == 0 and start is not None:
                result.append(text[start : index + 1])
                start = None
    return result


def inference_premises(expression: str) -> list[str]:
    start = expression.index("(|-")
    depth = 0
    end = None
    for index in range(start, len(expression)):
        if expression[index] == "(":
            depth += 1
        elif expression[index] == ")":
            depth -= 1
            if depth == 0:
                end = index
                break
    if end is None:
        raise ValueError(f"unbalanced inference expression: {expression}")
    return split_top(expression[start + len("(|-") : end].strip())


def harness_transform(raw: str, max_feedback: int = 50_000) -> str:
    """Code-order T: normalize_string, string-safe, then last_chars."""
    normalized = raw.encode("utf-8", errors="ignore").decode(
        "utf-8", errors="ignore"
    )
    safe = (
        normalized.replace('""', "_quote_")
        .replace("\n", "_newline_")
        .replace("'", "_apostrophe_")
    )
    return safe[-max_feedback:]


def extract(tag: str, terms: tuple[str, str, str, str]) -> dict[str, object]:
    a, b, c, d = terms
    path = HERE / f"c3-{tag}.rawlog"
    lines = path.read_text(encoding="utf-8").splitlines()
    calls: list[dict[str, object]] = []
    for line_number, line in enumerate(lines, start=1):
        match = RAW_RE.search(line)
        if match:
            calls.append(
                {
                    "line": line_number,
                    "input": match.group("input"),
                    "output": match.group("output"),
                }
            )

    hop1 = next(
        call
        for call in calls
        if f"(--> {a} {b})" in str(call["input"])
        and f"(--> {b} {c})" in str(call["input"])
    )
    hop2 = next(
        call
        for call in calls
        if int(call["line"]) > int(hop1["line"])
        and f"(--> {a} {c})" in str(call["input"])
        and f"(--> {c} {d})" in str(call["input"])
    )

    context_line_number = next(
        number
        for number in range(int(hop1["line"]) + 1, int(hop2["line"]))
        if " LAST_SKILL_USE_RESULTS: " in lines[number - 1]
    )
    context_line = lines[context_line_number - 1]
    context_block = context_line.split(" LAST_SKILL_USE_RESULTS: ", 1)[1].split(
        " HISTORY: ", 1
    )[0]

    premises = inference_premises(str(hop2["input"]))
    carried = next(premise for premise in premises if f"(--> {a} {c})" in premise)
    raw = str(hop1["output"])
    transformed = harness_transform(raw)

    return {
        "task": tag,
        "rawlog": path.name,
        "hop1_raw_eval_line": hop1["line"],
        "context_line": context_line_number,
        "hop2_raw_eval_line": hop2["line"],
        "hop1_raw_eval_output": raw,
        "last_skill_use_results_exact": context_block,
        "hop2_carried_premise": carried,
        "hop2_all_premises": premises,
        "T_declared": {
            "normalize_string": "utf-8 encode/decode, errors=ignore",
            "string_safe": [
                "double-double-quote -> _quote_",
                "newline -> _newline_",
                "apostrophe -> _apostrophe_",
            ],
            "maxFeedback_last_chars": 50_000,
            "result_for_raw_output": transformed,
        },
        "bytes_premise_eq_raw": carried.encode() == raw.encode(),
        "bytes_premise_eq_T_raw": carried.encode() == transformed.encode(),
        "context_contains_encoded_raw": f"_quote_{transformed}_quote_"
        in context_block,
        "classification": "faithful"
        if carried.encode() == transformed.encode()
        else "free",
    }


def main() -> None:
    rows = [extract(tag, terms) for tag, terms in SELECTED.items()]
    output = {
        "sample": list(SELECTED),
        "faithful": sum(row["classification"] == "faithful" for row in rows),
        "free": sum(row["classification"] == "free" for row in rows),
        "rows": rows,
    }
    destination = HERE / "c3-measurements.json"
    destination.write_text(
        json.dumps(output, ensure_ascii=False, indent=2) + "\n",
        encoding="utf-8",
    )
    print(destination)
    print(f"faithful={output['faithful']} free={output['free']}")


if __name__ == "__main__":
    main()

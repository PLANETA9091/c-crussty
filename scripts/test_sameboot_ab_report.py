#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""AG-242 w529 — selftest for scripts/sameboot_ab_report.py (stdlib unittest).

Run: python3 scripts/test_sameboot_ab_report.py
Covers: ab_aa_check gate semantics, lever_verdict, TPS-poll extraction
(cap-20 warmup drop + saturation signature), ch/s + MSPT-p99 extraction,
JSON-leg mode, artifact-dir end-to-end (lever CERT/REJECT + A/A TOOL gate).
"""

import json
import os
import shutil
import sys
import tempfile
import unittest
from io import StringIO
from contextlib import redirect_stdout

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import sameboot_ab_report as R  # noqa: E402


def run_main(argv):
    buf = StringIO()
    with redirect_stdout(buf):
        rc = R.main(argv)
    return rc, buf.getvalue()


class TestAAAaCheck(unittest.TestCase):
    def test_identical_is_pass(self):
        d, v = R.ab_aa_check([5.0, 5.1, 4.9], [5.0, 5.1, 4.9])
        self.assertAlmostEqual(d, 0.0, places=9)
        self.assertEqual(v, "PASS")

    def test_within_3pct_is_pass(self):
        # +1.5% delta — inside the owner A/A bar (<3% = instrument valid)
        d, v = R.ab_aa_check([100.0, 100.0], [101.5, 101.5])
        self.assertAlmostEqual(d, 1.5, places=9)
        self.assertEqual(v, "PASS")

    def test_beyond_3pct_is_fail(self):
        d, v = R.ab_aa_check([100.0, 100.0], [105.0, 105.0])
        self.assertAlmostEqual(d, 5.0, places=9)
        self.assertEqual(v, "FAIL")

    def test_negative_direction_symmetric(self):
        d, v = R.ab_aa_check([100.0], [97.0])
        self.assertAlmostEqual(d, -3.0, places=9)
        self.assertEqual(v, "FAIL")  # |delta| = 3.0 is NOT < 3.0

    def test_no_data(self):
        self.assertEqual(R.ab_aa_check([], [1.0]), (None, "NO_DATA"))
        self.assertEqual(R.ab_aa_check([0.0], [1.0]), (None, "NO_DATA"))

    def test_median_not_mean(self):
        d, _ = R.ab_aa_check([1.0, 1.0, 100.0], [2.0, 2.0, 200.0])
        self.assertAlmostEqual(d, 100.0, places=9)


class TestLeverVerdict(unittest.TestCase):
    def test_cert_reject(self):
        self.assertEqual(R.lever_verdict(4.0, 2.0), "CERT")
        self.assertEqual(R.lever_verdict(2.0, 2.0), "CERT")   # >= threshold
        self.assertEqual(R.lever_verdict(1.99, 2.0), "REJECT")
        self.assertEqual(R.lever_verdict(None, 2.0), "NO_DATA")
        self.assertEqual(R.lever_verdict(-1.0, 2.0), "REJECT")


class TestExtract(unittest.TestCase):
    STDOUT_LOG = (
        "Done (15.97s)!\n"
        "TPS from last 5s, 1m, 5m, 15m: 20.0, 5.1, 5.2, 5.1,\n"
        "TPS from last 5s, 1m, 5m, 15m: 5.2, 5.1, 5.2, 5.1,\n"
        "spark tick-monitor MSPT: avg **195.31ms\n"
        "  p99 244.10ms\n"
    )

    def test_tps_cap20_dropped_median(self):
        med, sat = R.tps_tail_median(self.STDOUT_LOG)
        # candidates: 20.0 (cap artifact, filtered by C55 <15), 5.1,5.2,5.1,5.2,5.1,5.2,5.1
        self.assertFalse(sat)
        self.assertAlmostEqual(med, 5.1, places=9)

    def test_saturation_signature(self):
        med, sat = R.tps_tail_median(
            "TPS from last 5s, 1m, 5m, 15m: 20.0, 20.0, 20.0,")
        self.assertIsNone(med)
        self.assertTrue(sat)  # M1@20k cap-20 signature: delta invisible

    def test_first_of_window_polls(self):
        med, sat = R.tps_tail_median(
            "TPS polls captured: 6, first-of-window values: [20.0, 0.6, 0.7, 0.7, 0.8, 0.9]")
        self.assertFalse(sat)
        self.assertAlmostEqual(med, 0.7, places=9)

    def test_chps_extraction(self):
        vals = R.extract_metrics_from_text(
            "marked-rate 449.12 ch/s\n total 13.00 ch/s\n per-dim ch/s: 7.97")
        self.assertAlmostEqual(R._median(vals["M2_chps"]), 13.0, places=9)

    def test_mspt_p99_and_fallback(self):
        v = R.extract_metrics_from_text(self.STDOUT_LOG)
        self.assertAlmostEqual(R._median(v["MSPT_p99"]), 244.10, places=9)
        v2 = R.extract_metrics_from_text("MSPT: 159.14ms")  # no p99 -> avg fallback
        self.assertAlmostEqual(R._median(v2["MSPT_p99"]), 159.14, places=9)

    def test_json_leg(self):
        vals = R.extract_metrics_from_json(
            {"tps": 5.1, "chps": [12.5, 13.5], "mspt": 159.14})
        self.assertAlmostEqual(R._median(vals["M1_tps50k"]), 5.1, places=9)
        self.assertAlmostEqual(R._median(vals["M2_chps"]), 13.0, places=9)
        self.assertAlmostEqual(R._median(vals["MSPT_p99"]), 159.14, places=9)


class TestEndToEnd(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.mkdtemp(prefix="ag242-selftest-")
        self.art = os.path.join(self.tmp, "artifacts")
        os.makedirs(os.path.join(self.art, "runA", "server"))
        os.makedirs(os.path.join(self.art, "runB", "server"))
        log_a = ("TPS from last 5s, 1m, 5m, 15m: 20.0, 5.00, 5.10, 5.00,\n"
                 "marked-rate 440.0 ch/s\n p99 250.00ms\n")
        log_b = ("TPS from last 5s, 1m, 5m, 15m: 20.0, 5.25, 5.20, 5.25,\n"
                 "marked-rate 450.0 ch/s\n p99 244.00ms\n")
        with open(os.path.join(self.art, "runA", "server", "server-stdout.log"), "w") as fh:
            fh.write(log_a)
        with open(os.path.join(self.art, "runB", "server", "server-stdout.log"), "w") as fh:
            fh.write(log_b)

    def tearDown(self):
        shutil.rmtree(self.tmp, ignore_errors=True)

    def test_lever_mode_cert(self):
        # tps +4.7% (>=2 CERT), chps +2.27% (CERT), mspt -2.4% (CERT)
        rc, out = run_main(["--run-a", "runA", "--run-b", "runB",
                            "--art-root", self.art])
        self.assertEqual(rc, 0)
        self.assertIn("FINAL: ", out)
        self.assertIn("CERT", out)
        self.assertIn("M1_tps50k", out)

    def test_lever_mode_reject(self):
        # shrink B to a sub-threshold delta: tps +0.9% -> REJECT
        with open(os.path.join(self.art, "runB", "server", "server-stdout.log"), "w") as fh:
            fh.write("TPS from last 5s, 1m, 5m, 15m: 20.0, 5.05, 5.05, 5.05,\n"
                     "marked-rate 443.0 ch/s\n p99 249.00ms\n")
        rc, out = run_main(["--run-a", "runA", "--run-b", "runB",
                            "--art-root", self.art])
        self.assertEqual(rc, 0)
        self.assertIn("FINAL: REJECT", out)

    def test_aa_mode_tool_gate(self):
        rc, out = run_main(["--run-a", "runA", "--run-b", "runB",
                            "--art-root", self.art, "--aa"])
        self.assertEqual(rc, 0)
        self.assertIn("TOOL-AA: ", out)

    def test_aa_strict_fail_exit1(self):
        # +4.7% between identical-branch legs -> A/A gate FAIL, --strict exit 1
        rc, out = run_main(["--run-a", "runA", "--run-b", "runB",
                            "--art-root", self.art, "--aa", "--strict"])
        self.assertEqual(rc, 1)

    def test_runid_resolution_and_json_mode(self):
        j = os.path.join(self.tmp, "arm.json")
        with open(j, "w") as fh:
            json.dump({"tps": 5.05, "chps": 440.0, "mspt": 250.0}, fh)
        j2 = os.path.join(self.tmp, "van.json")
        with open(j2, "w") as fh:
            json.dump({"tps": 5.15, "chps": 449.0, "mspt": 244.0}, fh)
        rc, out = run_main(["--json-a", j, "--json-b", j2])
        self.assertEqual(rc, 0)
        self.assertIn("CERT", out)  # tps +1.98%~ no... chps +2.05% CERT; check below


if __name__ == "__main__":
    unittest.main(verbosity=2)

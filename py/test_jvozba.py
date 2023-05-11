"""
Copyright (c) 2020 uakci (https://github.com/uakci)
Licensed under the MIT License

Modified by latkerlo (https://github.com/latkerlo), Copyright (c) 2023
"""

import unittest
from jvozba import get_lujvo
from jvozba_test_list import JVOZBAJVEKAhA_TESTS, CMENE_TESTS, JVOZBA_ONLY_TESTS, LUJVO_SCORE_TESTS, JVOZBA_FAIL_TESTS


class TestJvoZba(unittest.TestCase):
    def test_jvozba(self):
        for i, n in enumerate(JVOZBAJVEKAhA_TESTS + JVOZBA_ONLY_TESTS):
            lujvo, tanru = n
            res, score = get_lujvo(tanru)
            self.assertEqual(lujvo, res, "on example #{}: expected {}, got {} (input: {})".format(i, lujvo, res, tanru))

    def test_jvozba_cmene(self):
        for i, n in enumerate(CMENE_TESTS):
            lujvo, tanru = n
            res, score = get_lujvo(tanru, generate_cmene=True)
            self.assertEqual(lujvo, res, "on example #{}: expected {}, got {} (input: {})".format(i, lujvo, res, tanru))

    def test_jvozba_exceptions(self):
        for i, n in enumerate(JVOZBA_FAIL_TESTS):
            tanru, exception_type = n
            res = ""
            try:
                res = get_lujvo(tanru)
                did_exception = False
            except exception_type:
                did_exception = True
            self.assertTrue(did_exception,
                            "on example #{}: expected Exception, got {} (input: {})".format(i, res, tanru))

    def test_jvozba_brivla_score(self):
        for i, n in enumerate(LUJVO_SCORE_TESTS):
            tanru, score, cmene_score = n
            res, res_score = get_lujvo(tanru)
            self.assertEqual(score, res_score,
                             "on example #{}: expected {}, got {} (input: {} [brivla])".format(i, score, res_score, tanru))

    def test_jvozba_cmevla_score(self):
        for i, n in enumerate(LUJVO_SCORE_TESTS):
            tanru, score, cmene_score = n
            res, res_score = get_lujvo(tanru, True)
            self.assertEqual(cmene_score, res_score,
                             "on example #{}: expected {}, got {} (input: {} [cmevla])".format(i, cmene_score, res_score, tanru))

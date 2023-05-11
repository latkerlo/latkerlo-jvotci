"""
Copyright (c) 2020 uakci (https://github.com/uakci)
Licensed under the MIT License

Modified by latkerlo (https://github.com/latkerlo), Copyright (c) 2023
"""

import unittest
from jvozba_test_list import JVOZBAJVEKAhA_TESTS, CMENE_TESTS, JVOKAhA_ONLY_TESTS, JVOKAhA_FAIL_TESTS
from katna import get_veljvo


class TestKatna(unittest.TestCase):
    def test_veljvo(self):
        for i, n in enumerate(JVOZBAJVEKAhA_TESTS + CMENE_TESTS + JVOKAhA_ONLY_TESTS):
            lujvo, tanru = n
            res = get_veljvo(lujvo)
            res_tanru = " ".join(res)
            self.assertEqual(tanru, res_tanru, "on example #{}: expected {}, got {} (input: {})".format(i, tanru, res, lujvo))

    def test_veljvo_exceptions(self):
        for i, n in enumerate(JVOKAhA_FAIL_TESTS):
            lujvo, exception_type = n
            res = ""
            try:
                res = get_veljvo(lujvo)
                did_exception = False
            except exception_type:
                did_exception = True
            self.assertTrue(did_exception, "on example #{}: expected Exception, got {} (input: {})".format(i, lujvo, res))

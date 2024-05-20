"""
Copyright (c) 2020 uakci (https://github.com/uakci)
Licensed under the MIT License

Modified by latkerlo (https://github.com/latkerlo), Copyright (c) 2023-2024
"""

import unittest
import csv
from latkerlo_jvotci.katna import get_veljvo
from latkerlo_jvotci.exceptions import DecompositionError, NotBrivlaError
from latkerlo_jvotci.tarmi import SETTINGS, SettingsIterator
from test_other import check_conditions
import os


class TestKatna(unittest.TestCase):
    def test_veljvo_basic(self):
        with open(os.path.join(os.path.dirname(__file__), "../../tests/basic_test_list.csv")) as ipf:
            reader = csv.reader(ipf, delimiter="\t")
            for i, row in enumerate(reader):
                try:
                    if not row[1] or row[0][0] == "#":
                        continue
                except IndexError:
                    continue

                lujvo = row[0]
                tanru = row[1]
                try:
                    conditions = row[2]
                except IndexError:
                    conditions = None

                if conditions == "JVOZBA":
                    continue

                if tanru != "FAIL":
                    try:
                        res = get_veljvo(lujvo)
                    except Exception as e:
                        self.assertTrue(False, f"Example {i}: Expected {tanru}, got Error. Input: {[lujvo, tanru, conditions]}. Message: {e}")
                    res_string = " ".join(res)
                    self.assertEqual(tanru, res_string, f"Example {i}: Expected {tanru}, got {res}. Input: {[lujvo, tanru, conditions]}")
                else:
                    did_fail = False
                    try:
                        res = get_veljvo(lujvo)
                    except (DecompositionError, NotBrivlaError):
                        did_fail = True
                    self.assertTrue(did_fail, f"Example {i}: Expected exception, got {res}. Input: {[lujvo, tanru, conditions]}")

    def test_veljvo_my_list(self):
        with open(os.path.join(os.path.dirname(__file__), "../../tests/katna_test_list.csv")) as ipf:
            reader = csv.reader(ipf, delimiter="\t")
            for i, row in enumerate(reader):
                try:
                    if not row[1] or row[0][0] == "#":
                        continue
                except IndexError:
                    continue

                string = row[0]
                conditions = []
                for j in range(1, len(row)):
                    if j % 5 == 1:
                        conditions.append([])
                    conditions[-1].append(row[j])

                for settings in SettingsIterator(SETTINGS):
                    y_hyphen, more_rafsi, consonants, glides, mz = settings

                    tanru = None
                    for cd in conditions:
                        if check_conditions(cd[0], y_hyphen, more_rafsi, consonants, glides, mz):
                            b_type = cd[1]
                            try:
                                tanru = cd[3]
                            except IndexError:
                                pass
                            break

                    if b_type in ["LUJVO", "EXTENDED", "CMEVLA"]:
                        try:
                            res = get_veljvo(string, y_hyphen, more_rafsi, consonants, glides, mz)
                        except Exception as e:
                            self.assertTrue(False, f"Example {i}: Expected {tanru}, got Error. Input: {[string, y_hyphen, more_rafsi, consonants, glides, mz]}. Message: {e}")
                        self.assertEqual(tanru, " ".join(res), f"Example {i}: Expected {tanru}, got {res}. Input: {[string, y_hyphen, more_rafsi, consonants, glides, mz]}")
                    else:
                        res = None
                        did_fail = False
                        try:
                            res = get_veljvo(string, y_hyphen, more_rafsi, consonants, glides, mz)
                        except (DecompositionError, NotBrivlaError):
                            did_fail = True
                        self.assertTrue(did_fail, f"Example {i}: Expected exception, got {res}. Input: {b_type, [string, y_hyphen, more_rafsi, consonants, glides, mz]}")

"""
Copyright (c) 2023-2024 latkerlo (https://github.com/latkerlo)
Licensed under the MIT License
"""

import unittest
import csv
from latkerlo_jvotci.jvozba import get_lujvo, get_lujvo_with_analytics
from latkerlo_jvotci.exceptions import \
    NoLujvoFoundError, InvalidClusterError, NonLojbanCharacterError, NotZihevlaError
from latkerlo_jvotci.tarmi import LUJVO, EXTENDED_LUJVO, SETTINGS, SettingsIterator
from test_other import check_conditions
from latkerlo_jvotci.tools import analyse_brivla
import os


class TestJvozba(unittest.TestCase):
    def test_get_lujvo_basic(self):
        with open(os.path.join(os.path.dirname(__file__), "../../tests/basic_test_list.csv")) as ipf:
            reader = csv.reader(ipf, delimiter="\t")
            for i, row in enumerate(reader):
                try:
                    if (not row[0] and not row[1]) or row[0][0] == "#":
                        continue
                except IndexError:
                    continue

                lujvo = row[0]
                tanru = row[1]
                try:
                    conditions = row[2]
                except IndexError:
                    conditions = None
                do_cmevla = conditions == "CMEVLA"

                if conditions == "KATNA":
                    continue

                if lujvo != "FAIL":
                    try:
                        res = get_lujvo(tanru, generate_cmevla=do_cmevla)
                    except Exception as e:
                        self.assertTrue(False,
                                        f"Example {i}: Expected {lujvo}, got Error. Input: {[tanru, lujvo, conditions]}. Message: {e}")

                    self.assertEqual(lujvo, res,
                                     f"Example {i}: Expected {lujvo}, got {res}. Input: {[tanru, lujvo, conditions]}")
                else:
                    did_fail = False
                    try:
                        res = get_lujvo(tanru, generate_cmevla=do_cmevla)
                    except (NoLujvoFoundError, InvalidClusterError, NonLojbanCharacterError, IndexError, NotZihevlaError):
                        did_fail = True
                    self.assertTrue(did_fail,
                                    f"Example {i}: Expected exception, got {res}. Input: {[tanru, lujvo, conditions]}")

    def test_get_lujvo_my_list(self):
        with open(os.path.join(os.path.dirname(__file__), "../../tests/jvozba_test_list.csv")) as ipf:
            reader = csv.reader(ipf, delimiter="\t")
            for i, row in enumerate(reader):
                try:
                    if (not row[0] and not row[1]) or row[0][0] == "#":
                        continue
                except IndexError:
                    continue

                string = row[0]
                do_cmevla = row[1] == "C"
                conditions = []
                for j in range(2, len(row)):
                    if j % 4 == 2:
                        conditions.append([])
                    conditions[-1].append(row[j])


                for settings in SettingsIterator(SETTINGS):
                    y_hyphen, more_rafsi, consonants, glides, mz = settings

                    lujvo = None
                    for cd in conditions:
                        if check_conditions(cd[0], y_hyphen, more_rafsi, consonants, glides, mz):
                            lujvo = cd[1]

                            try:
                                score = int(cd[2])
                            except (ValueError, IndexError):
                                score = None

                            try:
                                index_list = cd[3] if len(cd[3]) > 0 else None
                            except IndexError:
                                index_list = None

                            break

                    assert lujvo is not None, f"{i}, {conditions}"
                    if lujvo == "NONE":
                        lujvo = None

                    if lujvo is not None:
                        try:
                            res = get_lujvo_with_analytics(string, generate_cmevla=do_cmevla, y_hyphens=y_hyphen, exp_rafsi_shapes=more_rafsi, consonants=consonants, glides=glides, allow_mz=mz)
                        except Exception as e:
                            self.assertTrue(False, f"Example {i}: Expected {lujvo}, got Error. Input: {[string, y_hyphen, more_rafsi, consonants, glides, mz]}. Message: {e}")

                        if not do_cmevla:
                            re_res = analyse_brivla(res[0], y_hyphens=y_hyphen, exp_rafsi_shapes=more_rafsi, consonants=consonants, glides=glides, allow_mz=mz)
                            self.assertTrue(re_res[0] in [LUJVO, EXTENDED_LUJVO], f"Example {i}: Produced non-brivla: {lujvo}. Input: {[string, y_hyphen, more_rafsi, consonants, glides, mz]}")
                        self.assertEqual(lujvo, res[0], f"Example {i}: Expected {lujvo}, got {res}. Input: {[string, y_hyphen, more_rafsi, consonants, glides, mz]}")

                        if score is not None:
                            self.assertEqual(score, res[1], f"Example {i}: Expected {score}, got {res}. Input: {[string, y_hyphen, more_rafsi, consonants, glides, mz]}")

                        if index_list is not None:
                            # Don't tell anyone I'm checking number equality with strings
                            res_ind_list = ",".join(["-".join(str(y) for y in x) for x in res[2]])
                            self.assertEqual(index_list, res_ind_list, f"Example {i}: Expected {index_list}, got {res[2]}. Input: {[string, y_hyphen, more_rafsi, consonants, glides, mz]}")

                    else:
                        did_fail = False
                        try:
                            res = get_lujvo(string, generate_cmevla=do_cmevla, y_hyphens=y_hyphen, exp_rafsi_shapes=more_rafsi, allow_mz=mz)
                        except (NoLujvoFoundError, InvalidClusterError, NonLojbanCharacterError, IndexError, NotZihevlaError):
                            did_fail = True
                        self.assertTrue(did_fail, f"Example {i}: Expected exception, got {res}. Input: {[string, y_hyphen, more_rafsi, consonants, glides, mz]}")

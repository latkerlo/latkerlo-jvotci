"""
Copyright (c) 2023-2024 latkerlo (https://github.com/latkerlo)
Licensed under the MIT License
"""

import unittest
import csv
from latkerlo_jvotci.data import HYPHENS
from latkerlo_jvotci.tools import is_gismu_or_lujvo, normalise, analyse_brivla, get_rafsi_indices
from latkerlo_jvotci.tarmi import \
    is_gismu, ALLOW_Y, FORCE_Y, TWO_CONSONANTS, ONE_CONSONANT, \
    LUJVO, EXTENDED_LUJVO, CMEVLA, SettingsIterator, SETTINGS
from latkerlo_jvotci.exceptions import NotBrivlaError
import os


def check_conditions(condition_str: str, y_hyphen: str, more_rafsi: bool, consonants: str, glides: bool, mz: bool) -> bool:
    # I don't know why it sometimes checks an empty string
    if condition_str == "ELSE" or not condition_str:
        return True
    elif condition_str == "ALLOW_Y":
        return y_hyphen == ALLOW_Y
    elif condition_str == "FORCE_Y":
        return y_hyphen == FORCE_Y
    elif condition_str == "MORE_RAF":
        return more_rafsi
    elif condition_str == "TWO_CONSONANTS":
        return consonants == TWO_CONSONANTS
    elif condition_str == "ONE_CONSONANT":
        return consonants == ONE_CONSONANT
    elif condition_str == "GLIDES":
        return glides
    elif condition_str == "YES_MZ":
        return mz

    i = 0
    if condition_str[i] == "(":
        depth = 1
        while depth > 0:
            i += 1
            if condition_str[i] == "(":
                depth += 1
            elif condition_str[i] == ")":
                depth -= 1
        left_string = condition_str[1:i]
        i += 1
        if i == len(condition_str):
            return check_conditions(left_string, y_hyphen, more_rafsi, consonants, glides, mz)

    else:
        while condition_str[i] not in "|&":
            i += 1
        left_string = condition_str[:i].strip()

    operator = condition_str[i:].strip()[0]

    right_string = condition_str[i+1:].strip()

    left_side = check_conditions(left_string, y_hyphen, more_rafsi, consonants, glides, mz)
    right_side = check_conditions(right_string, y_hyphen, more_rafsi, consonants, glides, mz)

    if operator == "|":
        return left_side or right_side
    elif operator == "&":
        return left_side and right_side
    else:
        assert operator in "|&"


def get_rafsi_string(result_list: list[str]):
    result_list = [x for x in result_list if x not in HYPHENS]
    return " ".join(result_list)


class TestOther(unittest.TestCase):
    def test_analyse_brivla_my_list(self):
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

                    b_type = None
                    decomp = None
                    for cd in conditions:
                        if check_conditions(cd[0], y_hyphen, more_rafsi, consonants, glides, mz):
                            b_type = cd[1]

                            try:
                                decomp = cd[2]
                            except IndexError:
                                pass


                            try:
                                index_list = cd[4] if len(cd[4]) > 0 else None
                            except IndexError:
                                index_list = None

                            break

                    assert b_type is not None, f"{i}, {conditions}"
                    if b_type == "EXTENDED":
                        b_type = EXTENDED_LUJVO

                    if not b_type == "NONE":
                        try:
                            res = analyse_brivla(string, y_hyphen, more_rafsi, consonants, glides, mz)
                        except Exception as e:
                            self.assertTrue(False, f"Example {i}: Expected {b_type}, got Error. Input: {[string, y_hyphen, more_rafsi, consonants, glides, mz]}. Message: {e}")
                        self.assertEqual(b_type, res[0], f"Example {i}: Expected {b_type}, got {res}. Input: {[string, y_hyphen, more_rafsi, consonants, glides, mz]}")
                        self.assertEqual(decomp, get_rafsi_string(res[1]), f"Example {i}: Expected {decomp}, got {res}. Input: {[string, y_hyphen, more_rafsi, consonants, glides, mz]}")

                        if index_list is not None:
                            res_ind_list = ",".join(["-".join(str(y) for y in x) for x in get_rafsi_indices(res[1])])
                            self.assertEqual(index_list, res_ind_list, f"Example {i}: Expected {index_list}, got {res_ind_list}. Input: {[string, y_hyphen, more_rafsi, consonants, glides, mz]}")

                    else:
                        did_fail = False
                        try:
                            res = analyse_brivla(string, y_hyphen, more_rafsi, consonants, glides, mz)
                        except NotBrivlaError:
                            did_fail = True
                        self.assertTrue(did_fail, f"Example {i}: Expected exception, got {res}. Input: {[string, y_hyphen, more_rafsi, consonants, glides, mz]}")

    def test_analyse_brivla_dictionary(self):
        import json
        with open(os.path.join(os.path.dirname(__file__), "../../tests/jvs_words.json")) as ipf:
            all_words = json.load(ipf)

        def match_type(from_list, result_type):
            if from_list == "LUJVO":
                return result_type in [LUJVO, EXTENDED_LUJVO]
            else:
                return from_list == result_type

        for b_type, word_list in all_words.items():
            for word in word_list:
                if word == "posytmo":
                    continue  # >:3
                if word == "gudjrati":
                    continue  # This is recent. How did phma add this?

                if b_type != "OTHER":
                    try:
                        res = analyse_brivla(word)
                    except Exception:
                        self.assertTrue(False, f"Expected {b_type}, got Error (input: {word})")
                    self.assertTrue(match_type(b_type, res[0]), f"Expected {b_type}, got {res[1]} (input: {word})")
                else:
                    did_fail = False
                    try:
                        res = analyse_brivla(word)
                    except NotBrivlaError:
                        did_fail = True

                    self.assertTrue(did_fail or res[0] == CMEVLA, f"Expected exception, got {res}. (input: {word})")

    def test_is_gismu_dictionary(self):
        import json
        with open(os.path.join(os.path.dirname(__file__), "../../tests/jvs_words.json")) as ipf:
            all_words = json.load(ipf)
        for b_type, word_list in all_words.items():
            for word in word_list:
                word = normalise(word)

                self.assertEqual(b_type == "GISMU", is_gismu(word), f"(input: {word})")
                if b_type == "GISMU":
                    self.assertTrue(is_gismu_or_lujvo(word))

    # def test_check(self):
    #     print(check_conditions("(ALLOW_Y | FORCE_Y) & YES_MZ", "ALLOW_Y", False, True))

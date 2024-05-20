"""
Copyright (c) 2020 uakci (https://github.com/uakci)
Licensed under the MIT License

Modified by latkerlo (https://github.com/latkerlo), Copyright (c) 2023-2024
"""

from latkerlo_jvotci.data import *
from latkerlo_jvotci.exceptions import DecompositionError
import re

# the tarmi enum directly corresponds to the scores for R
HYPHEN = 0
CVCCV = 1
CVCC = 2
CCVCV = 3
CCVC = 4
CVC = 5
CVhV = 6
CCV = 7
CVV = 8
OTHER_RAFSI = 9  # except later we actually replace this with 0

SONORANT_CONSONANTS = "lmnr"

GISMU = "GISMU"
ZIhEVLA = "ZIhEVLA"
LUJVO = "LUJVO"
EXTENDED_LUJVO = "EXTENDED_LUJVO"
RAFSI = "RAFSI"
CMEVLA = "CMEVLA"

STANDARD = "STANDARD"
ALLOW_Y = "ALLOW_Y"
FORCE_Y = "FORCE_Y"

CLUSTER = "CLUSTER"
TWO_CONSONANTS = "TWO_CONSONANTS"
ONE_CONSONANT = "ONE_CONSONANT"

SETTINGS = [
    [STANDARD, ALLOW_Y, FORCE_Y],  # y-hyphens
    [False, True],  # experimental rafsi shapes
    [CLUSTER, TWO_CONSONANTS, ONE_CONSONANT],  # consonants
    [False, True],  # glides are consonants
    [False, True]  # mz is valid cluster
]


class SettingsIterator:
    def __init__(self, settings):
        self.settings = settings
        self.index = 0
        self.possibilities = 1
        for setting in settings:
            self.possibilities *= len(setting)

    def __iter__(self):
        return self

    def __next__(self):
        if self.index < self.possibilities:
            item = [None] * len(self.settings)
            base = 1
            for i, setting in enumerate(self.settings):
                item[i] = setting[(self.index // base) % len(setting)]
                base *= len(setting)

            self.index += 1
            return item
        else:
            raise StopIteration


def is_vowel(character: str) -> bool:
    """
    Return True if character is a vowel (aeiou).

    :param character: Some character.
    :return: True if it is a vowel.
    """
    return character in "aeiou"


def is_consonant(character: str) -> bool:
    """
    Return True if character is a consonant (bcdfgjklmnprstvxz).

    :param character: Some character.
    :return: True if it is a consonant.
    """
    return character in "bcdfgjklmnprstvxz"


def is_glide(string: str) -> bool:
    """
    Return true if string starts with an on-glide.

    :param string: String to check.
    :return: True if string starts with an on-glide.
    """
    if len(string) < 2:
        return False
    return string[0] in "iu" and is_vowel(string[1])


def is_only_lojban_characters(string: str) -> bool:
    """
    Return true if string is only lojban characters except y.

    :param string: Some string.
    :return: True if it contains only lojban characters except y.
    """
    return re.fullmatch(r"[aeioubcdfgjklmnprstvxz']+", string) is not None


def contains_consonant(string: str) -> bool:
    """
    Return true if at least one character is a lojban consonant.

    :param string: Some string.
    :return: True if it contains a lojban consonant.
    """
    for character in string:
        if is_consonant(character):
            return True
    return False


def is_gismu_shape(valsi: str) -> bool:
    """
    Return true if valsi is shaped like CVCCV or CCVCV.
    Does NOT have to be a valid gismu.

    :param valsi: A word to check.
    :return: True if valsi is gismu-shaped.
    """
    if not (len(valsi) == 5 and is_consonant(valsi[0]) and is_consonant(valsi[3]) and is_vowel(valsi[4])):
        return False
    if is_vowel(valsi[1]) and is_consonant(valsi[2]):
        return True
    elif is_consonant(valsi[1]) and is_vowel(valsi[2]):
        return True
    else:
        return False


def is_gismu(valsi: str, allow_mz: bool = False) -> bool:
    """
    Check if valsi is a valid gismu.

    :param valsi: Some word.
    :param allow_mz: True if mz is a valid consonant cluster.
    :return: True if valid is a valid gismu.
    """
    if not is_gismu_shape(valsi):
        return False
    if is_vowel(valsi[1]):
        return valsi[2:4] in (MZ_VALID if allow_mz else VALID)
    else:
        return valsi[:2] in INITIAL


def split_vowel_cluster(vowels: str) -> list[str]:
    """
    Split vowel cluster into list of syllables.

    :param vowels: A string of vowels.
    :return: List of syllables in cluster.
    """
    def add_to_result(new_cluster):
        new_vowels = vowels[:-len(new_cluster)]
        if new_cluster[0] == "i" and new_vowels[-2:] in ["ai", "ei", "oi"]:
            raise DecompositionError(f"Couldn't decompose: {{{vowels_copy}}}")
        elif new_cluster[0] == "u" and new_vowels[-2:] == "au":
            raise DecompositionError(f"Couldn't decompose: {{{vowels_copy}}}")

        result.insert(0, new_cluster)

    vowels_copy = vowels
    result = []
    while True:
        if len(vowels) > 3 and vowels[-3:] in FOLLOW_VOWEL_CLUSTERS:
            add_to_result(vowels[-3:])
            vowels = vowels[:-3]
        elif len(vowels) > 2 and vowels[-2:] in FOLLOW_VOWEL_CLUSTERS:
            add_to_result(vowels[-2:])
            vowels = vowels[:-2]
        elif vowels in START_VOWEL_CLUSTERS:
            result.insert(0, vowels)
            return result
        else:
            raise DecompositionError(f"Couldn't decompose: {{{vowels_copy}}}")


def is_zihevla_initial_cluster(cluster: str) -> bool:
    """
    Check if consonant cluster can start a zi'evla.

    :param cluster: A consonant cluster.
    :return: True if valid beginning for zi'evla.
    """
    if len(cluster) > 3:
        return False
    elif len(cluster) == 3:
        if cluster[:2] not in INITIAL or cluster[1:] not in ZIhEVLA_INITIAL:
            return False
    elif len(cluster) == 2:
        if cluster not in INITIAL:
            return False
    return True


def is_zihevla_middle_cluster(cluster: str) -> bool:
    """
    Check if consonant cluster can be in a zi'evla.

    :param cluster: A consonant cluster.
    :return: True if valid in zi'evla.
    """
    if len(cluster) == 3:
        if cluster[1] in SONORANT_CONSONANTS:
            return True
        return cluster[:2] in VALID and cluster[1:] in INITIAL
    elif len(cluster) < 3:
        return True  # Pairs are already checked outside this function

    # I tried to be smart, but it was too hard, so I used regex (badly)
    if cluster[-2] == "m" and cluster[-2:] in INITIAL:
        if is_zihevla_initial_cluster(cluster[-3:]):
            match = re.fullmatch(r"([bcdfgjklmnprstvxz])?((?:[bcdfgjklmnprstvxz][lmnr])*)?", cluster[:-3])
        else:
            match = re.fullmatch(r"([bcdfgjklmnprstvxz])?((?:[bcdfgjklmnprstvxz][lmnr])*)?", cluster[:-2])
    else:
        match = re.fullmatch(r"([bcdfgjklmnprstvxz])?((?:[bcdfgjklmnprstvxz][lmnr])*)"
                             r"(?:([bcdfgjkpstvxz][bcdfgjklmnprstvxz]?[lmnr]?)|([bcdfgjklmnprstvxz]))",
                             cluster)

    if match is None:
        return False

    # Last part needs to be a cluster that could start a zi'evla
    # iln(sp)i -> YES
    # iln(kp)i -> NO
    # iln(skr)i -> YES
    # iln(tkr)i -> NO
    if match.groups()[-2] is not None and not is_zihevla_initial_cluster(match.groups()[-2]):
        return False

    return True


def is_valid_rafsi(rafsi: str, allow_mz: bool = False) -> bool:
    """
    Check if string is a valid CLL rafsi.

    :param rafsi: A string.
    :param allow_mz: True if mz is a valid consonant cluster.
    :return: True if string is a valid CLL rafsi.
    """
    raftai = rafsi_tarmi(rafsi)
    if raftai in [CVCCV, CVCC]:
        return rafsi[2:4] in (MZ_VALID if allow_mz else VALID)
    if raftai in [CCVCV, CCVC, CCV]:
        return rafsi[:2] in INITIAL
    return 1 <= raftai <= 8


def rafsi_tarmi(rafsi: str) -> int:
    """
    Get the shape of a rafsi.

    :param rafsi: A rafsi.
    :return: The rasfi's shape (an int enum).
    """
    raf_len = len(rafsi)
    if raf_len == 0:
        return OTHER_RAFSI
    elif raf_len == 2 and rafsi[0] == "'" and rafsi[1] == 'y':
        return HYPHEN
    elif not is_consonant(rafsi[0]) and raf_len != 1:
        return OTHER_RAFSI
    if raf_len == 1:
        if is_vowel(rafsi):
            return OTHER_RAFSI
        else:
            return HYPHEN
    elif raf_len == 3:
        if not is_vowel(rafsi[2]):
            if is_vowel(rafsi[1]) and is_consonant(rafsi[2]):
                return CVC
        else:
            if is_vowel(rafsi[1]):
                return CVV
            elif is_consonant(rafsi[1]):
                return CCV
    elif raf_len == 4:
        if is_vowel(rafsi[1]):
            if is_vowel(rafsi[3]):
                if rafsi[2] in '\'':
                    return CVhV
            elif is_consonant(rafsi[2]) and is_consonant(rafsi[3]):
                return CVCC
        elif is_consonant(rafsi[1]) and is_vowel(rafsi[2]) and is_consonant(rafsi[3]):
            return CCVC
    elif raf_len == 5:
        if is_gismu_shape(rafsi):
            if is_vowel(rafsi[2]):
                return CCVCV
            else:
                return CVCCV
    return OTHER_RAFSI


def strip_hyphens(rafsi: str) -> str:
    """
    Get the rafsi without any initial or final hyphen characters.

    :param rafsi: A rafsi.
    :return: The rafsi without hyphens.
    """
    while rafsi[0] in "'y":
        rafsi = rafsi[1:]
    while rafsi[-1] in "'y":
        rafsi = rafsi[:-1]
    return rafsi


def tarmi_ignoring_hyphen(rafsi: str) -> int:
    """
    Get the rafsi's shape, removing a final hyphen if necessary.

    :param rafsi: A rafsi.
    :return: The rasfi's shape (an int enum).
    """
    return rafsi_tarmi(strip_hyphens(rafsi))

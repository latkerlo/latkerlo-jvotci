"""
Copyright (c) 2020 uakci (https://github.com/uakci)
Licensed under the MIT License

Modified by latkerlo (https://github.com/latkerlo), Copyright (c) 2023
"""

from data import VALID, INITIAL
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
FUHIVLA = 9


def is_vowel(character: str) -> bool:
    return character in "aeiou"


def is_consonant(character: str) -> bool:
    return character in "bcdfgjklmnprstvxz"


def is_only_lojban_characters(valsi: str) -> bool:
    return re.fullmatch(r"[aeioubcdfgjklmnprstvxz']+", valsi) is not None


def is_gismu(valsi: str) -> bool:
    return len(valsi) == 5 and is_consonant(valsi[0]) and is_consonant(valsi[3]) and \
           is_vowel(valsi[4]) and ((is_vowel(valsi[1]) and is_consonant(valsi[2])) or
                                   (is_consonant(valsi[1]) and is_vowel(valsi[2])))


def is_valid_rafsi(rafsi: str) -> bool:
    raftai = rafsi_tarmi(rafsi)
    if raftai in [CVCCV, CVCC]:
        return rafsi[2:4] in VALID
    if raftai in [CCVCV, CCVC, CCV]:
        return rafsi[:2] in INITIAL
    return 1 <= raftai <= 8


def rafsi_tarmi(rafsi: str) -> int:
    raf_len = len(rafsi)
    if raf_len == 0:
        return FUHIVLA
    elif raf_len == 2 and rafsi[0] == "'" and rafsi[1] == 'y':
        return HYPHEN
    elif not is_consonant(rafsi[0]) and raf_len != 1:
        return FUHIVLA
    if raf_len == 1:
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
        if is_gismu(rafsi):
            if is_vowel(rafsi[2]):
                return CCVCV
            else:
                return CVCCV
    return FUHIVLA


def tarmi_ignoring_hyphen(rafsi: str) -> int:
    if rafsi[-1] == "y":
        rafsi = rafsi[:-1]
    return rafsi_tarmi(rafsi)

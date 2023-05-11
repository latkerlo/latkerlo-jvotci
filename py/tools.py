"""
Copyright (c) 2023 latkerlo (https://github.com/latkerlo)
Licensed under the MIT License
"""

import re
from katna import jvokaha
from tarmi import is_gismu, is_consonant
from lujvo_exceptions import NonLojbanCharacterError, DecompositionError, InvalidClusterError


def is_gismu_or_lujvo(a_string: str) -> bool:
    if is_gismu(a_string):
        return True

    try:
        jvokaha(a_string)
        return True
    except (DecompositionError, InvalidClusterError):
        return False


# Assumes the string does need to be split and the remainder will be valid lojban
def split_one_cmavo(a_string: str) -> list[str]:
    i = 0
    will_end = False
    while i < len(a_string):
        if a_string[i:i+2] in ["ai", "ei", "oi", "au"] and i + 2 < len(a_string) and a_string[i+2] not in "aeiouy":
            i += 2
            will_end = True
        elif a_string[i] in "iu" and i + 1 < len(a_string) and a_string[i+1:i+2] in "aeiouy":
            if will_end:
                break
            i += 2
            will_end = True
        elif a_string[i] in "aeiouy":
            i += 1
            will_end = True
        elif a_string[i] == "'":
            i += 1
            will_end = False
        elif a_string[i] in "bcdfgjklmnprstvxz":
            if i == 0:
                i += 1
                continue
            else:
                break
        else:
            raise NonLojbanCharacterError(f"Non-lojban character {{{a_string[i]}}} in {{{a_string}}} at index {{{i}}}")
    return [a_string[:i], a_string[i:]]


# Assumes valid lojban
def split_words(a_string: str) -> list[str]:
    if len(a_string) == 0:
        return []

    if is_consonant(a_string[-1]):
        return [a_string]

    first_five = re.sub(r"[y']", '', a_string)[:5]
    consonant_cluster = re.search(r"[bcdfgjklmnprstvxz][bcdfgjklmnprstvxz]", first_five)

    if consonant_cluster is None:
        cmavo, remainder = split_one_cmavo(a_string)
        return [cmavo] + split_words(remainder)

    if is_gismu_or_lujvo(a_string):
        return [a_string]

    i = 0
    for char in a_string:
        if i >= consonant_cluster.start():
            break

        if char not in ['y', "'"]:
            i += 1

    if is_gismu_or_lujvo(a_string[i:]):
        return [a_string[:i], a_string[i:]]
    else:
        return [a_string]

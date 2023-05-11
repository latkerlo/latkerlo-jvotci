"""
Copyright (c) 2021 sozysozbot (https://github.com/sozysozbot)
Licensed under the MIT License

Modified by latkerlo (https://github.com/latkerlo), Copyright (c) 2023
"""

from tarmi import *
from jvozba import get_lujvo_2
from lujvo_exceptions import DecompositionError, InvalidClusterError
from rafsi import RAFSI


def search_selrafsi_from_rafsi(rafsi: str) -> str:
    if len(rafsi) == 5 and rafsi in RAFSI:
        return rafsi  # 5-letter rafsi

    if rafsi != "brod" and len(rafsi) == 4 and "'" not in rafsi:  # 4-letter rafsi
        for u in range(5):
            gismu_candid = rafsi + "aeiou"[u]
            if gismu_candid in RAFSI:
                return gismu_candid
    for valsi, rafsi_list in RAFSI.items():
        if rafsi in rafsi_list:
            return valsi


# jvokaha("fu'ivla") --> ["fu'i", "vla"]
# jvokaha("fu'irvla") --> error // because r-hyphen is unnecessary
# jvokaha("pasymabru") --> ["pas", "y", "mabru"]
# jvokaha("pasmabru") --> error // because {pasmabru} is actually {pa smabru}
def jvokaha(lujvo: str) -> list[str]:
    arr = jvokaha_2(lujvo)

    rafsi_tanru = [f'-{x}-' for x in arr if len(x) > 1]
    correct_lujvo = get_lujvo_2(rafsi_tanru, generate_cmene=is_consonant(arr[-1][-1]))[0]

    if lujvo == correct_lujvo:
        return arr
    else:
        raise DecompositionError("malformed lujvo {" + lujvo +
                                 "}; it should be {" + correct_lujvo + "}")


# jvokaha2("fu'ivla") --> ["fu'i", "vla"]
# jvokaha2("fu'irvla") --> ["fu'i", "r", "vla"]
# jvokaha2("pasymabru") --> ["pas", "y", "mabru"]
# jvokaha2("pasmabru") --> ["pas", "mabru"]
def jvokaha_2(lujvo: str) -> list[str]:
    original_lujvo = lujvo
    res = []
    while True:
        if lujvo == "":
            return res

        # remove hyphen
        if len(res) > 0 and len(res[-1]) != 1:  # hyphen cannot begin a word; nor can two hyphens
            if (
                lujvo[0] == "y"  # y-hyphen
                or lujvo[:2] == "nr"  # n-hyphen is only allowed before r
                or lujvo[0] == "r" and is_consonant(lujvo[1])  # r followed by a consonant
            ):
                res.append(lujvo[0])
                lujvo = lujvo[1:]
                continue

        # drop rafsi from front

        # CVV can always be dropped
        if rafsi_tarmi(lujvo[:3]) == CVV and lujvo[1:3] in ["ai", "ei", "oi", "au"]:
            res.append(lujvo[:3])
            lujvo = lujvo[3:]
            continue

        # CV'V can always be dropped
        if rafsi_tarmi(lujvo[:4]) == CVhV:
            res.append(lujvo[:4])
            lujvo = lujvo[4:]
            continue

        # CVCCY and CCVCY can always be dropped
        if rafsi_tarmi(lujvo[:4]) in [CVCC, CCVC]:
            if is_vowel(lujvo[1]):
                if lujvo[2:4] not in VALID:
                    raise InvalidClusterError(f"Invalid cluster {{{lujvo[2:4]}}} in {{{original_lujvo}}}")
            else:
                if lujvo[0:2] not in INITIAL:
                    raise InvalidClusterError(f"Invalid initial cluster {{{lujvo[0:2]}}} in {{{original_lujvo}}}")

            if len(lujvo) == 4 or lujvo[4] == "y":
                res.append(lujvo[:4])
                try:
                    if lujvo[4] == "y":
                        res.append("y")
                except IndexError:
                    pass
                lujvo = lujvo[5:]
                continue

        # the final rafsi can be 5-letter
        if rafsi_tarmi(lujvo) in [CVCCV, CCVCV]:
            res.append(lujvo)
            return res

        if rafsi_tarmi(lujvo[:3]) in [CVC, CCV]:
            # TODO: Why is a test for valid initial not needed here?
            res.append(lujvo[:3])
            lujvo = lujvo[3:]
            continue

        # if all fails...
        # print(res, lujvo)
        raise DecompositionError("Failed to decompose {" + original_lujvo + "}")


def get_veljvo(lujvo: str) -> list[str]:
    rafsi_list = [x for x in jvokaha(lujvo) if len(x) > 1]
    selrafsi_list = [search_selrafsi_from_rafsi(x) for x in rafsi_list]
    for i, selrafsi in enumerate(selrafsi_list):
        rafsi_list[i] = selrafsi if selrafsi is not None else f'-{rafsi_list[i]}-'
    return rafsi_list

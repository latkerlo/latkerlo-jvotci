"""
Copyright (c) 2021 sozysozbot (https://github.com/sozysozbot)
Licensed under the MIT License

Modified by latkerlo (https://github.com/latkerlo), Copyright (c) 2023-2024
"""

from latkerlo_jvotci.tarmi import *
from latkerlo_jvotci.tools import is_brivla, analyse_brivla
from latkerlo_jvotci.jvozba import get_lujvo_from_list
from latkerlo_jvotci.exceptions import DecompositionError, InvalidClusterError, NotBrivlaError, NoLujvoFoundError
from latkerlo_jvotci.rafsi import RAFSI_LIST


def search_selrafsi_from_rafsi(rafsi: str) -> str:
    """
    Return the selrafsi for a given rafsi, if one exists.
    Otherwise, None is returned.

    :param rafsi: The rafsi to search for.
    :return: The corresponding selrafsi, if applicable, otherwise None.
    """
    if rafsi != "brod" and len(rafsi) == 4 and "'" not in rafsi:  # 4-letter rafsi
        for u in range(5):
            gismu_candid = rafsi + "aeiou"[u]
            if gismu_candid in RAFSI_LIST:
                return gismu_candid
    for valsi, rafsi_list in RAFSI_LIST.items():
        if rafsi in rafsi_list:
            return valsi


def selrafsi_list_from_rafsi_list(rafsi_list: list[str], allow_mz=False) -> list[str]:
    """
    Create a list of selrafsi and formatted rafsi from a list of rafsi.

    Example:
    ["lat", "mot", "kelr", "y", "kerlo"] ->
    ["mlatu", "-mot-", "kelr-", "kerlo"]

    :param rafsi_list: List of rafsi and hyphens (a decomposed word).
    :param allow_mz: True if mz is a valid consonant cluster.
    :return: List of selrafsi and formatted rafsi.
    """
    result = [("" if x in HYPHENS else x) for x in rafsi_list]
    selrafsi_list = [search_selrafsi_from_rafsi(x) for x in result]
    for i, selrafsi in enumerate(selrafsi_list):
        if not result[i]:
            continue

        if selrafsi is not None:
            result[i] = selrafsi
        elif i < len(rafsi_list) - 2 and rafsi_list[i+1][0] == "y" and is_brivla(result[i] + "a", allow_mz=allow_mz):
            result[i] = f'{result[i]}-'
        elif is_brivla(result[i], allow_mz=allow_mz):
            pass
        elif i == len(rafsi_list) - 1 and is_brivla(result[i] + "a", allow_mz=allow_mz):
            result[i] = f'{result[i]}-'
        else:
            result[i] = f'-{result[i]}-'
    return [x for x in result if len(x) > 0]


# Assumes... something? TODO
def compare_lujvo_pieces(corr: [str], other: [str]) -> bool:
    """
    Check if corr and other represent the same lujvo.
    other may have unnecessary hyphens.

    :param corr: A list of parts of the correct lujvo.
    :param other: A list of parts of a candidate to test.
    :return: True if the lujvo are the same except for hyphens.
    """
    i = 0
    for j, part in enumerate(corr):
        if part == other[i]:
            i += 1
            continue

        if (
            0 < i < len(other) - 1 and
            other[i] in ["r", "n"] and
            rafsi_tarmi(other[i-1]) in [CVV, CVhV] and
            (i > 1 or rafsi_tarmi(other[i+1]) in [CCVCV, CCVC, CCV])
        ):
            i += 1

        if part == other[i]:
            i += 1
        else:
            return False
    return i == len(other)


# jvokaha("fu'ivla") --> ["fu'i", "vla"]
# jvokaha("fu'irvla") --> error // because r-hyphen is unnecessary
# jvokaha("pasymabru") --> ["pas", "y", "mabru"]
# jvokaha("pasmabru") --> error // because {pasmabru} is actually {pa smabru}
def jvokaha(
        lujvo: str,
        allow_rn_hyphens: bool = False,
        y_hyphens: str = STANDARD,
        consonants: str = CLUSTER,
        glides: bool = False,
        allow_mz: bool = False) \
        -> list[str]:
    """
    Decompose lujvo to get a list of pieces (rafsi and hyphens). Raise
    an error if the lujvo is not well-formed.

    :param lujvo: A lujvo to decompose.
    :param allow_rn_hyphens: True if unnecessary r & n hyphens are
    allowed.
    :param y_hyphens: Which y-hyphen rules to use.
    :param consonants: Which consonant rules to use.
    :param glides: True if glides count as consonants.
    :param allow_mz: True if mz is a valid consonant cluster.
    :return: List of lujvo pieces (rafsi and hyphens).
    """
    arr = jvokaha_2(lujvo, y_hyphens=y_hyphens, allow_mz=allow_mz)

    rafsi_tanru = [f'-{x}-' for x in arr if len(x) > 2]
    try:
        correct_lujvo = get_lujvo_from_list(
            rafsi_tanru,
            generate_cmevla=is_consonant(arr[-1][-1]),
            y_hyphens=y_hyphens,
            consonants=consonants,
            glides=glides,
            allow_mz=allow_mz)[0]
    except NoLujvoFoundError:
        raise DecompositionError(f"no lujvo for {rafsi_tanru}")

    if allow_rn_hyphens and not y_hyphens == FORCE_Y:
        cool_and_good = compare_lujvo_pieces(jvokaha_2(correct_lujvo, allow_mz=allow_mz), arr)
    else:
        cool_and_good = correct_lujvo == lujvo

    if cool_and_good:
        return arr
    else:
        raise DecompositionError("malformed lujvo {" + lujvo +
                                 "}; it should be {" + correct_lujvo + "}")


# jvokaha2("fu'ivla") --> ["fu'i", "vla"]
# jvokaha2("fu'irvla") --> ["fu'i", "r", "vla"]
# jvokaha2("pasymabru") --> ["pas", "y", "mabru"]
# jvokaha2("pasmabru") --> ["pas", "mabru"]
def jvokaha_2(
        lujvo: str,
        y_hyphens: str = STANDARD,
        allow_mz: bool = False) \
        -> list[str]:
    """
    Decompose lujvo to get a list of pieces (rafsi and hyphens).
    Raises an error if the string is not decomposable, but not if it
    is invalid for other reasons.

    :param lujvo: A lujvo to decompose.
    :param y_hyphens: Which y-hyphen rules to use.
    :param allow_mz: True if mz is a valid consonant cluster.
    :return: List of lujvo pieces (rafsi and hyphens).
    """
    original_lujvo = lujvo
    res = []
    while True:
        if lujvo == "":
            return res

        # remove hyphen
        if len(res) > 0 and len(res[-1]) != 1:  # hyphen cannot begin a word; nor can two hyphens
            if lujvo[0] == "y":  # y-hyphen
                res.append(lujvo[0])
                lujvo = lujvo[1:]
                continue
            elif y_hyphens != FORCE_Y and (
                lujvo[:2] == "nr"  # n-hyphen is only allowed before r
                or lujvo[0] == "r" and is_consonant(lujvo[1])  # r followed by a consonant
            ):
                res.append(lujvo[0])
                lujvo = lujvo[1:]
                continue
            elif y_hyphens != STANDARD and lujvo[:2] == "'y":
                res.append(lujvo[:2])
                lujvo = lujvo[2:]
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
                if lujvo[2:4] not in (MZ_VALID if allow_mz else VALID):
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

        if rafsi_tarmi(lujvo[:3]) == CVC:
            res.append(lujvo[:3])
            lujvo = lujvo[3:]
            continue

        if rafsi_tarmi(lujvo[:3]) == CCV:
            if lujvo[0:2] not in INITIAL:
                raise InvalidClusterError(f"Invalid initial cluster {{{lujvo[0:2]}}} in {{{original_lujvo}}}")
            res.append(lujvo[:3])
            lujvo = lujvo[3:]
            continue

        # if all fails...
        raise DecompositionError("Failed to decompose {" + original_lujvo + "}")


def get_veljvo(
        lujvo: str,
        y_hyphens: str = STANDARD,
        exp_rafsi_shapes: bool = False,
        consonants: str = CLUSTER,
        glides: bool = False,
        allow_mz: bool = False) \
        -> list[str]:
    """
    Decompose a lujvo into a list of selrafsi and formatted rafsi.

    :param lujvo: Lujvo to decompose.
    :param y_hyphens: Which y-hyphen rules to use.
    :param exp_rafsi_shapes: True if experimental rafsi shapes are allowed.
    :param consonants: Which consonant rules to use.
    :param glides: True if glides count as consonants.
    :param allow_mz: True if mz is a valid consonant cluster.
    :return: List of selrafsi and rafsi.
    """
    b_type, rafsi_list = analyse_brivla(
        lujvo,
        y_hyphens=y_hyphens,
        exp_rafsi_shapes=exp_rafsi_shapes,
        consonants=consonants,
        glides=glides,
        allow_mz=allow_mz
    )

    if b_type not in [LUJVO, EXTENDED_LUJVO, CMEVLA]:
        raise DecompositionError(f"Valsi is of type {b_type}")
    return selrafsi_list_from_rafsi_list(rafsi_list, allow_mz)


if __name__ == "__main__":
    # Terrible CLI for quick testing. Use at own risk.
    y_setting = STANDARD
    raf_setting = False
    con_setting = CLUSTER
    glide_setting = False
    mz_setting = False
    while True:
        string = input(f"({y_setting[0]}{str(raf_setting)[0]}{con_setting[0]}{str(glide_setting)[0]}{str(mz_setting)[0]}) Enter a lujvo: ")
        if not string:
            break
        if string[0] == "/":
            for change in string[1:].lower().split():
                match change:
                    case "standard":
                        y_setting = STANDARD
                    case "allow":
                        y_setting = ALLOW_Y
                    case "force":
                        y_setting = FORCE_Y
                    case "more":
                        raf_setting = True
                    case "less":
                        raf_setting = False
                    case "cluster":
                        con_setting = CLUSTER
                    case "2c":
                        con_setting = TWO_CONSONANTS
                    case "1c":
                        con_setting = ONE_CONSONANT
                    case "glide":
                        glide_setting = True
                    case "glidnt":
                        glide_setting = False
                    case "mz":
                        mz_setting = True
                    case "mznt":
                        mz_setting = False
                    case _:
                        continue
            continue
        try:
            print(analyse_brivla(string, y_hyphens=y_setting, exp_rafsi_shapes=raf_setting, consonants=con_setting, glides=glide_setting, allow_mz=mz_setting))
            print(get_veljvo(string, y_hyphens=y_setting, exp_rafsi_shapes=raf_setting, consonants=con_setting, glides=glide_setting, allow_mz=mz_setting))
        except (DecompositionError, NotBrivlaError) as e:
            print(f"Error: {e}")

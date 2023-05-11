"""
Copyright (c) 2023 Antonia Brown (https://codeberg.org/tb148)
Licensed under the Apache License, Version 2.0

Modified by latkerlo (https://github.com/latkerlo), Copyright (c) 2023
"""

from tarmi import *
from rafsi import RAFSI
from lujvo_exceptions import NoLujvoFoundError, InvalidClusterError, NonLojbanCharacterError


def score(rafsi: str) -> int:
    return (
        1000 * len(rafsi)
        - 500 * rafsi.count("'")
        + 100 * rafsi.count("y")
        - 10 * tarmi_ignoring_hyphen(rafsi)
        - (
            rafsi.count("a")
            + rafsi.count("e")
            + rafsi.count("i")
            + rafsi.count("o")
            + rafsi.count("u")
        )
    )


def process_tanru(tanru: str | list[str]) -> list[str]:
    from tools import split_words

    if isinstance(tanru, str):
        valsi_list = tanru.split()
    elif isinstance(tanru, list):
        valsi_list = tanru
    else:
        raise TypeError(f"Cannot make lujvo from type {type(tanru)}")

    expanded_valsi_list: list[str] = []
    for valsi in valsi_list:
        expanded_valsi_list += split_words(valsi) if "-" not in valsi else [valsi]
    return expanded_valsi_list


def get_rafsi_list_list(valsi_list: list[str], generate_cmene=False) -> list[list[str]]:
    list_of_rafsi_lists: list[list[str]] = []
    for i, valsi in enumerate(valsi_list):
        rafsi_list: list[str] = []
        if valsi[-1] == "-":
            valsi = valsi.strip('-')
            if not is_only_lojban_characters(valsi):
                raise NonLojbanCharacterError(f"Non-lojban character in {{{valsi}}}")
            if not is_valid_rafsi(valsi):
                raise InvalidClusterError(f"Invalid cluster in {{{valsi}}}")
            if is_gismu(valsi) and i != len(valsi_list) - 1:
                raise NoLujvoFoundError(f"Non-final 5-letter rafsi: {{{valsi}}}")

            if rafsi_tarmi(valsi) in [CCVC, CVCC]:
                if generate_cmene and i == len(valsi_list) - 1:
                    rafsi_list.append(valsi)
            else:
                rafsi_list.append(valsi)
            if is_consonant(valsi[-1]) and not (generate_cmene and i == len(valsi_list) - 1):
                rafsi_list.append(valsi + "y")

        else:
            if not is_only_lojban_characters(valsi):
                raise NonLojbanCharacterError(f"Non-lojban character in {{{valsi}}}")

            try:
                for short_rafsi in RAFSI[valsi]:
                    rafsi_list.append(short_rafsi)
                    if is_consonant(short_rafsi[-1]):
                        rafsi_list.append(short_rafsi + "y")
            except KeyError:
                pass

            if is_gismu(valsi):
                if not is_valid_rafsi(valsi):
                    raise InvalidClusterError(f"Invalid cluster in {{{valsi}}}")
                if i == len(valsi_list) - 1:
                    if generate_cmene:
                        rafsi_list.append(valsi[:-1])
                    else:
                        rafsi_list.append(valsi)
                else:
                    rafsi_list.append(valsi[:-1] + "y")

        list_of_rafsi_lists.append(rafsi_list)
    return list_of_rafsi_lists


def combine(
        lujvo: str,
        rafsi: str,
        lujvo_score: int,
        is_tosmabru: int,
        generate_cmene: bool,
        tanru_len=0) \
        -> tuple[int, int, str] | None:
    if is_consonant(lujvo[-1]) and is_consonant(rafsi[0]) \
            and lujvo[-1] + rafsi[0] not in VALID:
        return
    if lujvo[-1] + rafsi[:2] in ["ndj", "ndz", "ntc", "nts"]:
        return
    
    raftai_1 = tarmi_ignoring_hyphen(rafsi)
    hyphen = ""
    if len(lujvo) <= 5 and not generate_cmene:
        raftai_0 = tarmi_ignoring_hyphen(lujvo)
        if raftai_0 in [CVhV, CVV]:
            if rafsi[0] == "r":
                hyphen = "n"
            else:
                hyphen = "r"
        if tanru_len == 2 and raftai_1 == CCV:
            hyphen = ""

    if is_tosmabru:
        if lujvo[-1] + rafsi[0] not in INITIAL:
            is_tosmabru = 0
        elif raftai_1 == CVCCV:
            if rafsi[2:4] in INITIAL:
                return
            is_tosmabru = 0
        elif raftai_1 == CVC:
            if rafsi[-1] == "y":
                return
        else:
            is_tosmabru = 0

    return (
        is_tosmabru,
        lujvo_score + 1100 * len(hyphen) + score(rafsi),
        lujvo + hyphen + rafsi,
    )


def update_current_best(candidate, current_best):
    if candidate is None:
        return
    is_tosmabru, res_score, res_lujvo = candidate
    if (
        res_lujvo[-1] not in current_best[is_tosmabru]
        or current_best[is_tosmabru][res_lujvo[-1]][1] > res_score
    ):
        current_best[is_tosmabru][res_lujvo[-1]] = res_lujvo, res_score


def get_lujvo(tanru, generate_cmene=False):
    return get_lujvo_2(process_tanru(tanru), generate_cmene)


def get_lujvo_2(valsi_list, generate_cmene=False):
    rafsi_list_list = get_rafsi_list_list(valsi_list, generate_cmene)
    current_best = [{}, {}]
    for rafsi0 in rafsi_list_list[0]:
        for rafsi1 in rafsi_list_list[1]:
            result = combine(
                rafsi0,
                rafsi1,
                score(rafsi0),
                tarmi_ignoring_hyphen(rafsi0) == CVC and rafsi0[-1] != "y" and not generate_cmene,
                generate_cmene,
                tanru_len=len(rafsi_list_list)
            )
            update_current_best(result, current_best)
    previous_best = current_best
    for rafsi_list in rafsi_list_list[2:]:
        current_best = [{}, {}]
        for rafsi in rafsi_list:
            for is_tosmabru in range(2):
                for lerfu in previous_best[is_tosmabru]:
                    result = combine(
                        previous_best[is_tosmabru][lerfu][0],
                        rafsi,
                        previous_best[is_tosmabru][lerfu][1],
                        is_tosmabru,
                        generate_cmene
                    )
                    update_current_best(result, current_best)
        previous_best = current_best
    best_lujvo = None
    best_score = 10**100
    for lerfu, lujvo_and_score in previous_best[0].items():
        if (is_vowel(lerfu) and not generate_cmene) or \
                (not is_vowel(lerfu) and generate_cmene):
            if lujvo_and_score[1] < best_score:
                best_lujvo, best_score = lujvo_and_score
    if best_lujvo is None:
        raise NoLujvoFoundError(f"No lujvo found for {{{' '.join(valsi_list)}}}")
    return best_lujvo, best_score

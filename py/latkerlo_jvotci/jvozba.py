"""
Copyright (c) 2023 Miao Liang (https://codeberg.org/tb148)
Licensed under the Apache License, Version 2.0

Modified by latkerlo (https://github.com/latkerlo), Copyright (c) 2023-2024
"""

from latkerlo_jvotci.tarmi import *
from latkerlo_jvotci.tools import analyse_brivla, check_zihevla_or_rafsi, normalise
from latkerlo_jvotci.rafsi import RAFSI_LIST
from latkerlo_jvotci.data import BANNED_TRIPLES
from latkerlo_jvotci.exceptions import NoLujvoFoundError, InvalidClusterError, NonLojbanCharacterError, NotZihevlaError, NotBrivlaError

TOSYNONE = 0
TOSMABRU = 1
TOSYhUhU = 2


def score(rafsi: str) -> int:
    """
    Calculate the lujvo score for a rafsi.

    :param rafsi: A rafsi, possibly including a hyphen.
    :return: The lujvo score for the rafsi (+hyphen).
    """
    tarmi_score = tarmi_ignoring_hyphen(rafsi)
    if tarmi_score == OTHER_RAFSI:
        tarmi_score = 0
    return (
        1000 * len(rafsi)
        - 400 * rafsi.count("'")  # a lie that draws a smile
        + 100 * rafsi.count("y")
        - 10 * tarmi_score
        - (
            rafsi.count("a")
            + rafsi.count("e")
            + rafsi.count("i")
            + rafsi.count("o")
            + rafsi.count("u")
        )
    )


def process_tanru(tanru: str | list[str]) -> list[str]:
    """
    Create a cleaned-up list of tanru components from a string or list.

    :param tanru: A tanru string or list.
    :return: A list of normalised tanru components.
    """
    if isinstance(tanru, str):
        valsi_list = tanru.split()
    elif isinstance(tanru, list):
        valsi_list = tanru
    else:
        raise TypeError(f"Cannot make lujvo from type {type(tanru)}")

    # TODO: replace, or make split_words play nice with zi'evla
    # expanded_valsi_list: list[str] = []
    # for valsi in valsi_list:
    #     expanded_valsi_list += split_words(valsi) if "-" not in valsi else [valsi]
    valsi_list = [normalise(x) for x in valsi_list]
    return valsi_list


def get_rafsi_for_rafsi(
        rafsi: str,
        r_type: str | int,
        is_first: bool,
        is_last: bool,
        consonants: str = CLUSTER,
        glides: bool = False) \
        -> list[tuple[str, int]]:
    """
    Create list of possible rafsi + hyphen forms for a rafsi.

    :param rafsi: The rafsi to use.
    :param r_type: The rafsi's form.
    :param is_first: True if the rafsi is the first tanru component.
    :param is_last: True if the rafsi is the last tanru component.
    :param consonants: Which consonant rules to use.
    :param glides: True if glides count as consonants.
    :return: List of possibly rafsi+hyphen forms.
    """
    result: list[tuple[str, int]] = []

    if not is_first and is_vowel(rafsi[0]) and not is_glide(rafsi):
        rafsi = "'" + rafsi

    if r_type in ["SHORT BRIVLA", CCVC, CVCC]:
        if is_last:
            result.append((rafsi, 2))
        else:
            result.append((rafsi + "y", 2))

    elif r_type in ["LONG BRIVLA", CCVCV, CVCCV]:
        if is_last:
            result.append((rafsi, 2))
        elif not (r_type == CVCCV and rafsi[2:4] in INITIAL):
            result.append((rafsi + "'y", 2))

    elif r_type == "EXPERIMENTAL RAFSI":
        if consonants != CLUSTER and (is_consonant(rafsi[0]) or (glides and is_glide(rafsi))):
            num_consonants = 1
        else:
            num_consonants = 0

        if is_last:
            result.append((rafsi, num_consonants))
        elif not is_first:
            result.append((rafsi + "'y", num_consonants))
        else:
            # We end it with a ' to show that a rafsi like u'u
            # will fall off if you don't do something later.
            # This is probably a bad idea. samyuan stop me.
            result.append((rafsi + "'", num_consonants))

    elif r_type in [CVV, CVhV]:
        num_consonants = 0 if consonants == CLUSTER else 1
        if is_first:
            result.append((rafsi + "'", num_consonants))
        elif not is_last:
            result.append((rafsi + "'y", num_consonants))
        result.append((rafsi, num_consonants))

    elif r_type == CCV:
        result.append((rafsi, 2))
        result.append((rafsi + "'y", 2))

    elif r_type == CVC:
        result.append((rafsi, 2))
        if not is_last:
            result.append((rafsi + "y", 2))

    else:
        assert False, r_type
    return result


def get_rafsi_list_list(
        valsi_list: list[str],
        y_hyphens: str = STANDARD,
        exp_rafsi_shapes: bool = False,
        consonants: str = CLUSTER,
        glides: bool = False,
        allow_mz: bool = False) \
        -> list[list[tuple[str, int]]]:
    """
    Create list of rafsi lists for each valsi in valsi_list.

    :param valsi_list: List of valsi to find rafsi for.
    :param y_hyphens: Which y-hyphen rules to use.
    :param exp_rafsi_shapes: True if experimental rafsi shapes are allowed.
    :param allow_mz: True if mz is a valid consonant cluster.
    :param consonants: Which consonant rules to use.
    :param glides: True if glides count as consonants.
    :return: List of rafsi lists.
    """
    from latkerlo_jvotci.katna import jvokaha_2

    list_of_rafsi_lists: list[list[tuple[str, int]]] = []
    for i, valsi in enumerate(valsi_list):
        rafsi_list: list[tuple[str, int]] = []
        is_first = i == 0
        is_last = i == len(valsi_list) - 1
        if valsi[-1] == "-":
            is_short_brivla = valsi[0] != "-"
            valsi = valsi.strip('-')
            if not is_only_lojban_characters(valsi):
                raise NonLojbanCharacterError(f"Non-lojban character in {{{valsi}}}")
            if valsi[-1] == "'":
                raise NonLojbanCharacterError(f"rafsi cannot end with ': {{{valsi}}}")

            if is_short_brivla:
                try:
                    b_type, _ = analyse_brivla(
                        valsi + "a",
                        y_hyphens=y_hyphens,
                        exp_rafsi_shapes=exp_rafsi_shapes,
                        allow_mz=allow_mz
                    )
                except NotBrivlaError:
                    raise NoLujvoFoundError(f"rafsi + a is not a brivla: {{{valsi}}}")

                if b_type not in [ZIhEVLA, GISMU]:
                    raise NoLujvoFoundError(f"rafsi + a is not a gismu or zi'evla: {{{valsi}}}")

                if len(valsi) >= 6 and is_consonant(valsi[-1]):
                    does_decompose = True
                    try:
                        jvokaha_2(valsi, y_hyphens=y_hyphens, allow_mz=allow_mz)
                    except (DecompositionError, InvalidClusterError):
                        does_decompose = False
                    if does_decompose:
                        raise NoLujvoFoundError(f"short zi'evla rafsi falls apart: {{{valsi}}}")

                rafsi_list += get_rafsi_for_rafsi(valsi, "SHORT BRIVLA", is_first, is_last, consonants, glides)

            else:
                raftai = rafsi_tarmi(valsi)
                if raftai == OTHER_RAFSI:
                    zihevla_or_rafsi = None
                    try:
                        b_type, _ = analyse_brivla(
                            valsi,
                            y_hyphens=y_hyphens,
                            exp_rafsi_shapes=exp_rafsi_shapes,
                            allow_mz=allow_mz
                        )
                        if b_type == ZIhEVLA:
                            zihevla_or_rafsi = ZIhEVLA

                    except NotBrivlaError:
                        if exp_rafsi_shapes:
                            try:
                                shape = check_zihevla_or_rafsi(
                                    valsi,
                                    exp_rafsi_shapes=exp_rafsi_shapes,
                                    allow_mz=allow_mz
                                )
                                if shape == RAFSI:
                                    zihevla_or_rafsi = RAFSI

                            except NotZihevlaError:
                                raise NoLujvoFoundError(f"Not a valid rafsi shape: -{{{valsi}}}-")

                    if zihevla_or_rafsi is None:
                        raise NotZihevlaError(f"Not a valid rafsi or zi'evla shape: -{{{valsi}}}-")

                    r_type = "LONG BRIVLA" if zihevla_or_rafsi == ZIhEVLA else "EXPERIMENTAL RAFSI"
                    rafsi_list += get_rafsi_for_rafsi(valsi, r_type, is_first, is_last, consonants, glides)

                else:
                    if not is_valid_rafsi(valsi, allow_mz=allow_mz):
                        raise InvalidClusterError(f"Invalid cluster in rafsi: -{{{valsi}}}-")

                    rafsi_list += get_rafsi_for_rafsi(valsi, raftai, is_first, is_last, consonants, glides)

        else:
            if not is_only_lojban_characters(valsi):
                raise NonLojbanCharacterError(f"Non-lojban character in {{{valsi}}}")

            short_rafsi_list = None
            try:
                short_rafsi_list = RAFSI_LIST[valsi]
            except KeyError:
                pass

            if short_rafsi_list:
                for short_rafsi in short_rafsi_list:
                    raftai = rafsi_tarmi(short_rafsi)
                    if raftai == OTHER_RAFSI and not exp_rafsi_shapes:
                        continue

                    rafsi_list += get_rafsi_for_rafsi(short_rafsi, raftai, is_first, is_last, consonants, glides)

            b_type = None
            try:
                b_type, _ = analyse_brivla(
                    valsi,
                    y_hyphens=y_hyphens,
                    exp_rafsi_shapes=exp_rafsi_shapes,
                    allow_mz=allow_mz
                )
            except NotBrivlaError:
                pass

            if b_type == GISMU:
                rafsi_list += get_rafsi_for_rafsi(valsi[:-1], "SHORT BRIVLA", is_first, is_last, consonants, glides)
            if b_type in [GISMU, ZIhEVLA]:
                rafsi_list += get_rafsi_for_rafsi(valsi, "LONG BRIVLA", is_first, is_last, consonants, glides)

        list_of_rafsi_lists.append(rafsi_list)
    return list_of_rafsi_lists


def combine(
        lujvo: str,
        rafsi: str,
        lujvo_consonants: int,  # Max 2, so we can use it as an index
        rafsi_consonants: int,
        lujvo_score: int,
        index_list: [(int, int)],
        tosmabru_type: int,
        generate_cmevla: bool,
        tanru_len=0,
        y_hyphens: str = STANDARD,
        consonants: str = CLUSTER,
        glides: bool = False,
        allow_mz: bool = False) \
        -> tuple[int, int, int, str, list[(int, int)]] | None:
    """
    Add one rafsi to the end of the current lujvo (if possible) and
    calculate the score.

    :param lujvo: The current working lujvo.
    :param rafsi: The rafsi to add.
    :param lujvo_consonants: Number of consonants in the lujvo.
    :param rafsi_consonants: Number of consonants in the rafsi.
    :param lujvo_score: Current score of the lujvo.
    :param index_list: List of rafsi start/end indices.
    :param tosmabru_type: Which way the lujvo could still fall apart.
    :param generate_cmevla: True if final result should end in
    consonant.
    :param tanru_len: Number of components in the tanru.
    :param y_hyphens: Which y-hyphen rules to use.
    :param consonants: Which consonant rules to use.
    :param glides: True if glides count as consonants.
    :param allow_mz: True if mz is a valid consonant cluster.
    :return: Final tosmabru_type, num_consonants, score, and lujvo.
    """

    if is_consonant(lujvo[-1]):
        if is_consonant(rafsi[0]) and lujvo[-1] + rafsi[0] not in (MZ_VALID if allow_mz else VALID):
            return
    if lujvo[-1] not in "y'" and rafsi[0] == "'":  # TODO: this line not in JS?
        return
    if lujvo[-1] + rafsi[:2] in BANNED_TRIPLES:
        return

    raftai_1 = tarmi_ignoring_hyphen(rafsi)
    if lujvo[-1] not in "y'" and raftai_1 == OTHER_RAFSI:
        return
    hyphen = ""
    if lujvo[-1] == "'":
        if rafsi[0] == "'" or y_hyphens != STANDARD:
            hyphen = "y"
        else:
            return
    elif len(lujvo) <= 5 and not generate_cmevla:
        raftai_0 = tarmi_ignoring_hyphen(lujvo)
        if raftai_0 in [CVhV, CVV]:
            if y_hyphens == "FORCE_Y":
                hyphen = "'y"
            elif rafsi[0] == "r":
                hyphen = "n"
            else:
                hyphen = "r"
        if tanru_len == 2 and raftai_1 == CCV:
            hyphen = ""

    if tosmabru_type == TOSMABRU:
        if lujvo[-1] + rafsi[0] not in INITIAL:
            tosmabru_type = TOSYNONE
        elif raftai_1 == CVCCV:
            if rafsi[2:4] in INITIAL:
                return
            tosmabru_type = TOSYNONE
        elif raftai_1 == CVC:
            if rafsi[-1] == "y":
                return
        else:
            tosmabru_type = TOSYNONE
    elif tosmabru_type == TOSYhUhU:
        if rafsi[0] != "'" or contains_consonant(rafsi):
            tosmabru_type = TOSYNONE

    rafsi_start = len(lujvo) + len(hyphen) + (1 if rafsi[0] == "'" else 0)
    rafsi_end = rafsi_start + len(strip_hyphens(rafsi))
    index_list = index_list + [(rafsi_start, rafsi_end)]

    new_consonants = rafsi_consonants
    if hyphen and hyphen in "nr":
        new_consonants = 2
    elif consonants == CLUSTER:
        if rafsi_consonants != 2:
            i = -1
            while lujvo[i] in "'y":
                i -= 1

            j = 0
            while rafsi[j] == "'":
                j += 1

            if is_consonant(lujvo[i]) and (is_consonant(rafsi[j]) or (glides and is_glide(rafsi[j:]))):
                new_consonants = 2
            else:
                new_consonants = 0

    total_consonants = min(2, lujvo_consonants + new_consonants)
    if consonants == ONE_CONSONANT and total_consonants > 0:
        total_consonants = 2

    hyphen_score = 1700 if hyphen == "'y" else 1100 * len(hyphen)
    return (
        tosmabru_type,
        total_consonants,
        lujvo_score + hyphen_score + score(rafsi),
        lujvo + hyphen + rafsi,
        index_list
    )


def update_current_best(candidate, current_best):
    """
    Add the candidate to current_best if it is the best.

    :param candidate: The new candidate lujvo.
    :param current_best: The list of existing best candidates for each
    combination of tosmabru type and number of consonants.
    """
    if candidate is None:
        return
    tosmabru_type, num_consonants, res_score, res_lujvo, res_index_list = candidate
    if (
        res_lujvo[-1] not in current_best[tosmabru_type][num_consonants]
        or current_best[tosmabru_type][num_consonants][res_lujvo[-1]][1] > res_score
    ):
        current_best[tosmabru_type][num_consonants][res_lujvo[-1]] = res_lujvo, res_score, res_index_list


def get_lujvo_from_list(
        valsi_list: list[str],
        generate_cmevla: bool = False,
        y_hyphens: str = "STANDARD",
        exp_rafsi_shapes: bool = False,
        consonants: str = CLUSTER,
        glides: bool = False,
        allow_mz = False) \
        -> (str, int, list[(int, int)]):
    """
    Create the best lujvo for the given tanru (normalised list). Also get its
    score and rafsi index list.

    :param valsi_list: A pre-normalised list of tanru component strings.
    :param generate_cmevla: True if result should end in a consonant.
    :param y_hyphens: Which y-hyphen rules to use.
    :param exp_rafsi_shapes: True if experimental rafsi shapes are allowed.
    :param consonants: Which consonant rules to use.
    :param glides: True if glides count as consonants.
    :param allow_mz: True if mz is a valid consonant cluster.
    :return: The best lujvo, its score, and list of rafsi start/end indices.
    """
    rafsi_list_list = get_rafsi_list_list(valsi_list, y_hyphens, exp_rafsi_shapes, consonants, glides, allow_mz)
    current_best = [[{}, {}, {}], [{}, {}, {}], [{}, {}, {}]]
    for rafsi0, num_consonants_0 in rafsi_list_list[0]:
        for rafsi1, num_consonants_1 in rafsi_list_list[1]:
            if tarmi_ignoring_hyphen(rafsi0) != CVC or generate_cmevla:
                tosmabru_type = TOSYNONE
            else:
                tosmabru_type = TOSYhUhU if rafsi0[-1] == "y" else TOSMABRU
            result = combine(
                rafsi0,
                rafsi1,
                num_consonants_0,
                num_consonants_1,
                score(rafsi0),
                [(0, len(strip_hyphens(rafsi0)))],
                tosmabru_type,
                generate_cmevla,
                tanru_len=len(rafsi_list_list),
                y_hyphens=y_hyphens,
                consonants=consonants,
                glides=glides,
                allow_mz=allow_mz
            )
            update_current_best(result, current_best)
    previous_best = current_best
    for rafsi_list in rafsi_list_list[2:]:
        current_best = [[{}, {}, {}], [{}, {}, {}], [{}, {}, {}]]
        for rafsi, rafsi_consonants in rafsi_list:
            for tosmabru_type in range(3):
                for num_consonants in range(3):
                    for lerfu in previous_best[tosmabru_type][num_consonants]:
                        result = combine(
                            previous_best[tosmabru_type][num_consonants][lerfu][0],
                            rafsi,
                            num_consonants,
                            rafsi_consonants,
                            previous_best[tosmabru_type][num_consonants][lerfu][1],
                            previous_best[tosmabru_type][num_consonants][lerfu][2],
                            tosmabru_type,
                            generate_cmevla,
                            y_hyphens=y_hyphens,
                            consonants=consonants,
                            glides=glides,
                            allow_mz=allow_mz
                        )
                        update_current_best(result, current_best)
        previous_best = current_best
    best_lujvo = None
    best_score = 10**100
    best_index_list = None
    for lerfu, lujvo_and_score in previous_best[0][2].items():
        if (is_vowel(lerfu) and not generate_cmevla) or \
                (is_consonant(lerfu) and generate_cmevla):
            if lujvo_and_score[1] < best_score:
                best_lujvo, best_score, best_index_list = lujvo_and_score
    if best_lujvo is None:
        raise NoLujvoFoundError(f"No lujvo found for {{{' '.join(valsi_list)}}}")

    return best_lujvo, best_score, best_index_list


def get_lujvo_with_analytics(
        tanru: str | list[str],
        generate_cmevla: bool = False,
        y_hyphens: str = STANDARD,
        exp_rafsi_shapes: bool = False,
        consonants: str = CLUSTER,
        glides: bool = False,
        allow_mz = False) \
        -> (str, int, list[(int, int)]):
    """
    Create the best lujvo for the given tanru (string or list). Also get its
    score and rafsi index list.

    :param tanru: A tanru in string or list form.
    :param generate_cmevla: True if result should end in a consonant.
    :param y_hyphens: Which y-hyphen rules to use.
    :param exp_rafsi_shapes: True if experimental rafsi shapes are allowed.
    :param consonants: Which consonant rules to use.
    :param glides: True if glides count as consonants.
    :param allow_mz: True if mz is a valid consonant cluster.
    :return: The best lujvo, its score, and list of rafsi start/end indices.
    """
    return get_lujvo_from_list(
        process_tanru(tanru),
        generate_cmevla,
        y_hyphens,
        exp_rafsi_shapes,
        consonants,
        glides,
        allow_mz)


def get_lujvo(
        tanru: str | list[str],
        generate_cmevla = False,
        y_hyphens: str = STANDARD,
        exp_rafsi_shapes = False,
        consonants: str = CLUSTER,
        glides: bool = False,
        allow_mz = False) \
        -> str:
    """
    Create the best lujvo for the given tanru (string or list).

    :param tanru: A tanru in string or list form.
    :param generate_cmevla: True if result should end in a consonant.
    :param y_hyphens: Which y-hyphen rules to use.
    :param exp_rafsi_shapes: True if experimental rafsi shapes are allowed.
    :param consonants: Which consonant rules to use.
    :param glides: True if glides count as consonants.
    :param allow_mz: True if mz is a valid consonant cluster.
    :return: The best lujvo.
    """
    return get_lujvo_from_list(
        process_tanru(tanru),
        generate_cmevla,
        y_hyphens,
        exp_rafsi_shapes,
        consonants,
        glides,
        allow_mz)[0]


if __name__ == "__main__":
    # Terrible CLI for quick testing. Use at own risk.
    cmevla_setting = False
    y_setting = STANDARD
    raf_setting = False
    con_setting = CLUSTER
    glide_setting = False
    mz_setting = False
    while True:
        string = input(
            f"({str(cmevla_setting)[0]}{y_setting[0]}{str(raf_setting)[0]}{con_setting[0]}{str(glide_setting)[0]}{str(mz_setting)[0]}) Enter a tanru: ")
        if not string:
            break
        if string[0] == "/":
            for change in string[1:].lower().split():
                match change:
                    case "cmevla":
                        cmevla_setting = True
                    case "brivla":
                        cmevla_setting = False
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
            print(get_lujvo_with_analytics(string,
                            generate_cmevla=cmevla_setting, y_hyphens=y_setting,
                            exp_rafsi_shapes=raf_setting, consonants=con_setting, glides=glide_setting, allow_mz=mz_setting))
        except (NoLujvoFoundError, NonLojbanCharacterError, NotZihevlaError) as e:
            print(e)

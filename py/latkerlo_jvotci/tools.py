"""
Copyright (c) 2023-2024 latkerlo (https://github.com/latkerlo)
Licensed under the MIT License
"""

from latkerlo_jvotci.tarmi import *
from latkerlo_jvotci.exceptions import DecompositionError, InvalidClusterError, NotBrivlaError, NotZihevlaError


def normalise(word: str) -> str:
    """
    Convert word to standard lojban form:

    - Remove initial period
    - Make lowercase
    - Convert h -> '
    - Remove commas

    :param word: A lojban word.
    :return: The normalised form.
    """
    if word[0] == ".":
        word = word[1:]
    word = word.lower()
    word = word.replace("h", "'")
    word = word.replace(",", "")
    return word


def is_gismu_or_lujvo(a_string: str, allow_rn_hyphens: bool = False, allow_mz: bool = False) -> bool:
    """
    Check if the string is a valid gismu or lujvo.

    :param a_string: Some string.
    :param allow_rn_hyphens: True if unnecessary r & n hyphens are
    allowed.
    :param allow_mz: True if mz is a valid consonant cluster.
    :return: Return true if the string is a valid gismu or lujvo.
    """
    from latkerlo_jvotci.katna import jvokaha

    if len(a_string) < 5:
        return False
    if not is_vowel(a_string[-1]):
        return False

    if is_gismu(a_string, allow_mz=allow_mz):
        return True

    try:
        jvokaha(a_string, allow_rn_hyphens, allow_mz=allow_mz)  # TODO more settings?
    except (DecompositionError, InvalidClusterError):
        return False

    return True


def is_slinkuhi(string: str, allow_mz: bool = False) -> bool:
    """
    Check if string is not a valid word because a leading CV cmavo
    would combine to create a lujvo. (slinku'i)

    :param string: A string.
    :param allow_mz: True if mz is a valid consonant cluster.
    :return: True if string fails slinku'i test.
    """
    from latkerlo_jvotci.katna import jvokaha
    try:
        jvokaha("to" + string, allow_rn_hyphens=True, allow_mz=allow_mz)
        return True
    except (DecompositionError, InvalidClusterError):
        return False


def check_zihevla_or_rafsi(
        valsi: str,
        require_zihevla: bool = False,
        exp_rafsi_shapes: bool = False,
        allow_mz: bool = False) \
        -> str:
    """
    Check morphology rules that are specific to zi'evla and
    experimental rafsi shapes.

    Caution: May return ZIhEVLA when string is not valid word.

    :param valsi: A string.
    :param require_zihevla: True if rafsi-shapes should raise an Error.
    :param exp_rafsi_shapes: True if experimental rafsi shapes are
    allowed.
    :param allow_mz: True if mz is a valid consonant cluster.
    :return: ZIhEVLA or RAFSI if string passes tests.
    :raises:
        NotZihevlaError: if any morphology tests are failed.
    """
    valsi_copy = valsi

    if require_zihevla and len(valsi) < 4:
        raise NotZihevlaError(f"too short to be zi'evla: {{{valsi_copy}}}")

    chunk = ""
    pos = 0
    num_syllables = 0
    cluster_pos = None
    num_consonants = 0
    final_consonant_pos = 0
    while len(valsi) > 0:
        if is_consonant(valsi[0]):
            while valsi and is_consonant(valsi[0]):
                chunk += valsi[0]
                valsi = valsi[1:]

            if len(chunk) >= 2 and cluster_pos is None:
                if num_consonants > 1:
                    raise NotZihevlaError(f"too many consonants before first cluster: {{{valsi_copy}}}")
                cluster_pos = pos

            if num_syllables == 0 and len(chunk) >= 2:
                if chunk[:2] not in INITIAL:
                    raise NotZihevlaError(f"invalid word initial: {{{valsi_copy}}}")

            for i in range(len(chunk) - 1):
                cluster = chunk[i:i+2]
                if cluster not in (MZ_VALID if allow_mz else VALID):
                    raise NotZihevlaError(f"invalid cluster {{{cluster}}} in {{{valsi_copy}}}")

            for i in range(len(chunk) - 2):
                cluster = chunk[i:i+3]
                if cluster in BANNED_TRIPLES:
                    raise NotZihevlaError(f"banned triple {{{cluster}}} in {{{valsi_copy}}}")

            if pos == 0:
                valid = is_zihevla_initial_cluster(chunk)
                if not valid:
                    raise NotZihevlaError(f"invalid zi'evla initial cluster {{{chunk}}} in word {{{valsi_copy}}}")
            else:
                valid = is_zihevla_middle_cluster(chunk)
                if not valid:
                    raise NotZihevlaError(f"invalid zi'evla middle cluster {{{chunk}}} in word {{{valsi_copy}}}")

            final_consonant_pos = pos
            num_consonants += len(chunk)

        elif is_vowel(valsi[0]):
            while valsi and is_vowel(valsi[0]):
                chunk += valsi[0]
                valsi = valsi[1:]

            if pos == 0:
                if chunk in START_VOWEL_CLUSTERS or chunk in FOLLOW_VOWEL_CLUSTERS:
                    num_syllables += 1
                else:
                    raise NotZihevlaError(f"starts with bad vowels: {{{valsi_copy}}}")

            else:
                try:
                    num_syllables += len(split_vowel_cluster(chunk))
                except DecompositionError:
                    raise NotZihevlaError(f"vowel decomp error: {{{chunk}}} in {{{valsi_copy}}}")

        elif valsi[0] == "'":
            chunk = "'"
            valsi = valsi[1:]

            if pos < 1 or not is_vowel(valsi_copy[pos-1]):
                raise NotZihevlaError("' not preceded by vowel")
            if not valsi or not is_vowel(valsi_copy[pos+1]):
                raise NotZihevlaError("' not followed by vowel")
        else:
            raise NotZihevlaError(f"unexpected character {{{valsi[0]}}} in {{{valsi_copy}}}")

        pos += len(chunk)
        chunk = ""

    if num_syllables < 2 and (require_zihevla or not exp_rafsi_shapes):
        raise NotZihevlaError(f"too few syllables: {{{valsi_copy}}}")

    elif num_syllables > 2:
        if cluster_pos and cluster_pos > 0:
            if is_brivla(valsi_copy[cluster_pos:]):
                raise NotZihevlaError(f"falls apart at cluster: {{{valsi_copy[0:cluster_pos]}_{valsi_copy[cluster_pos:]}}}")

            for i in range(cluster_pos):
                if is_consonant(valsi_copy[cluster_pos - i]):
                    if is_brivla(valsi_copy[cluster_pos - i:]):
                        raise NotZihevlaError(f"falls apart before cluster: {{{valsi_copy[0:cluster_pos-i]}_{valsi_copy[cluster_pos-i:]}}}")

    if cluster_pos is None:
        if require_zihevla:
            raise NotZihevlaError(f"no cluster: {{{valsi_copy}}}")
        if not is_consonant(valsi_copy[0]) and not exp_rafsi_shapes:
            raise NotZihevlaError(f"not valid rafsi shape: {{{valsi_copy}}}")
        if num_consonants > 1:
            raise NotZihevlaError(f"too many consonants without cluster: {{{valsi_copy}}}")
        if final_consonant_pos > 0:
            raise NotZihevlaError(f"non-initial consonant(s) without cluster: {{{valsi_copy}}}")
    else:
        if not (is_vowel(valsi_copy[0]) and is_consonant(valsi_copy[1])):
            if is_slinkuhi(valsi_copy, allow_mz=allow_mz):
                raise NotZihevlaError(f"slinku'i: {{to,{valsi_copy}}}")

    return ZIhEVLA if cluster_pos is not None else RAFSI


def is_brivla(
        valsi: str,
        y_hyphens: str = STANDARD,
        exp_rafsi_shapes: bool = False,
        consonants: str = CLUSTER,
        glides: bool = False,
        allow_mz: bool = False) \
        -> bool:
    """
    Check if string is a valid lojban brivla.

    :param valsi: A string.
    :param y_hyphens: Which y-hyphen rules to use.
    :param exp_rafsi_shapes: True if experimental rafsi shapes are
    allowed.
    :param consonants: Which consonant rules to use.
    :param glides: True if glides count as consonants.
    :param allow_mz: True if mz is a valid consonant cluster.
    :return: True if string is a valid lojban brivla.
    """
    try:
        return CMEVLA != analyse_brivla(valsi, y_hyphens, exp_rafsi_shapes, consonants, glides, allow_mz)[0]
    except NotBrivlaError:
        return False


def analyse_brivla(
        valsi: str,
        y_hyphens: str = STANDARD,
        exp_rafsi_shapes: bool = False,
        consonants: str = CLUSTER,
        glides: bool = False,
        allow_mz: bool = False) \
        -> (str, list[str]):
    """
    Tells you the type and decomposition of any brivla or decomposable
    cmevla.
    Raises an error for single-unit cmevla because it doesn't check the
    cmevla morphology rules.

    :param valsi: A string.
    :param y_hyphens: Which y-hyphen rules to use.
    :param exp_rafsi_shapes: True if experimental rafsi shapes are
    allowed.
    :param consonants: Which consonant rules to use.
    :param glides: True if glides count as consonants.
    :param allow_mz: True if mz is a valid consonant cluster.
    :return: The word type and a list of pieces (rafsi + hyphens).
    :raises:
        NotBrivlaError: if string is neither a brivla nor decomposable
        cmevla.
    """
    from latkerlo_jvotci.katna import jvokaha_2, jvokaha
    valsi = normalise(valsi)

    is_cmevlatai = False
    if not valsi:
        raise NotBrivlaError("empty string")
    elif is_consonant(valsi[-1]):
        is_cmevlatai = True
    elif not is_vowel(valsi[-1]):
        raise NotBrivlaError(f"doesn't end in consonant or vowel: {{{valsi}}}")

    if is_cmevlatai:
        if is_gismu(valsi + "a", allow_mz=allow_mz):
            raise NotBrivlaError(f"non-decomposable cmevla: {{{valsi}}}")
    else:
        if is_gismu(valsi, allow_mz=allow_mz):
            return GISMU, [valsi]

    try:
        allow_rn = y_hyphens != FORCE_Y
        result_parts = jvokaha(valsi, allow_rn_hyphens=allow_rn, y_hyphens=y_hyphens, consonants=consonants, glides=glides, allow_mz=allow_mz)
        return CMEVLA if is_cmevlatai else LUJVO, result_parts
    except (DecompositionError, InvalidClusterError, IndexError):
        pass

    if not (is_vowel(valsi[0]) or is_consonant(valsi[0])):
        raise NotBrivlaError(f"doesn't start with vowel or consonant: {{{valsi}}}")

    y_parts = valsi.split("y")

    if len(y_parts) == 1:
        if is_cmevlatai:
            raise NotBrivlaError(f"non-decomposable cmevla: {{{valsi}}}")

        try:
            check_zihevla_or_rafsi(valsi, require_zihevla=True, exp_rafsi_shapes=exp_rafsi_shapes, allow_mz=allow_mz)
            return ZIhEVLA, [valsi]
        except NotZihevlaError:
            raise NotBrivlaError("no hyphens, and not valid zi'evla")

    result_parts = []
    next_hyphen = ""
    has_cluster = False
    is_mahortai = True
    consonant_before_break = False
    num_consonants = 0
    for i, part in enumerate(y_parts):
        if i != 0:
            next_hyphen += "y"
        part_copy = part

        if not part:
            raise NotBrivlaError("double y")

        if part[0] == "'":
            part = part[1:]
            part_copy = part
            next_hyphen += "'"

            if not part:
                raise NotBrivlaError("that was only a '")
            if not (is_vowel(part[0]) and not is_glide(part)):
                raise NotBrivlaError(f"consonant or glide after ': {{{part}}}")
        elif i > 0 and is_vowel(part[0]) and not is_glide(part):
            raise NotBrivlaError(f"non-glide vowel after y: {{{part}}}")
        if next_hyphen:
            result_parts.append(next_hyphen)
            next_hyphen = ""

        if rafsi_tarmi(part) == CVC:
            result_parts.append(part)
            consonant_before_break = True
            num_consonants += 2
            continue
        if rafsi_tarmi(part + "a") == CCV:
            raise NotBrivlaError("can't drop vowel on CCV rafsi")

        if i > 0 and (is_consonant(part[0]) or is_glide(part)):
            is_mahortai = False
        if consonant_before_break and (is_consonant(part[0]) or (glides and is_glide(part))):
            has_cluster = True

        can_be_rafsi = True  # including string of rafsi
        require_cluster = False
        did_add_a = False
        if part[-1] == "'":
            if y_hyphens == STANDARD and not has_cluster and i < len(y_parts) - 1 and y_parts[i+1][0] != "'":
                require_cluster = True
            part = part[:-1]
            part_copy = part
            next_hyphen += "'"

            if not is_vowel(part[-1]):
                raise NotBrivlaError(f"non-vowel before ': {part}")
        elif i < len(y_parts) - 1 or is_cmevlatai:
            if is_vowel(part[-1]):
                can_be_rafsi = False
            part = part + "a"
            did_add_a = True
            require_cluster = True

        did_kaha = False
        if can_be_rafsi:
            found_parts = [part]
            try:
                found_parts = jvokaha_2(part_copy, y_hyphens=y_hyphens, allow_mz=allow_mz)
                if len(found_parts) < 2 and not is_valid_rafsi(found_parts[0], allow_mz=allow_mz):
                    raise NotBrivlaError(f"invalid rafsi: {{{found_parts[0]}}}")
                result_parts += found_parts
                did_kaha = True
            except (DecompositionError, InvalidClusterError, IndexError):
                pass

            for part_part in found_parts:
                raftai = rafsi_tarmi(part_part)
                if raftai in [CVV, CVhV]:
                    num_consonants += 1
                elif raftai != OTHER_RAFSI:
                    num_consonants += 2
                    has_cluster = True

        if did_kaha:
            if rafsi_tarmi(part) in [CVV, CVhV]:
                if require_cluster and not has_cluster and \
                        (y_hyphens == STANDARD or
                        not (i == len(y_parts) - 2 and rafsi_tarmi(y_parts[1]) in [CVV, CCV])):
                    raise NotBrivlaError("falls off because y")

            if i == 0:
                smabru_part = ""
                if rafsi_tarmi(part[:4]) == CVhV:
                    smabru_part = part[4:]
                elif rafsi_tarmi(part[:3]) == CVV:
                    smabru_part = part[3:]
                elif is_consonant(part[0]) and is_vowel(part[1]): # and not is_gismu(part):
                    smabru_part = part[2:]

                if smabru_part:
                    if did_add_a:
                        smabru_part = smabru_part[:-1]
                    else:
                        smabru_part = strip_hyphens(smabru_part)

                    if is_valid_rafsi(smabru_part):
                        raise NotBrivlaError("tosmabru")

                    try:
                        jvokaha(smabru_part, allow_mz=allow_mz)
                        raise NotBrivlaError("tosmabru")
                    except (DecompositionError, InvalidClusterError):
                        pass
        else:
            require_zihevla = require_cluster or not exp_rafsi_shapes
            try:
                shape_type = check_zihevla_or_rafsi(
                    part,
                    require_zihevla=require_zihevla,
                    exp_rafsi_shapes=exp_rafsi_shapes,
                    allow_mz=allow_mz)
            except NotZihevlaError as e:
                raise NotBrivlaError(e)

            if shape_type == ZIhEVLA:
                has_cluster = True

            if is_consonant(part[0]) or (glides and is_glide(part)):
                num_consonants += 1

            result_parts.append(part_copy)

        consonant_before_break = False

    if not has_cluster:
        if consonants == CLUSTER:
            raise NotBrivlaError("no clusters")
        elif consonants == TWO_CONSONANTS and num_consonants < 2:
            raise NotBrivlaError("not enough consonants")
        elif consonants == ONE_CONSONANT and num_consonants < 1:
            raise NotBrivlaError("not enough consonants")
        elif is_mahortai:
            raise NotBrivlaError("cmavo shaped or maybe multiple cmavo shaped")

    if not (is_vowel(valsi[0]) and (is_consonant(valsi[1]) or valsi[1] == "y")):
        if is_slinkuhi(valsi, allow_mz=allow_mz):
            raise NotBrivlaError("slinku'i")

    return CMEVLA if is_cmevlatai else EXTENDED_LUJVO, result_parts


def get_rafsi_indices(rafsi_list: list[str]) -> list[(int, int)]:
    """
    Create a list of start and end indices for each rafsi in a list of rafsi
    and hyphens (a word split into its components).

    Example:
    ["tcan", "y", "ja'a"] ->
    [(0, 4), (5, 9)]

    :param rafsi_list: List of rafsi and hyphens (a decomposed word).
    :return: List of start and end indices for non-hyphen components.
    """
    position = 0
    index_list = []
    for piece in rafsi_list:
        if piece not in HYPHENS:
            index_list.append((position, position + len(piece)))
        position += len(piece)
    return index_list

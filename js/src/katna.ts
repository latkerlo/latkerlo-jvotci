/*
Copyright (c) 2021 sozysozbot (https://github.com/sozysozbot)
Licensed under the MIT License

Modified by latkerlo (https://github.com/latkerlo), Copyright (c) 2023-2024
*/

/**
 * Return the selrafsi for a given rafsi, if one exists.
 * Otherwise, null is returned.
 *
 * @param rafsi The rafsi to search for.
 * @returns The corresponding selrafsi, if applicable, otherwise None.
 */
function searchSelrafsiFromRafsi(rafsi: string): string | null {
  if (rafsi !== "brod" && rafsi.length === 4 && !rafsi.includes("'")) {  // 4-letter rafsi
    for (let u = 0; u < 5; u++) {
      const gismuCandid = rafsi + "aeiou"[u];
      if (RAFSI_LIST.has(gismuCandid))
        return gismuCandid;
    }
  }
  for (const [valsi, rafsi_list] of RAFSI_LIST.entries()) {
    if (rafsi_list.includes(rafsi))
      return valsi;
  }
  return null;
}

/**
 * Create a list of selrafsi and formatted rafsi from a list of rafsi.
 *
 * Example:
 * ["lat", "mot", "kelr", "y", "kerlo"] ->
 * ["mlatu", "-mot-", "kelr-", "kerlo"]
 * 
 * @param rafsiList List of rafsi and hyphens (a decomposed word).
 * @param yHyphens Which y-hyphen rules to use.
 * @param allowMZ True if mz is a valid consonant cluster.
 * @returns List of selrafsi and formatted rafsi.
 */
function selrafsiListFromRafsiList(
  rafsiList: string[],
  {
    yHyphens = YHyphenSetting.STANDARD,
    allowMZ = false
  } = {}
): string[] {
  const result = rafsiList.map((rafsi) => HYPHENS.includes(rafsi) ? "" : rafsi);
  const selrafsiList = result.map((rafsi) => searchSelrafsiFromRafsi(rafsi));
  for (let i = 0; i < result.length; i++) {
    if (result[i].length === 0)
      continue;

    if (selrafsiList[i] !== null)
      result[i] = selrafsiList[i]!;
    else if (i < rafsiList.length - 2 && rafsiList[i + 1][0] === "y" && isBrivla(result[i] + "a", { yHyphens: yHyphens, allowMZ: allowMZ }))
      result[i] = result[i] + "-";
    else if (isBrivla(result[i], { yHyphens: yHyphens, allowMZ: allowMZ })) { }
    else if (i === rafsiList.length - 1 && isBrivla(result[i] + "a", { yHyphens: yHyphens, allowMZ: allowMZ }))
      result[i] = result[i] + "-";
    else
      result[i] = "-" + result[i] + "-";
  }
  return result.filter((rafsi) => rafsi.length > 0);
}

/**
 * Check if corr and other represent the same lujvo.
 * other may have unnecessary hyphens.
 * 
 * @param corr A list of parts of the correct lujvo.
 * @param other A list of parts of a candidate to test.
 * @returns True if the lujvo are the same except for hyphens.
 */
function compareLujvoPieces(corr: string[], other: string[]): boolean {
  let i = 0;
  for (let j = 0; j < corr.length; j++) {
    const part = corr[j];
    if (part === other[i]) {
      i += 1;
      continue;
    }

    if (
      0 < i && i < other.length - 1
      && "rn".includes(other[i])
      && [Tarmi.CVV, Tarmi.CVhV].includes(rafsiTarmi(other[i - 1]))
      && (i > 1 || [Tarmi.CCVCV, Tarmi.CCVC, Tarmi.CCV].includes(rafsiTarmi(other[i + 1])))
    ) {
      i += 1;
    }

    if (part === other[i])
      i += 1;
    else
      return false;
  }

  return i == other.length;
}

/**
 * Decompose lujvo to get a list of pieces (rafsi and hyphens). Raise
 * an error if the lujvo is not well-formed.
 * 
 * @param lujvo A lujvo to decompose.
 * @param allowRNHyphens True if unnecessary r & n hyphens are allowed.
 * @param yHyphens Which y-hyphen rules to use.
 * @param consonants Which consonant rules to use.
 * @param glides True if glides count as consonants.
 * @param allowMZ True if mz is a valid consonant cluster.
 * @returns List of lujvo pieces (rafsi and hyphens).
 */
function jvokaha(
  lujvo: string,
  {
    yHyphens = YHyphenSetting.STANDARD,
    consonants = ConsonantSetting.CLUSTER,
    glides = false,
    allowMZ = false
  } = {}
): string[] {
  const arr = jvokaha2(lujvo, { yHyphens: yHyphens, allowMZ: allowMZ });

  const rafsiTanru = arr.filter(x => x.length > 2).map(x => `-${x}-`);
  if (rafsiTanru.length == 1) {
    throw new TypeError("not enough rafsi");
  }
  let correctLujvo: string;
  try {
    correctLujvo = getLujvoFromList(
      rafsiTanru,
      {
        generateCmevla: isConsonant(arr[arr.length - 1].slice(-1)),
        yHyphens: yHyphens,
        consonants: consonants,
        glides: glides,
        allowMZ: allowMZ
      }
    )[0];
  } catch (e) {
    if (e instanceof NoLujvoFoundError)
      throw new DecompositionError(`no lujvo for ${rafsiTanru}`);
    else
      throw e;
  }

  let coolAndGood: boolean;
  if (yHyphens == YHyphenSetting.FORCE_Y)
    coolAndGood = correctLujvo === lujvo;
  else
    coolAndGood = compareLujvoPieces(jvokaha2(correctLujvo, { yHyphens: YHyphenSetting.STANDARD, allowMZ: allowMZ }), arr);

  if (coolAndGood)
    return arr;
  else
    throw new DecompositionError("malformed lujvo {" + lujvo +
      "}; it should be {" + correctLujvo + "}")
}

/**
 * Decompose lujvo to get a list of pieces (rafsi and hyphens).
 * Raises an error if the string is not decomposable, but not if it
 * is invalid for other reasons.
 * 
 * @param lujvo A lujvo to decompose.
 * @param yHyphens Which y-hyphen rules to use.
 * @param allowMZ True if mz is a valid consonant cluster.
 * @returns List of lujvo pieces (rafsi and hyphens).
 */
function jvokaha2(
  lujvo: string,
  {
    yHyphens = YHyphenSetting.STANDARD,
    allowMZ = false
  } = {}
): string[] {
  const original_lujvo = lujvo;
  const res: string[] = [];
  while (true) {
    if (lujvo === "")
      return res;

    // remove hyphen
    if (res.length > 0 && res[res.length - 1].length !== 1) {  // hyphen cannot begin a word; nor can two hyphens
      if (lujvo[0] === "y") {  // y-hyphen
        res.push(lujvo[0]);
        lujvo = lujvo.slice(1);
        continue;

      } else if (
        yHyphens !== YHyphenSetting.FORCE_Y
        && [Tarmi.CVV, Tarmi.CVhV].includes(rafsiTarmi(res[res.length - 1]))
        && (lujvo.slice(0, 2) === "nr"  // n-hyphen is only allowed before r
          || (lujvo[0] === "r" && isConsonant(lujvo[1]) && lujvo[1] !== "r"))  // r followed by a consonant
      ) {
        res.push(lujvo[0]);
        lujvo = lujvo.slice(1);
        continue;

      } else if (yHyphens !== YHyphenSetting.STANDARD && lujvo.slice(0, 2) === "'y") {
        res.push(lujvo.slice(0, 2));
        lujvo = lujvo.slice(2);
        continue;
      }
    }

    // drop rafsi from front

    // CVV can always be dropped
    if (rafsiTarmi(lujvo.slice(0, 3)) === Tarmi.CVV
      && ["ai", "ei", "oi", "au"].includes(lujvo.slice(1, 3))) {
      res.push(lujvo.slice(0, 3));
      lujvo = lujvo.slice(3);
      continue;
    }

    // CVhV can always be dropped
    if (rafsiTarmi(lujvo.slice(0, 4)) === Tarmi.CVhV) {
      res.push(lujvo.slice(0, 4));
      lujvo = lujvo.slice(4);
      continue;
    }

    // CVCCY and CCVCY can always be dropped
    if ([Tarmi.CVCC, Tarmi.CCVC].includes(rafsiTarmi(lujvo.slice(0, 4)))) {
      if (isVowel(lujvo[1])) {
        if (!(allowMZ ? MZ_VALID : VALID).includes(lujvo.slice(2, 4)))
          throw new InvalidClusterError(`Invalid cluster {${lujvo.slice(2, 4)}} in 
          {${original_lujvo}}`);
      } else {
        if (!INITIAL.includes(lujvo.slice(0, 2)))
          throw new InvalidClusterError(`Invalid initial cluster {${lujvo.slice(0, 2)}} in 
          {${original_lujvo}}`);
      }

      if (lujvo.length === 4 || lujvo[4] === "y") {
        res.push(lujvo.slice(0, 4));
        if (lujvo[4] === "y")
          res.push("y");
        lujvo = lujvo.slice(5);
        continue;
      }
    }

    // the final rafsi can be 5-letter
    if ([Tarmi.CVCCV, Tarmi.CCVCV].includes(rafsiTarmi(lujvo))) {
      res.push(lujvo);
      return res;
    }

    if (rafsiTarmi(lujvo.slice(0, 3)) === Tarmi.CVC) {
      if (BANNED_TRIPLES.includes(lujvo.slice(2, 5)))
        throw new InvalidClusterError(`Invalid triple {${lujvo.slice(2, 5)}} in {${original_lujvo}}`);
      res.push(lujvo.slice(0, 3));
      lujvo = lujvo.slice(3);
      continue;
    }

    if (rafsiTarmi(lujvo.slice(0, 3)) === Tarmi.CCV) {
      if (!INITIAL.includes(lujvo.slice(0, 2)))
        throw new InvalidClusterError(`Invalid initial cluster {${lujvo.slice(0, 2)}} in {${original_lujvo}}`);
      if (lujvo == original_lujvo && lujvo.slice(3, 5) == "'y")
        throw new NotBrivlaError(`{${original_lujvo}} starts with CCV'y, making it a slinku'i`);
      res.push(lujvo.slice(0, 3));
      lujvo = lujvo.slice(3);
      continue;
    }

    // if all fails...
    // console.log(res, lujvo)
    throw new DecompositionError("Failed to decompose {" + original_lujvo + "}");
  }
}

/**
 * Calculate the score for a lujvo
 * 
 * @param lujvo the lujvo
 * @returns its score
 */
function scoreLujvo(lujvo: string, {
  generateCmevla = false,
  yHyphens = YHyphenSetting.STANDARD,
  consonants = ConsonantSetting.CLUSTER,
  expRafsiShapes = false,
  glides = false,
  allowMZ = false
} = {}): number {
  let settings = { generateCmevla, yHyphens, consonants, expRafsiShapes, glides, allowMZ };
  try {
    getVeljvo(lujvo, settings);
  } catch (e) {
    throw e;
  }
  let decomp;
  try {
    decomp = analyseBrivla(lujvo, settings)[1];
  } catch (e) {
    throw e;
  }
  return decomp.map(r =>
    ["y", "n", "r", ""].includes(r) ? 1100 * r.length : score(r)
  ).reduce((a, b) => a + b) - tiebreak(lujvo);
}

/**
 * Decompose a lujvo into a list of selrafsi and formatted rafsi.
 * 
 * @param lujvo Lujvo to decompose.
 * @param yHyphens Which y-hyphen rules to use.
 * @param exp_rafsi_shapes True if experimental rafsi shapes are allowed.
 * @param consonants Which consonant rules to use.
 * @param glides True if glides count as consonants.
 * @param allowMZ True if mz is a valid consonant cluster.
 * @returns List of selrafsi and rafsi.
 */
function getVeljvo(
  lujvo: string,
  {
    yHyphens = YHyphenSetting.STANDARD,
    expRafsiShapes = false,
    consonants = ConsonantSetting.CLUSTER,
    glides = false,
    allowMZ = false
  } = {}
): string[] {
  const [bType, rafsiList] = analyseBrivla(
    lujvo,
    {
      yHyphens: yHyphens,
      expRafsiShapes: expRafsiShapes,
      consonants: consonants,
      glides: glides,
      allowMZ: allowMZ
    }
  );

  if (![BrivlaType.LUJVO, BrivlaType.EXTENDED_LUJVO, BrivlaType.CMEVLA].includes(bType))
    throw new DecompositionError("Valsi is of type {" + bType + "}");
  return selrafsiListFromRafsiList(rafsiList, { yHyphens: yHyphens, allowMZ: allowMZ });
}

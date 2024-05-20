/*
Copyright (c) 2023-2024 latkerlo (https://github.com/latkerlo)
Licensed under the MIT License
*/

/**
 * Convert word to standard lojban form:
 *
 * - Remove initial period
 * - Make lowercase
 * - Convert h -> '
 * - Remove commas
 * 
 * @param word A lojban word.
 * @returns The normalised form.
 */
function normalise(word: string): string {
  if (word[0] === ".")
    word = word.slice(1);
  word = word.toLowerCase();
  word = word.replace(/h/g, "'");
  word = word.replace(/,/g, "");
  return word;
}

/**
 * Check if the string is a valid gismu or lujvo.
 * 
 * @param aString Some string.
 * @param allowRNHyphens True if unnecessary r & n hyphens are allowed.
 * @param allowMZ True if mz is a valid consonant cluster.
 * @returns Return true if the string is a valid gismu or lujvo.
 */
function isGismuOrLujvo(
  aString: string, 
  {
    allowRNHyphens = false, 
    allowMZ = false
  } = {}
): boolean {
  if (aString.length < 5)
    return false;
  if (!isVowel(aString.slice(-1)))
    return false;
  
  if (isGismu(aString, allowMZ))
    return true;

  try {
    jvokaha(
      aString, 
      {
        allowRNHyphens: allowRNHyphens, 
        yHyphens: YHyphenSetting.STANDARD, 
        allowMZ: allowMZ
      });
  } catch (e) {
    if (e instanceof DecompositionError || e instanceof InvalidClusterError)
      return false;
    else
      throw e;
  }

  return true;
}

/**
 * Check if string is not a valid word because a leading CV cmavo
 * would combine to create a lujvo. (slinku'i)
 * 
 * @param aString A string.
 * @param allowMZ True if mz is a valid consonant cluster.
 * @returns True if string fails slinku'i test.
 */
function isSlinkuhi(aString: string, allowMZ = false) {
  try {
    jvokaha("to" + aString, { allowRNHyphens: true, allowMZ: allowMZ });
    return true;
  } catch (e) {
    if (e instanceof DecompositionError || e instanceof InvalidClusterError)
      return false;
    else
      throw e;
  }
}

/**
 * Check morphology rules that are specific to zi'evla and
 * experimental rafsi shapes.
 *
 * Caution: May return ZIhEVLA when string is not valid word.
 * 
 * @param valsi A string.
 * @param requireZihevla True if rafsi-shapes should raise an Error.
 * @param expRafsiShapes True if experimental rafsi shapes are allowed.
 * @param allowMZ True if mz is a valid consonant cluster.
 * @returns ZIhEVLA or RAFSI if string passes tests.
 */
function checkZihevlaOrRafsi(
  valsi: string,
  {
    requireZihevla = false,
    expRafsiShapes = false,
    allowMZ = false
  } = {}
): BrivlaType {
  const valsiCopy = valsi;

  if (requireZihevla && valsi.length < 4)
    throw new NotZihevlaError("too short to be zi'evla: {" + valsiCopy + "}");

  let chunk = "";
  let pos = 0;
  let numSyllables = 0;
  let clusterPos: number | null = null;
  let numConsonants = 0;
  let finalConsonantPos = 0

  while (valsi.length > 0) {
    if (isConsonant(valsi[0])) {
      while (valsi.length > 0 && isConsonant(valsi[0])) {
        chunk += valsi[0];
        valsi = valsi.slice(1);
      }

      if (chunk.length >= 2 && clusterPos === null) {
        if (numConsonants > 1)
          throw new NotZihevlaError("too many consonants before first cluster: {" + valsiCopy + "}");
        clusterPos = pos;
      }

      if (numSyllables === 0 && chunk.length >= 2) {
        if (!INITIAL.includes(chunk.slice(0, 2)))
          throw new NotZihevlaError("invalid word initial: {" + valsiCopy + "}");
      }

      for (let i = 0; i < chunk.length - 1; i++) {
        const cluster = chunk.slice(i, i+2);
        if (!(allowMZ ? MZ_VALID : VALID).includes(cluster))
          throw new NotZihevlaError("invalid cluster {" + cluster + "} in word {" + valsiCopy + "}");
      }

      for (let i = 0; i < chunk.length - 2; i++) {
        const cluster = chunk.slice(i, i+3);
        if (BANNED_TRIPLES.includes(cluster))
          throw new NotZihevlaError("banned triple {" + cluster + "} in word {" + valsiCopy + "}");
      }

      if (pos === 0) {
        if (!isZihevlaInitialCluster(chunk))
          throw new NotZihevlaError("invalid zi'evla initial cluster {" + chunk + "} in word {" + valsiCopy + "}");
      } else {
        if (!isZihevlaMiddleCluster(chunk))
          throw new NotZihevlaError("invalid zi'evla middle cluster {" + chunk + "} in word {" + valsiCopy + "}");
      }

      finalConsonantPos = pos;
      numConsonants += chunk.length;

    } else if (isVowel(valsi[0])) {
      while (valsi.length > 0 && isVowel(valsi[0])) {
        chunk += valsi[0];
        valsi = valsi.slice(1);
      }

      if (pos === 0) {
        if (START_VOWEL_CLUSTERS.includes(chunk) || FOLLOW_VOWEL_CLUSTERS.includes(chunk))
          numSyllables += 1;
        else
          throw new NotZihevlaError("starts with bad vowels: {" + valsiCopy + "}");

      } else {
        try {
          numSyllables += splitVowelCluster(chunk).length;
        } catch (e) {
          if (e instanceof DecompositionError)
            throw new NotZihevlaError(`vowel decomp error: {${chunk}} in {${valsiCopy}}`);
          else
            throw e;
        }
      }

    } else if (valsi[0] === "'") {
      chunk = "'";
      valsi = valsi.slice(1);

      if (pos < 1 || !isVowel(valsiCopy[pos-1]))
        throw new NotZihevlaError("' not preceded by vowel");
      if (valsi.length === 0 || !isVowel(valsiCopy[pos+1]))
        throw new NotZihevlaError("' not followed by vowel");

    } else {
      throw new NotZihevlaError("unexpected character {" + valsi[0] + "} in {" + valsiCopy + "}");
    }

    pos += chunk.length;
    chunk = "";
  }

  if (numSyllables < 2 && (requireZihevla || !expRafsiShapes)) {
    throw new NotZihevlaError("too few syllables: {" + valsiCopy + "}");

  } else if (numSyllables > 2) {
    if (clusterPos !== null && clusterPos > 0) {
      if (isBrivla(valsiCopy.slice(clusterPos)))
        throw new NotZihevlaError(`falls apart at cluster: {${valsiCopy.slice(0, clusterPos)}_${valsiCopy.slice(clusterPos)}}`);

      for (let i = 0; i < clusterPos; i++) {
        if (isConsonant(valsiCopy[clusterPos-i]) && isBrivla(valsiCopy.slice(clusterPos-i)))
          throw new NotZihevlaError(`falls apart before cluster: {${valsiCopy.slice(0, clusterPos-i)}_${valsiCopy.slice(clusterPos-i)}}`);
      }
    }
  }

  if (clusterPos === null) {
    if (requireZihevla)
      throw new NotZihevlaError(`no cluster: {${valsiCopy}}`);
    if (!isConsonant(valsiCopy[0]) && !expRafsiShapes)
      throw new NotZihevlaError(`not valid rafsi shape: {${valsiCopy}}`);
    if (numConsonants > 1)
      throw new NotZihevlaError(`too many consonants without cluster: {${valsiCopy}}`);
    if (finalConsonantPos > 0)
      throw new NotZihevlaError(`non-initial consonant(s) without cluster: {${valsiCopy}}`);

  } else {
    if (!(isVowel(valsiCopy[0]) && isConsonant(valsiCopy[1]))) {
      if (isSlinkuhi(valsiCopy, allowMZ))
        throw new NotZihevlaError(`slinku'i: {to,${valsiCopy}}`);
    }
  }

  return clusterPos === null ? BrivlaType.RAFSI : BrivlaType.ZIhEVLA;
}

/**
 * Check if string is a valid lojban brivla.
 * 
 * @param valsi A string.
 * @param yHyphens Which y-hyphen rules to use.
 * @param expRafsiShapes True if experimental rafsi shapes are allowed.
 * @param consonants Which consonant rules to use.
 * @param glides True if glides count as consonants.
 * @param allowMZ True if mz is a valid consonant cluster.
 * @returns True if string is a valid lojban brivla.
 */
function isBrivla(
  valsi: string, 
  {
    yHyphens = YHyphenSetting.STANDARD,
    expRafsiShapes = false,
    consonants = ConsonantSetting.CLUSTER,
    glides = false,
    allowMZ = false
  } = {}
): boolean {
  try {
    const bType = analyseBrivla(valsi, {yHyphens, expRafsiShapes, consonants, glides, allowMZ})[0];
    return bType !== BrivlaType.CMEVLA;
  } catch (e) {
    if (e instanceof NotBrivlaError)
      return false;
    else
      throw e;
  }
}

/**
 * Tells you the type and decomposition of any brivla or decomposable
 * cmevla.
 * Raises an error for single-unit cmevla because it doesn't check the
 * cmevla morphology rules.
 * 
 * @param valsi A string.
 * @param yHyphens Which y-hyphen rules to use.
 * @param expRafsiShapes True if experimental rafsi shapes are allowed.
 * @param consonants Which consonant rules to use.
 * @param glides True if glides count as consonants.
 * @param allowMZ True if mz is a valid consonant cluster.
 * @returns The word type and a list of pieces (rafsi + hyphens).
 */
function analyseBrivla(
  valsi: string, 
  {
    yHyphens = YHyphenSetting.STANDARD,
    expRafsiShapes = false,
    consonants = ConsonantSetting.CLUSTER,
    glides = false,
    allowMZ = false
  } = {}
): [BrivlaType, string[]] {
  valsi = normalise(valsi);

  let isCmevlatai = false;
  if (valsi.length === 0)
    throw new NotBrivlaError("empty string");
  else if (isConsonant(valsi.slice(-1)))
    isCmevlatai = true;
  else if (!isVowel(valsi.slice(-1)))
    throw new NotBrivlaError(`doesn't end in consonant or vowel: {${valsi}}`);

  if (isCmevlatai) {
    if (isGismu(valsi + "a", allowMZ))
      throw new NotBrivlaError(`non-decomposable cmevla: {${valsi}}`);
  } else {
    if (isGismu(valsi, allowMZ))
      return [BrivlaType.GISMU, [valsi]];
  }

  try {
    const allowRN = yHyphens !== YHyphenSetting.FORCE_Y;
    const resultParts = jvokaha(valsi, {allowRNHyphens: allowRN, yHyphens: yHyphens, consonants: consonants, glides: glides, allowMZ: allowMZ});
    return [isCmevlatai ? BrivlaType.CMEVLA : BrivlaType.LUJVO, resultParts];
  } catch (e) {
    if (!(e instanceof DecompositionError || e instanceof InvalidClusterError || e instanceof TypeError))
      throw e;
  }

  if (!(isVowel(valsi[0]) || isConsonant(valsi[0])))
    throw new NotBrivlaError(`doesn't start with vowel or consonant: {${valsi}}`);

  const yParts = valsi.split("y");

  if (yParts.length === 1) {
    if (isCmevlatai)
      throw new NotBrivlaError(`non-decomposable cmevla: {${valsi}}`);

    try {
      checkZihevlaOrRafsi(valsi, {requireZihevla: true, expRafsiShapes: expRafsiShapes, allowMZ: allowMZ});
      return [BrivlaType.ZIhEVLA, [valsi]];
    } catch (e) {
      if (e instanceof NotZihevlaError)
        throw new NotBrivlaError(`no hyphens, and not valid zi'evla: {${valsi}}`);
      else
        throw e;
    }
  }

  let resultParts: string[] = [];
  let nextHyphen = "";
  let hasCluster = false;
  let isMahortai = true;
  let consonantBeforeBreak = false;
  let numConsonants = 0;

  for (let i = 0; i < yParts.length; i++) {
    if (i !== 0)
      nextHyphen += "y";
    let part = yParts[i];
    let partCopy = part;

    if (part.length === 0)
      throw new NotBrivlaError("double y");

    if (part[0] === "'") {
      part = part.slice(1);
      partCopy = part;
      nextHyphen += "'";

      if (part.length === 0)
        throw new NotBrivlaError("that was only a '");
      if (!(isVowel(part[0]) && !isGlide(part)))
        throw new NotBrivlaError(`consonant or glide after ': {${part}}`);
    } else if (i > 0 && isVowel(part[0]) && !isGlide(part)) {
      throw new NotBrivlaError(`non-glide vowel after y: {${part}}`);
    }
    if (nextHyphen.length > 0) {
      resultParts.push(nextHyphen);
      nextHyphen = "";
    }

    if (rafsiTarmi(part) === Tarmi.CVC) {
      resultParts.push(part);
      consonantBeforeBreak = true;
      numConsonants += 2;
      continue;
    }
    if (rafsiTarmi(part + "a") === Tarmi.CCV)
      throw new NotBrivlaError("can't drop vowel on CCV rafsi");

    if (i > 0 && (isConsonant(part[0]) || isGlide(part)))
      isMahortai = false;
    if (consonantBeforeBreak && (isConsonant(part[0]) || (glides && isGlide(part))))
      hasCluster = true;

    let canBeRafsi = true;
    let requireCluster = false;
    let didAddA = false;

    if (part.slice(-1) === "'") {
      if (yHyphens === YHyphenSetting.STANDARD && !hasCluster && i < yParts.length - 1 && yParts[i+1][0] !== "'")
        requireCluster = true;
      part = part.slice(0, -1);
      partCopy = part;
      nextHyphen += "'";

      if (!isVowel(part.slice(-1)))
        throw new NotBrivlaError(`non-vowel before ': {${part}}`);
    } else if (i < yParts.length - 1 || isCmevlatai) {
      if (isVowel(part.slice(-1)))
        canBeRafsi = false;
      part = part + "a";
      didAddA = true;
      requireCluster = true;
    }

    let didKaha = false;
    if (canBeRafsi) {
      let foundParts = [part]
      try {
        foundParts = jvokaha2(partCopy, {yHyphens: yHyphens, allowMZ: allowMZ});
        if (foundParts.length < 2 && !isValidRafsi(foundParts[0], allowMZ=allowMZ))
          throw new NotBrivlaError(`invalid rafsi: {${foundParts[0]}}`);
        resultParts = resultParts.concat(foundParts);
        didKaha = true;
      } catch (e) {
        if (!(e instanceof DecompositionError || e instanceof InvalidClusterError || e instanceof TypeError))
          throw e;
      }

      foundParts.forEach((partPart) => {
        const raftai = rafsiTarmi(partPart);
        if ([Tarmi.CVV, Tarmi.CVhV].includes(raftai)) {
          numConsonants += 1;
        } else if (raftai !== Tarmi.OtherRafsi) {
          numConsonants += 2;
          hasCluster = true;
        }
      })
    }

    if (didKaha) {
      if ([Tarmi.CVV, Tarmi.CVhV].includes(rafsiTarmi(part))) {
        if (
          requireCluster && !hasCluster 
          && (yHyphens === YHyphenSetting.STANDARD 
            || !(i === yParts.length - 2 && [Tarmi.CVV, Tarmi.CCV].includes(rafsiTarmi(yParts[1]))))
        ) {
          throw new NotBrivlaError("falls off because y");
        }
      }

      if (i === 0) {
        let smabruPart = "";
        if (rafsiTarmi(part.slice(0, 4)) === Tarmi.CVhV)
          smabruPart = part.slice(4);
        else if (rafsiTarmi(part.slice(0, 3)) === Tarmi.CVV)
          smabruPart = part.slice(3);
        else if (isConsonant(part[0]) && isVowel(part[1]))
          smabruPart = part.slice(2);

        if (smabruPart.length > 0) {
          if (didAddA)
            smabruPart = smabruPart.slice(0, -1);
          else
            smabruPart = stripHyphens(smabruPart);

          if (isValidRafsi(smabruPart))
            throw new NotBrivlaError("tosmabru");

          try {
            jvokaha(smabruPart, {allowMZ: allowMZ});
            throw new NotBrivlaError("tosmabru");
          } catch (e) {
            if (!(e instanceof DecompositionError || e instanceof InvalidClusterError))
              throw e;
          }
        }
      }

    } else {
      const requireZihevla = requireCluster || !expRafsiShapes;
      let shapeType;
      try {
        shapeType = checkZihevlaOrRafsi(
          part,
          {
            requireZihevla: requireZihevla,
            expRafsiShapes: expRafsiShapes,
            allowMZ: allowMZ
          }
        )
      } catch (e) {
        if (e instanceof NotZihevlaError)
          throw new NotBrivlaError(e.message);
        else
          throw e;
      }

      if (shapeType == BrivlaType.ZIhEVLA)
        hasCluster = true;

      if (isConsonant(part[0]) || (glides && isGlide(part)))
        numConsonants += 1;

      resultParts.push(partCopy);
    }

    consonantBeforeBreak = false;
  }

  if (!hasCluster) {
    if (consonants === ConsonantSetting.CLUSTER)
      throw new NotBrivlaError("no clusters");
    else if (consonants === ConsonantSetting.TWO_CONSONANTS && numConsonants < 2)
      throw new NotBrivlaError("not enough consonants");
    else if (consonants === ConsonantSetting.ONE_CONSONANT && numConsonants < 1)
      throw new NotBrivlaError("not enough consonants");
    else if (isMahortai)
      throw new NotBrivlaError("cmavo shaped or maybe multiple cmavo shaped");
  }

  if (!(isVowel(valsi[0]) && (isConsonant(valsi[1]) || valsi[1] == "y"))) {
    if (isSlinkuhi(valsi, allowMZ))
      throw new NotBrivlaError(`slinku'i: {to,${valsi}}`);
  }

  return [isCmevlatai ? BrivlaType.CMEVLA : BrivlaType.EXTENDED_LUJVO, resultParts];
}

/**
 * Create a list of start and end indices for each rafsi in a list of rafsi
 * and hyphens (a word split into its components).
 *
 * Example:
 * ["tcan", "y", "ja'a"] ->
 * [(0, 4), (5, 9)]
 * 
 * @param rafsiList List of rafsi and hyphens (a decomposed word).
 * @returns List of start and end indices for non-hyphen components.
 */
function getRafsiIndices(rafsiList: string[]): [number, number][] {
  let position = 0;
  const indexList: [number, number][] = [];
  rafsiList.forEach((piece) => {
    if (!HYPHENS.includes(piece))
      indexList.push([position, position + piece.length]);
    position += piece.length;
  })
  return indexList;
}

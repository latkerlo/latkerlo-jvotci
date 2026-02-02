/*
Copyright (c) 2023 Toni Brown (https://codeberg.org/tb148)
Licensed under the Apache License, Version 2.0

Modified by latkerlo (https://github.com/latkerlo), Copyright (c) 2023-2024
*/
var TosyType;
(function (TosyType) {
    TosyType[TosyType["Tosynone"] = 0] = "Tosynone";
    TosyType[TosyType["Tosmabru"] = 1] = "Tosmabru";
    TosyType[TosyType["Tosyhuhu"] = 2] = "Tosyhuhu";
})(TosyType || (TosyType = {}));
/**
 * Calculate the lujvo score for a rafsi.
 *
 * @param rafsi A rafsi, possibly including a hyphen.
 * @returns The lujvo score for the rafsi (+hyphen).
 */
function score(rafsi) {
    let tarmiScore = tarmiIgnoringHyphen(rafsi);
    if (tarmiScore == Tarmi.OtherRafsi)
        tarmiScore = 0;
    return (1000 * rafsi.length
        + 100 * (rafsi.match(/y/g) || []).length
        - 10 * tarmiScore
        - (rafsi.match(/[aeiou]/g) || []).length);
}
function tiebreak(lujvo) {
    return +(rafsiTarmi(lujvo.slice(0, 3)) == Tarmi.CVV && [Tarmi.CCV, Tarmi.CCVC, Tarmi.CVC, Tarmi.CVCC].includes(rafsiTarmi(lujvo.slice(3))));
}
/**
 * Create a cleaned-up list of tanru components from a string or list.
 *
 * @param tanru A tanru string or list.
 * @returns A list of normalised tanru components.
 */
function processTanru(tanru) {
    let valsiList;
    if (typeof tanru === 'string')
        valsiList = tanru.trim().split(/\s+/);
    else if (Array.isArray(tanru))
        valsiList = tanru;
    else
        throw new TypeError(`Cannot make lujvo from type ${typeof (tanru)}`);
    valsiList = valsiList.map(normalise);
    return valsiList;
}
/**
 * Create list of possible rafsi + hyphen forms for a rafsi.
 *
 * @param rafsi The rafsi to use.
 * @param rType The rafsi's form.
 * @param isFirst True if the rafsi is the first tanru component.
 * @param isLast True if the rafsi is the last tanru component.
 * @param consonants Which consonant rules to use.
 * @param glides True if glides count as consonants.
 * @returns List of possibly rafsi+hyphen forms.
 */
function getRafsiForRafsi(rafsi, rType, isFirst, isLast, consonants, // TODO: defaults?
glides) {
    const result = [];
    if (!isFirst && isVowel(rafsi[0]) && !isGlide(rafsi))
        rafsi = "'" + rafsi;
    if (["SHORT BRIVLA", Tarmi.CCVC, Tarmi.CVCC].includes(rType)) {
        if (!isLast)
            result.push([rafsi + "y", 2]);
        else if (!isVowel(rafsi.slice(-1)))
            result.push([rafsi, 2]);
    }
    else if (["LONG BRIVLA", Tarmi.CCVCV, Tarmi.CVCCV].includes(rType)) {
        if (isLast)
            result.push([rafsi, 2]);
        else
            result.push([rafsi + "'y", 2]);
    }
    else if (rType === "EXPERIMENTAL RAFSI") {
        let numConsonants = 0;
        if (consonants !== ConsonantSetting.CLUSTER && (isConsonant(rafsi[0]) || (glides && isGlide(rafsi))))
            numConsonants = 1;
        if (isLast)
            result.push([rafsi, numConsonants]);
        else if (!isFirst)
            result.push([rafsi + "'y", numConsonants]);
        else
            result.push([rafsi + "'", numConsonants]);
    }
    else if (rType === Tarmi.CVV || rType === Tarmi.CVhV) {
        const numConsonants = consonants === ConsonantSetting.CLUSTER ? 0 : 1;
        if (isFirst)
            result.push([rafsi + "'", numConsonants]);
        else if (!isLast)
            result.push([rafsi + "'y", numConsonants]);
        result.push([rafsi, numConsonants]);
    }
    else if (rType === Tarmi.CCV) {
        result.push([rafsi, 2]);
        result.push([rafsi + "'y", 2]);
    }
    else if (rType === Tarmi.CVC) {
        result.push([rafsi, 2]);
        if (!isLast)
            result.push([rafsi + "y", 2]);
    }
    else {
        throw Error(`Unrecognised rafsi type: ${rType}`);
    }
    return result;
}
/**
 * Create list of rafsi lists for each valsi in valsi_list.
 *
 * @param valsiList List of valsi to find rafsi for.
 * @param yHyphens Which y-hyphen rules to use.
 * @param expRafsiShapes True if experimental rafsi shapes are allowed.
 * @param consonants Which consonant rules to use.
 * @param glides True if glides count as consonants.
 * @param allowMZ True if mz is a valid consonant cluster.
 * @returns List of rafsi lists.
 */
function getRafsiListList(valsiList, { yHyphens = YHyphenSetting.STANDARD, expRafsiShapes = false, consonants = ConsonantSetting.CLUSTER, glides = false, allowMZ = false } = {}) {
    const listOfRafsiLists = [];
    for (let [i, valsi] of valsiList.entries()) {
        let rafsiList = [];
        const isFirst = i === 0;
        const isLast = i === valsiList.length - 1;
        if (valsi.slice(-1) === "-") {
            const isShortBrivla = valsi[0] !== "-";
            valsi = valsi.replace(/^\-+|\-+$/g, '');
            if (!isOnlyLojbanCharacters(valsi))
                throw new NonLojbanCharacterError("Non-lojban character in {" + valsi + "}");
            if (valsi.slice(-1) === "'")
                throw new NonLojbanCharacterError("Rafsi cannot end with ': {" + valsi + "}");
            if (isShortBrivla) {
                let bType;
                try {
                    [bType,] = analyseBrivla(valsi + "a", {
                        yHyphens: yHyphens,
                        expRafsiShapes: expRafsiShapes,
                        allowMZ: allowMZ
                    });
                }
                catch (e) {
                    if (e instanceof NotBrivlaError)
                        throw new NoLujvoFoundError(`rafsi + a is not a brivla: {${valsi}}`);
                    else
                        throw e;
                }
                if (![BrivlaType.ZIhEVLA, BrivlaType.GISMU].includes(bType))
                    throw new NoLujvoFoundError(`rafsi + a is not a gismu or zi'evla: {${valsi}}`);
                if (valsi.length >= 6 && isConsonant(valsi.slice(-1))) {
                    let doesDecompose = true;
                    try {
                        jvokaha2(valsi, { yHyphens: yHyphens, allowMZ: allowMZ });
                    }
                    catch (e) {
                        if (e instanceof DecompositionError || e instanceof InvalidClusterError)
                            doesDecompose = false; // TODO: missing IndexError
                        else
                            throw e;
                    }
                    if (doesDecompose)
                        throw new NoLujvoFoundError(`short zi'evla rafsi falls apart: {${valsi}}`);
                }
                rafsiList = rafsiList.concat(getRafsiForRafsi(valsi, "SHORT BRIVLA", isFirst, isLast, consonants, glides));
            }
            else {
                const raftai = rafsiTarmi(valsi);
                if (raftai === Tarmi.OtherRafsi) {
                    let zihevlaOrRafsi;
                    try {
                        const [bType,] = analyseBrivla(valsi, {
                            yHyphens: yHyphens,
                            expRafsiShapes: expRafsiShapes,
                            allowMZ: allowMZ
                        });
                        if (bType === BrivlaType.ZIhEVLA)
                            zihevlaOrRafsi = BrivlaType.ZIhEVLA;
                    }
                    catch (e) {
                        if (!(e instanceof NotBrivlaError))
                            throw e;
                        if (expRafsiShapes) {
                            try {
                                const shape = checkZihevlaOrRafsi(valsi, {
                                    yHyphens: yHyphens,
                                    expRafsiShapes: expRafsiShapes,
                                    allowMZ: allowMZ
                                });
                                if (shape === BrivlaType.RAFSI)
                                    zihevlaOrRafsi = BrivlaType.RAFSI;
                            }
                            catch (e) {
                                if (e instanceof NotZihevlaError)
                                    throw new NoLujvoFoundError(`Not a valid rafsi shape: -${valsi}-`);
                                else
                                    throw e;
                            }
                        }
                    }
                    if (zihevlaOrRafsi === undefined)
                        throw new NotZihevlaError(`Not a valid rafsi or zi'evla shape: -${valsi}-`);
                    const rType = zihevlaOrRafsi === BrivlaType.ZIhEVLA ? "LONG BRIVLA" : "EXPERIMENTAL RAFSI";
                    rafsiList = rafsiList.concat(getRafsiForRafsi(valsi, rType, isFirst, isLast, consonants, glides));
                }
                else {
                    if (!isValidRafsi(valsi, allowMZ = allowMZ))
                        throw new InvalidClusterError("Invalid cluster in rafsi: -{" + valsi + "}-");
                    rafsiList = rafsiList.concat(getRafsiForRafsi(valsi, raftai, isFirst, isLast, consonants, glides));
                }
            }
        }
        else {
            if (!isOnlyLojbanCharacters(valsi))
                throw new NonLojbanCharacterError("Non-lojban character in {" + valsi + "}");
            const shortRafsiList = RAFSI_LIST.get(valsi);
            if (shortRafsiList !== undefined) {
                shortRafsiList.forEach(shortRafsi => {
                    const raftai = rafsiTarmi(shortRafsi);
                    if (raftai === Tarmi.OtherRafsi && !expRafsiShapes)
                        return;
                    rafsiList = rafsiList.concat(getRafsiForRafsi(shortRafsi, raftai, isFirst, isLast, consonants, glides));
                });
            }
            let bType;
            try {
                bType = analyseBrivla(valsi, {
                    yHyphens: yHyphens,
                    expRafsiShapes: expRafsiShapes,
                    allowMZ: allowMZ
                })[0];
            }
            catch (e) {
                if (!(e instanceof NotBrivlaError))
                    throw e;
            }
            if (bType === BrivlaType.GISMU)
                rafsiList = rafsiList.concat(getRafsiForRafsi(valsi.slice(0, -1), "SHORT BRIVLA", isFirst, isLast, consonants, glides));
            if (bType === BrivlaType.GISMU || bType === BrivlaType.ZIhEVLA)
                rafsiList = rafsiList.concat(getRafsiForRafsi(valsi, "LONG BRIVLA", isFirst, isLast, consonants, glides));
        }
        listOfRafsiLists.push(rafsiList);
    }
    return listOfRafsiLists;
}
/**
 * Add one rafsi to the end of the current lujvo (if possible) and
 * calculate the score.
 *
 * @param lujvo The current working lujvo.
 * @param rafsi The rafsi to add.
 * @param lujvoConsonants Number of consonants in the lujvo.
 * @param rafsiConsonants Number of consonants in the rafsi.
 * @param lujvoScore Current score of the lujvo.
 * @param indexList List of rafsi start/end indices.
 * @param tosmabruType Which way the lujvo could still fall apart.
 * @param generateCmevla True if final result should end in
    consonant.
 * @param tanruLen Number of components in the tanru.
 * @param yHyphens Which y-hyphen rules to use.
 * @param consonants Which consonant rules to use.
 * @param glides True if glides count as consonants.
 * @param allowMZ True if mz is a valid consonant cluster.
 * @returns Final tosmabru_type, num_consonants, score, and lujvo.
 */
function combine(lujvo, rafsi, lujvoConsonants, rafsiConsonants, lujvoScore, indexList, tosmabruType, generateCmevla, tanruLen = 0, yHyphens = YHyphenSetting.STANDARD, consonants = ConsonantSetting.CLUSTER, glides = false, allowMZ = false) {
    const lujvoFinal = lujvo.slice(-1);
    const rafsiInitial = rafsi[0];
    if (isConsonant(lujvoFinal)
        && isConsonant(rafsiInitial)
        && !(allowMZ ? MZ_VALID : VALID).includes(lujvoFinal + rafsiInitial)) {
        return null;
    }
    if (BANNED_TRIPLES.includes(lujvoFinal + rafsi.slice(0, 2))) {
        return null;
    }
    const raftai1 = tarmiIgnoringHyphen(rafsi);
    if (!"y'".includes(lujvoFinal) && raftai1 === Tarmi.OtherRafsi)
        return null;
    let prulamrafsi = lujvo.slice(indexList.slice(-1)[0][0], indexList.slice(-1)[0][1]);
    if ([Tarmi.CVV, Tarmi.CVC].includes(rafsiTarmi(prulamrafsi))
        && /^[aeiou']+$/.test(rafsi)
        && (rafsi[0] == "'"
            || isGlide(rafsi) && !glides && consonants == ConsonantSetting.CLUSTER))
        return null;
    let hyphen = "";
    if (lujvoFinal === "'") {
        if (rafsiInitial === "'" || yHyphens !== YHyphenSetting.STANDARD)
            hyphen = "y";
        else
            return null;
    }
    else if (lujvo.length == 5 && rafsiTarmi(lujvo.slice(0, 3)) == Tarmi.CCV && lujvo.slice(3) == "'y") {
        return null;
    }
    else if (lujvo.length <= 5 && !generateCmevla) {
        const raftai0 = tarmiIgnoringHyphen(lujvo);
        if ([Tarmi.CVhV, Tarmi.CVV].includes(raftai0)) {
            if (yHyphens === YHyphenSetting.FORCE_Y)
                hyphen = "'y";
            else if (rafsiInitial === "r")
                hyphen = "n";
            else
                hyphen = "r";
        }
        if (tanruLen === 2 && raftai1 === Tarmi.CCV)
            hyphen = "";
    }
    if (tosmabruType === TosyType.Tosmabru) {
        if (!INITIAL.includes(lujvoFinal + rafsiInitial)) {
            tosmabruType = TosyType.Tosynone;
        }
        else if (raftai1 === Tarmi.CVCCV) {
            if (INITIAL.includes(rafsi.slice(2, 4))) {
                return null;
            }
            tosmabruType = TosyType.Tosynone;
        }
        else if (raftai1 === Tarmi.CVC) {
            if (rafsi.slice(-1) === "y") {
                return null;
            }
        }
        else {
            tosmabruType = TosyType.Tosynone;
        }
    }
    else if (tosmabruType === TosyType.Tosyhuhu) {
        if (rafsiInitial !== "'" || containsConsonant(rafsi))
            tosmabruType = TosyType.Tosynone;
    }
    const rafsiStart = lujvo.length + hyphen.length + (rafsi[0] === "'" ? 1 : 0);
    const rafsiEnd = rafsiStart + stripHyphens(rafsi).length;
    indexList = indexList.concat([[rafsiStart, rafsiEnd]]);
    let newConsonants = rafsiConsonants;
    if (hyphen.length > 0 && "nr".includes(hyphen)) {
        newConsonants = 2;
    }
    else if (consonants == ConsonantSetting.CLUSTER) {
        if (rafsiConsonants !== 2) {
            let i = lujvo.length - 1;
            while ("'y".includes(lujvo[i]))
                i -= 1;
            let j = 0;
            while (rafsi[j] === "'")
                j += 1;
            if (isConsonant(lujvo[i]) && (isConsonant(rafsi[j]) || (glides && isGlide(rafsi.slice(j)))))
                newConsonants = 2;
            else
                newConsonants = 0;
        }
    }
    let totalConsonants = Math.min(2, lujvoConsonants + newConsonants);
    if (consonants === ConsonantSetting.ONE_CONSONANT && totalConsonants > 0)
        totalConsonants = 2;
    return [
        tosmabruType,
        totalConsonants,
        lujvoScore + score(hyphen) + score(rafsi) - tiebreak(lujvo + hyphen + rafsi),
        lujvo + hyphen + rafsi,
        indexList
    ];
}
/**
 * Add the candidate to current_best if it is the best.
 *
 * @param candidate The new candidate lujvo.
 * @param currentBest The list of existing best candidates for each
    combination of tosmabru type and number of consonants.
 */
function updateCurrentBest(candidate, currentBest) {
    if (candidate == null)
        return;
    const [tosmabruType, numConsonants, resScore, resLujvo, resIndexList] = candidate;
    const lujvoFinal = resLujvo.slice(-1);
    if (!currentBest[tosmabruType][numConsonants].has(lujvoFinal) ||
        currentBest[tosmabruType][numConsonants].get(lujvoFinal)[1] > resScore) {
        currentBest[tosmabruType][numConsonants].set(lujvoFinal, [resLujvo, resScore, resIndexList]);
    }
}
/**
 * Create the best lujvo for the given tanru (normalised list). Also get its
 * score and rafsi index list.
 *
 * @param valsiList A pre-normalised list of tanru component strings.
 * @param generateCmevla True if result should end in a consonant.
 * @param yHyphens Which y-hyphen rules to use.
 * @param expRafsiShapes True if experimental rafsi shapes are allowed.
 * @param consonants Which consonant rules to use.
 * @param glides True if glides count as consonants.
 * @param allowMZ True if mz is a valid consonant cluster.
 * @returns The best lujvo, its score, and list of rafsi start/end indices.
 */
function getLujvoFromList(valsiList, { generateCmevla = false, yHyphens = YHyphenSetting.STANDARD, expRafsiShapes = false, consonants = ConsonantSetting.CLUSTER, glides = false, allowMZ = false } = {}) {
    const rafsiListList = getRafsiListList(valsiList, { yHyphens, expRafsiShapes, consonants, glides, allowMZ });
    let currentBest = [[new Map(), new Map(), new Map()], [new Map(), new Map(), new Map()], [new Map(), new Map(), new Map()]];
    rafsiListList[0].forEach((rafsi0) => {
        rafsiListList[1].forEach((rafsi1) => {
            let tosmabruType = TosyType.Tosynone;
            if (tarmiIgnoringHyphen(rafsi0[0]) === Tarmi.CVC && !generateCmevla)
                tosmabruType = rafsi0[0].slice(-1) === "y" ? TosyType.Tosyhuhu : TosyType.Tosmabru;
            const result = combine(rafsi0[0], rafsi1[0], rafsi0[1], // TODO: Bring back numConsonants. This is confusing.
            rafsi1[1], score(rafsi0[0]), [[0, stripHyphens(rafsi0[0]).length]], tosmabruType, generateCmevla, rafsiListList.length, yHyphens = yHyphens, consonants = consonants, glides = glides, allowMZ = allowMZ);
            updateCurrentBest(result, currentBest);
        });
    });
    let previousBest = currentBest;
    rafsiListList.slice(2).forEach((rafsiList) => {
        currentBest = [[new Map(), new Map(), new Map()], [new Map(), new Map(), new Map()], [new Map(), new Map(), new Map()]];
        ;
        rafsiList.forEach((rafsi) => {
            for (let tosmabruType = 0; tosmabruType < 3; tosmabruType++) {
                for (let numConsonants = 0; numConsonants < 3; numConsonants++) {
                    previousBest[tosmabruType][numConsonants].forEach((lujvoAndScore, _) => {
                        const result = combine(lujvoAndScore[0], rafsi[0], numConsonants, rafsi[1], lujvoAndScore[1], lujvoAndScore[2], tosmabruType, generateCmevla, rafsiListList.length, yHyphens = yHyphens, consonants = consonants, glides = glides, allowMZ = allowMZ);
                        updateCurrentBest(result, currentBest);
                    });
                }
            }
        });
        previousBest = currentBest;
    });
    let bestLujvo = "";
    let bestScore = 10 ** 100;
    let bestIndexList = [];
    previousBest[0][2].forEach((lujvoAndScore, lerfu) => {
        if ((isVowel(lerfu) && !generateCmevla)
            || (isConsonant(lerfu) && generateCmevla)) {
            if (lujvoAndScore[1] < bestScore)
                [bestLujvo, bestScore, bestIndexList] = lujvoAndScore;
        }
    });
    if (bestLujvo === "")
        throw new NoLujvoFoundError("No lujvo found for {" + valsiList.join(" ") + "}");
    if (!generateCmevla && isSlinkuhi(bestLujvo, { yHyphens, allowMZ }))
        throw new NoLujvoFoundError(`{${valsiList.join(" ")}} can't be turned into a lujvo because it would produce a slinku'i, {${bestLujvo}}`);
    return [bestLujvo, bestScore, bestIndexList];
}
/**
 * Create the best lujvo for the given tanru (string or list). Also get its
 * score and rafsi index list.
 *
 * @param tanru A tanru in string or list form.
 * @param generateCmevla True if result should end in a consonant.
 * @param yHyphens Which y-hyphen rules to use.
 * @param expRafsiShapes True if experimental rafsi shapes are allowed.
 * @param consonants Which consonant rules to use.
 * @param glides True if glides count as consonants.
 * @param allowMZ True if mz is a valid consonant cluster.
 * @returns The best lujvo, its score, and list of rafsi start/end indices.
 */
function getLujvoWithAnalytics(tanru, { generateCmevla = false, yHyphens = YHyphenSetting.STANDARD, expRafsiShapes = false, consonants = ConsonantSetting.CLUSTER, glides = false, allowMZ = false } = {}) {
    return getLujvoFromList(processTanru(tanru), {
        generateCmevla,
        yHyphens,
        expRafsiShapes,
        consonants,
        glides,
        allowMZ
    });
}
/**
 * Create the best lujvo for the given tanru (string or list).
 *
 * @param tanru A tanru in string or list form.
 * @param generateCmevla True if result should end in a consonant.
 * @param yHyphens Which y-hyphen rules to use.
 * @param expRafsiShapes True if experimental rafsi shapes are allowed.
 * @param consonants Which consonant rules to use.
 * @param glides True if glides count as consonants.
 * @param allowMZ True if mz is a valid consonant cluster.
 * @returns The best lujvo.
 */
function getLujvo(tanru, { generateCmevla = false, yHyphens = YHyphenSetting.STANDARD, expRafsiShapes = false, consonants = ConsonantSetting.CLUSTER, glides = false, allowMZ = false } = {}) {
    return getLujvoFromList(processTanru(tanru), {
        generateCmevla,
        yHyphens,
        expRafsiShapes,
        consonants,
        glides,
        allowMZ
    })[0];
}

/*
Copyright (c) 2020 uakci (https://github.com/uakci)
Licensed under the MIT License

Modified by latkerlo (https://github.com/latkerlo), Copyright (c) 2023-2024
*/
var Tarmi;
(function (Tarmi) {
    Tarmi[Tarmi["Hyphen"] = 0] = "Hyphen";
    Tarmi[Tarmi["CVCCV"] = 1] = "CVCCV";
    Tarmi[Tarmi["CVCC"] = 2] = "CVCC";
    Tarmi[Tarmi["CCVCV"] = 3] = "CCVCV";
    Tarmi[Tarmi["CCVC"] = 4] = "CCVC";
    Tarmi[Tarmi["CVC"] = 5] = "CVC";
    Tarmi[Tarmi["CVhV"] = 6] = "CVhV";
    Tarmi[Tarmi["CCV"] = 7] = "CCV";
    Tarmi[Tarmi["CVV"] = 8] = "CVV";
    Tarmi[Tarmi["OtherRafsi"] = 9] = "OtherRafsi";
})(Tarmi || (Tarmi = {}));
const SONORANT_CONSONANTS = "lmnr";
var BrivlaType;
(function (BrivlaType) {
    BrivlaType["GISMU"] = "GISMU";
    BrivlaType["ZIhEVLA"] = "ZIhEVLA";
    BrivlaType["LUJVO"] = "LUJVO";
    BrivlaType["EXTENDED_LUJVO"] = "EXTENDED_LUJVO";
    BrivlaType["RAFSI"] = "RAFSI";
    BrivlaType["CMEVLA"] = "CMEVLA";
})(BrivlaType || (BrivlaType = {}));
var YHyphenSetting;
(function (YHyphenSetting) {
    YHyphenSetting["STANDARD"] = "STANDARD";
    YHyphenSetting["ALLOW_Y"] = "ALLOW_Y";
    YHyphenSetting["FORCE_Y"] = "FORCE_Y";
})(YHyphenSetting || (YHyphenSetting = {}));
var ConsonantSetting;
(function (ConsonantSetting) {
    ConsonantSetting["CLUSTER"] = "CLUSTER";
    ConsonantSetting["TWO_CONSONANTS"] = "TWO_CONSONANTS";
    ConsonantSetting["ONE_CONSONANT"] = "ONE_CONSONANT";
})(ConsonantSetting || (ConsonantSetting = {}));
const SETTINGS = [
    [YHyphenSetting.STANDARD, YHyphenSetting.ALLOW_Y, YHyphenSetting.FORCE_Y],
    [false, true],
    [ConsonantSetting.CLUSTER, ConsonantSetting.TWO_CONSONANTS, ConsonantSetting.ONE_CONSONANT],
    [false, true],
    [false, true] // mz is a valid cluster
];
/**
 * Returns an iterator that iterates through every possible combination
 * of settings. Only used for testing.
 *
 * @param settings An array of the array of possibilities for each setting.
 * @returns An iterator for each possible combination.
 */
function makeSettingsIterator(settings) {
    let index = 0;
    let possibilities = 1;
    settings.forEach((setting) => {
        possibilities *= setting.length;
    });
    const settingsIterator = {
        next() {
            let result;
            if (index < possibilities) {
                const item = [];
                let base = 1;
                settings.forEach((setting) => {
                    item.push(setting[Math.floor(index / base) % setting.length]);
                    base *= setting.length;
                });
                index += 1;
                result = { value: item, done: false };
                return result;
            }
            return { value: null, done: true };
        },
    };
    return settingsIterator;
}
/**
 * Return True if character is a vowel (aeiou).
 *
 * @param character Some character.
 * @returns True if it is a vowel.
 */
function isVowel(character) {
    return "aeiou".includes(character);
}
/**
 * Return True if character is a consonant (bcdfgjklmnprstvxz).
 *
 * @param character Some character.
 * @returns True if it is a consonant.
 */
function isConsonant(character) {
    return "bcdfgjklmnprstvxz".includes(character);
}
/**
 * Return true if string starts with an on-glide.
 *
 * @param aString String to check.
 * @returns True if string starts with an on-glide.
 */
function isGlide(aString) {
    if (aString.length < 2)
        return false;
    return "iu".includes(aString[0]) && isVowel(aString[1]);
}
/**
 * Return true if string is only lojban characters except y.
 *
 * @param aString Some string.
 * @returns True if it contains only lojban characters except y.
 */
function isOnlyLojbanCharacters(aString) {
    return /^[aeioubcdfgjklmnprstvxz']+$/.test(aString);
}
/**
 * Return true if at least one character is a lojban consonant.
 *
 * @param aString Some string.
 * @returns True if it contains a lojban consonant.
 */
function containsConsonant(aString) {
    for (const character of aString) {
        if (isConsonant(character))
            return true;
    }
    return false;
}
/**
 * Return true if valsi is shaped like CVCCV or CCVCV.
 * Does NOT have to be a valid gismu.
 *
 * @param valsi A word to check.
 * @returns True if valsi is gismu-shaped.
 */
function isGismuShape(valsi) {
    if (!(valsi.length === 5 && isConsonant(valsi[0]) && isConsonant(valsi[3]) && isVowel(valsi[4])))
        return false;
    if (isVowel(valsi[1]) && isConsonant(valsi[2]))
        return true;
    else if (isConsonant(valsi[1]) && isVowel(valsi[2]))
        return true;
    else
        return false;
}
/**
 * Check if valsi is a valid gismu.
 *
 * @param valsi Some word.
 * @param allowMZ True if mz is a valid consonant cluster.
 * @returns True if valid is a valid gismu.
 */
function isGismu(valsi, allowMZ = false) {
    if (!isGismuShape(valsi))
        return false;
    if (isVowel(valsi[1]))
        return (allowMZ ? MZ_VALID : VALID).includes(valsi.slice(2, 4));
    else
        return INITIAL.includes(valsi.slice(0, 2));
}
/**
 * Split vowel cluster into list of syllables.
 *
 * @param vowels A string of vowels.
 * @returns List of syllables in cluster.
 */
function splitVowelCluster(vowels) {
    function addToResult(newCluster) {
        const newVowels = vowels.slice(0, -newCluster.length);
        if (newCluster[0] === "i" && ["ai", "ei", "oi"].includes(newVowels.slice(-2)))
            throw new DecompositionError("Couldn't decompose: {" + vowelsCopy + "}");
        else if (newCluster[0] === "u" && newVowels.slice(-2) === "au")
            throw new DecompositionError("Couldn't decompose: {" + vowelsCopy + "}");
        result.unshift(newCluster);
    }
    const vowelsCopy = vowels;
    const result = [];
    while (true) {
        if (vowels.length > 3 && FOLLOW_VOWEL_CLUSTERS.includes(vowels.slice(-3))) {
            addToResult(vowels.slice(-3));
            vowels = vowels.slice(0, -3);
        }
        else if (vowels.length > 2 && FOLLOW_VOWEL_CLUSTERS.includes(vowels.slice(-2))) {
            addToResult(vowels.slice(-2));
            vowels = vowels.slice(0, -2);
        }
        else if (START_VOWEL_CLUSTERS.includes(vowels)) {
            result.unshift(vowels);
            return result;
        }
        else {
            throw new DecompositionError("Couldn't decompose {" + vowelsCopy + "}");
        }
    }
}
/**
 * Check if consonant cluster can start a zi'evla.
 *
 * @param cluster A consonant cluster.
 * @returns True if valid beginning for zi'evla.
 */
function isZihevlaInitialCluster(cluster) {
    if (cluster.length > 3) {
        return false;
    }
    else if (cluster.length === 3) {
        if (!INITIAL.includes(cluster.slice(0, 2)) || !ZIhEVLA_INITIAL.includes(cluster.slice(1)))
            return false;
    }
    else if (cluster.length == 2) {
        if (!INITIAL.includes(cluster))
            return false;
    }
    return true;
}
/**
 * Check if consonant cluster can be in a zi'evla.
 *
 * @param cluster A consonant cluster.
 * @returns True if valid in zi'evla.
 */
function isZihevlaMiddleCluster(cluster) {
    if (cluster.length === 3) {
        if (SONORANT_CONSONANTS.includes(cluster[1]))
            return true;
        return VALID.includes(cluster.slice(0, 2)) && INITIAL.includes(cluster.slice(1));
    }
    else if (cluster.length < 3) {
        return true; // Pairs are already checked outside this function
    }
    let match;
    // I tried to be smart, but it was too hard, so I used regex (badly)
    if (cluster.slice(-2, -1) === "m" && INITIAL.includes(cluster.slice(-2))) {
        if (isZihevlaInitialCluster(cluster.slice(-3)))
            match = cluster.slice(0, -3).match(/^([bcdfgjklmnprstvxz])?((?:[bcdfgjklmnprstvxz][lmnr])*)?$/);
        else
            match = cluster.slice(0, -2).match(/^([bcdfgjklmnprstvxz])?((?:[bcdfgjklmnprstvxz][lmnr])*)?$/);
    }
    else {
        match = cluster.match(/^([bcdfgjklmnprstvxz])?((?:[bcdfgjklmnprstvxz][lmnr])*)(?:([bcdfgjkpstvxz][bcdfgjklmnprstvxz]?[lmnr]?)|([bcdfgjklmnprstvxz]))$/);
    }
    if (match === null)
        return false;
    // Last part needs to be a cluster that could start a zi'evla
    // iln(sp)i -> YES
    // iln(kp)i -> NO
    // iln(skr)i -> YES
    // iln(tkr)i -> NO
    if (match[match.length - 2] !== undefined && !isZihevlaInitialCluster(match[match.length - 2]))
        return false;
    return true;
}
/**
 * Check if string is a valid CLL rafsi.
 *
 * @param rafsi A string.
 * @param allowMZ True if mz is a valid consonant cluster.
 * @returns True if string is a valid CLL rafsi.
 */
function isValidRafsi(rafsi, allowMZ = false) {
    const raftai = rafsiTarmi(rafsi);
    if ([Tarmi.CVCCV, Tarmi.CVCC].includes(raftai))
        return (allowMZ ? MZ_VALID : VALID).includes(rafsi.slice(2, 4));
    if ([Tarmi.CCVCV, Tarmi.CCVC, Tarmi.CCV].includes(raftai))
        return INITIAL.includes(rafsi.slice(0, 2));
    return 1 <= raftai && raftai <= 8;
}
/**
 * Get the shape of a rafsi.
 *
 * @param rafsi A rafsi.
 * @returns The rasfi's shape (an int enum).
 */
function rafsiTarmi(rafsi) {
    const rafLen = rafsi.length;
    if (rafLen === 0) {
        return Tarmi.OtherRafsi;
    }
    else if (rafLen === 2 && rafsi[0] === "'" && rafsi[1] === "y") {
        return Tarmi.Hyphen;
    }
    else if (!isConsonant(rafsi[0]) && rafLen !== 1) {
        return Tarmi.OtherRafsi;
    }
    switch (rafLen) {
        case 1:
            if (isVowel(rafsi))
                return Tarmi.OtherRafsi;
            else
                return Tarmi.Hyphen;
        case 3:
            if (!isVowel(rafsi[2])) {
                if (isVowel(rafsi[1]) && isConsonant(rafsi[2]))
                    return Tarmi.CVC;
            }
            else {
                if (isVowel(rafsi[1]))
                    return Tarmi.CVV;
                else if (isConsonant(rafsi[1]))
                    return Tarmi.CCV;
            }
        case 4:
            if (isVowel(rafsi[1])) {
                if (isVowel(rafsi[3])) {
                    if (rafsi[2] === "'")
                        return Tarmi.CVhV;
                }
                else if (isConsonant(rafsi[2]) && isConsonant(rafsi[3])) {
                    return Tarmi.CVCC;
                }
            }
            else if (isConsonant(rafsi[1]) && isVowel(rafsi[2]) &&
                isConsonant(rafsi[3])) {
                return Tarmi.CCVC;
            }
        case 5:
            if (isGismuShape(rafsi)) {
                if (isVowel(rafsi[2]))
                    return Tarmi.CCVCV;
                else
                    return Tarmi.CVCCV;
            }
    }
    return Tarmi.OtherRafsi;
}
/**
 * Get the rafsi without any initial or final hyphen characters.
 *
 * @param rafsi A rafsi.
 * @returns The rafsi without hyphens.
 */
function stripHyphens(rafsi) {
    while ("'y".includes(rafsi[0]))
        rafsi = rafsi.slice(1);
    while ("'y".includes(rafsi.slice(-1)))
        rafsi = rafsi.slice(0, -1);
    return rafsi;
}
/**
 * Get the rafsi's shape, removing a final hyphen if necessary.
 *
 * @param rafsi A rafsi.
 * @returns The rasfi's shape (an int enum).
 */
function tarmiIgnoringHyphen(rafsi) {
    return rafsiTarmi(stripHyphens(rafsi));
}

/*
Copyright (c) 2020 uakci (https://github.com/uakci)
Licensed under the MIT License

Modified by latkerlo (https://github.com/latkerlo), Copyright (c) 2023
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
    Tarmi[Tarmi["Fuhivla"] = 9] = "Fuhivla";
})(Tarmi || (Tarmi = {}));
function isVowel(character) {
    return "aeiou".includes(character);
}
function isConsonant(character) {
    return "bcdfgjklmnprstvxz".includes(character);
}
function isOnlyLojbanCharacters(valsi) {
    return /^[aeioubcdfgjklmnprstvxz']+$/.test(valsi);
}
function isGismu(valsi) {
    return valsi.length == 5 && isConsonant(valsi[0]) &&
        isConsonant(valsi[3]) && isVowel(valsi[4]) &&
        ((isVowel(valsi[1]) && isConsonant(valsi[2])) ||
            (isConsonant(valsi[1]) && isVowel(valsi[2])));
}
function isValidRafsi(rafsi) {
    const raftai = rafsiTarmi(rafsi);
    if ([Tarmi.CVCCV, Tarmi.CVCC].includes(raftai))
        return VALID.includes(rafsi.slice(2, 4));
    if ([Tarmi.CCVCV, Tarmi.CCVC, Tarmi.CCV].includes(raftai))
        return INITIAL.includes(rafsi.slice(0, 2));
    return 1 <= raftai && raftai <= 8;
}
function rafsiTarmi(rafsi) {
    const rafLen = rafsi.length;
    if (rafLen === 0) {
        return Tarmi.Fuhivla;
    }
    else if (rafLen === 2 && rafsi[0] === "'" && rafsi[1] === "y") {
        return Tarmi.Hyphen;
    }
    else if (!isConsonant(rafsi[0]) && rafLen !== 1) {
        return Tarmi.Fuhivla;
    }
    switch (rafLen) {
        case 1:
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
            if (isGismu(rafsi)) {
                if (isVowel(rafsi[2]))
                    return Tarmi.CCVCV;
                else
                    return Tarmi.CVCCV;
            }
    }
    return Tarmi.Fuhivla;
}
function tarmiIgnoringHyphen(rafsi) {
    if (rafsi.slice(-1) === "y")
        rafsi = rafsi.slice(0, -1);
    return rafsiTarmi(rafsi);
}

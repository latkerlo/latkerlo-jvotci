/*
Copyright (c) 2020 uakci (https://github.com/uakci)
Licensed under the MIT License

Modified by latkerlo (https://github.com/latkerlo), Copyright (c) 2023
*/

enum Tarmi {
  Hyphen,
  CVCCV,
  CVCC,
  CCVCV,
  CCVC,
  CVC,
  CVhV,
  CCV,
  CVV,
  Fuhivla
}

function isVowel(character: string): boolean {
  return "aeiou".includes(character);
}

function isConsonant(character: string): boolean {
  return "bcdfgjklmnprstvxz".includes(character);
}

function isOnlyLojbanCharacters(valsi: string): boolean {
  return /^[aeioubcdfgjklmnprstvxz']+$/.test(valsi);
}

function isGismu(valsi: string): boolean {
  return valsi.length == 5 && isConsonant(valsi[0]) && 
    isConsonant(valsi[3]) && isVowel(valsi[4]) && 
    ((isVowel(valsi[1]) && isConsonant(valsi[2])) || 
    (isConsonant(valsi[1]) && isVowel(valsi[2])));
}

function isValidRafsi(rafsi: string): boolean {
  const raftai = rafsiTarmi(rafsi);
  if ([Tarmi.CVCCV, Tarmi.CVCC].includes(raftai))
    return VALID.includes(rafsi.slice(2, 4));
  if ([Tarmi.CCVCV, Tarmi.CCVC, Tarmi.CCV].includes(raftai))
    return INITIAL.includes(rafsi.slice(0, 2));
  return 1 <= raftai && raftai <= 8;
}

function rafsiTarmi(rafsi: string): Tarmi {
  const rafLen = rafsi.length;
  if (rafLen === 0) {
    return Tarmi.Fuhivla;
  } else if (rafLen === 2 && rafsi[0] === "'" && rafsi[1] === "y") {
    return Tarmi.Hyphen;
  } else if (!isConsonant(rafsi[0]) && rafLen !== 1) {
    return Tarmi.Fuhivla;
  }
  switch(rafLen) {
    case 1:
      return Tarmi.Hyphen;
    case 3:
      if (!isVowel(rafsi[2])) {
        if (isVowel(rafsi[1]) && isConsonant(rafsi[2]))
          return Tarmi.CVC;
      } else {
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
        } else if (isConsonant(rafsi[2]) && isConsonant(rafsi[3])) {
          return Tarmi.CVCC;
        }
      } else if (isConsonant(rafsi[1]) && isVowel(rafsi[2]) && 
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

function tarmiIgnoringHyphen(rafsi: string): Tarmi {
  if (rafsi.slice(-1) === "y")
    rafsi = rafsi.slice(0, -1);
  return rafsiTarmi(rafsi);
}
/*
Copyright (c) 2021 sozysozbot (https://github.com/sozysozbot)
Licensed under the MIT License

Modified by latkerlo (https://github.com/latkerlo), Copyright (c) 2023
*/

function searchSelrafsiFromRafsi(rafsi: string): string | null {
  if (rafsi.length === 5 && RAFSI.has(rafsi))
    return rafsi;  // 5-letter rafsi

  if (rafsi !== "brod" && rafsi.length === 4 && !rafsi.includes("'")) {  // 4-letter rafsi
    for (let u = 0; u < 5; u++) {
      const gismuCandid = rafsi + "aeiou"[u];
      if (RAFSI.has(gismuCandid))
        return gismuCandid;
    }
  }
  for (const [valsi, rafsi_list] of RAFSI.entries()) {
    if (rafsi_list.includes(rafsi))
      return valsi;
  }
  return null;
}

function jvokaha(lujvo: string): string[] {
  const arr = jvokaha2(lujvo);

  const rafsiTanru = arr.filter(x => x.length > 1).map(x => `-${x}-`);
  const correctLujvo = getLujvo2(
    rafsiTanru, 
    isConsonant(arr[arr.length - 1].slice(-1))
  )[0];

  if (lujvo === correctLujvo)
    return arr;
  else
    throw new Error("malformed lujvo {" + lujvo + 
    "}; it should be {" + correctLujvo + "}")
}

function jvokaha2(lujvo: string): string[] {
  const original_lujvo = lujvo;
  const res: string[] = [];
  while (true) {
    if (lujvo === "")
        return res;

    // remove hyphen
    if (res.length > 0 && res[res.length - 1].length !== 1) {  // hyphen cannot begin a word; nor can two hyphens
      if (
        lujvo[0] === "y"  // y-hyphen
        || lujvo.slice(0, 2) === "nr"  // n-hyphen is only allowed before r
        || lujvo[0] === "r" && isConsonant(lujvo[1])  // r followed by a consonant
      ) {
        res.push(lujvo[0]);
        lujvo = lujvo.slice(1);
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
        if (!VALID.includes(lujvo.slice(2, 4)))
          throw new Error(`Invalid cluster {${lujvo.slice(2, 4)}} in 
          {${original_lujvo}}`);
      } else {
        if (!INITIAL.includes(lujvo.slice(0, 2)))
          throw new Error(`Invalid initial cluster {${lujvo.slice(0, 2)}} in 
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

    if ([Tarmi.CVC, Tarmi.CCV].includes(rafsiTarmi(lujvo.slice(0, 3)))) {
      // TODO: Why is a test for valid initial not needed here?
      res.push(lujvo.slice(0, 3));
      lujvo = lujvo.slice(3);
      continue;
    }

    // if all fails...
    // console.log(res, lujvo)
    throw new Error("Failed to decompose {" + original_lujvo + "}");
  }
}

function getVeljvo(lujvo: string): string[] {
  const rafsiList = jvokaha(lujvo).filter(x => x.length > 1);
  const selrafsiList = rafsiList.map(x => searchSelrafsiFromRafsi(x));
  for (const [i, selrafsi] of selrafsiList.entries())
    rafsiList[i] = selrafsi !== null ? selrafsi : `-${rafsiList[i]}-`;
  return rafsiList;
}

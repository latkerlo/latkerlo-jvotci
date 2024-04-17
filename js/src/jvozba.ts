/*
Copyright (c) 2023 la latxli (https://codeberg.org/tb148)
Licensed under the Apache License, Version 2.0

Modified by latkerlo (https://github.com/latkerlo), Copyright (c) 2023
*/

type bestLujvoMap = Map<string, [string, number]>;

function score(rafsi: string): number {
  return (
    1000 * rafsi.length
    - 500 * (rafsi.match(/'/g) || []).length
    + 100 * (rafsi.match(/y/g) || []).length
    - 10 * tarmiIgnoringHyphen(rafsi)
    - (rafsi.match(/[aeiou]/g) || []).length
  );
}

function processTanru(tanru: string | string[]): string[] {
  let valsiList: string[];
  if (typeof tanru === 'string')
    valsiList = tanru.trim().split(/\s+/);
  else if (Array.isArray(tanru))
    valsiList = tanru;
  else
    throw new Error(`Cannot make lujvo from type ${typeof(tanru)}`);

  let expandedValsiList: string[] = [];
  valsiList.forEach(valsi => {
    expandedValsiList = expandedValsiList.concat(
      !valsi.includes("-") ? splitWords(valsi) : [valsi]);
  });
  return expandedValsiList;
}

function getRafsiListList(
  valsiList: string[], 
  generateCmene = false
): string[][] {
  const listOfRafsiLists: string[][] = [];
  for (let [i, valsi] of valsiList.entries()) {
    const rafsiList: string[] = [];
    if (valsi.slice(-1) === "-") {
      valsi = valsi.replace(/^\-+|\-+$/g, '');
      if (!isOnlyLojbanCharacters(valsi))
        throw new Error("Non-lojban character in {" + valsi + "}")
      if (!isValidRafsi(valsi))
        throw new Error("Invalid cluster in {" + valsi + "}");
      if (isGismu(valsi) && i !== valsiList.length - 1)
        throw new Error("Non-final 5-letter rafsi: {" + valsi + "}")

      if ([Tarmi.CCVC, Tarmi.CVCC].includes(rafsiTarmi(valsi))) {
        if (generateCmene && i === valsiList.length - 1)
          rafsiList.push(valsi);
      } else {
        rafsiList.push(valsi);
      }
      if (isConsonant(valsi.slice(-1)) && 
          !(generateCmene && i === valsiList.length - 1))
        rafsiList.push(valsi + "y");

    } else {
      if (!isOnlyLojbanCharacters(valsi))
        throw new Error("Non-lojban character in {" + valsi + "}")

      const shortRafsiList = RAFSI.get(valsi);
      if (shortRafsiList != undefined){
        shortRafsiList.forEach(shortRafsi => {
          rafsiList.push(shortRafsi);
          if (isConsonant(shortRafsi.slice(-1)))
            rafsiList.push(shortRafsi + "y");
        });
      }
      
      if (isGismu(valsi)) {
        if (!isValidRafsi(valsi))
          throw new Error("Invalid cluster in {" + valsi + "}");
        if (i === valsiList.length - 1) {
          if (generateCmene)
            rafsiList.push(valsi.slice(0, -1));
          else
            rafsiList.push(valsi);
        } else {
          rafsiList.push(valsi.slice(0, -1) + "y");
        }
      }
    }
    listOfRafsiLists.push(rafsiList);
  }
  return listOfRafsiLists;
}

function combine(
    lujvo: string, 
    rafsi: string, 
    lujvo_score: number, 
    isTosmabru: number, 
    generateCmene: boolean, 
    tanruLen = 0): 
    [number, number, string] | null {
  const lujvoFinal = lujvo.slice(-1);
  const rafsiInitial = rafsi[0];
  if (
    isConsonant(lujvoFinal) 
    && isConsonant(rafsiInitial) 
    && !VALID.includes(lujvoFinal + rafsiInitial)
  ) {
    return null;
  }
  if (
    ["ndj", "ndz", "ntc", "nts"].includes(lujvoFinal + rafsi.slice(0, 2))
  ) {
    return null;
  }

  const raftai1 = tarmiIgnoringHyphen(rafsi);
  let hyphen = "";
  if (lujvo.length <= 5 && !generateCmene) {
    const raftai0 = tarmiIgnoringHyphen(lujvo);
    if ([Tarmi.CVhV, Tarmi.CVV].includes(raftai0)) {
      if (rafsiInitial == "r")
        hyphen = "n";
      else
        hyphen = "r";
    }
    if (tanruLen === 2 && raftai1 === Tarmi.CCV)
      hyphen = "";
  }

  if (isTosmabru) {
    if (!INITIAL.includes(lujvoFinal + rafsiInitial)){
      isTosmabru = 0;
    } else if (raftai1 === Tarmi.CVCCV) {
      if (INITIAL.includes(rafsi.slice(2, 4))) {
        return null;
      }
      isTosmabru = 0;
    } else if (raftai1 === Tarmi.CVC) {
      if (rafsi.slice(-1) === "y") {
        return null;
      }
    } else {
      isTosmabru = 0;
    }
  }

  return [
    isTosmabru,
    lujvo_score + 1100 * hyphen.length + score(rafsi),
    lujvo + hyphen + rafsi,
  ];
} 

function updateCurrentBest(
    result: [number, number, string] | null, 
    currentBest: [bestLujvoMap, bestLujvoMap]) {
  if (result == null)
    return;
  const [isTosmabru, resScore, resLujvo] = result!;
  const lujvoFinal = resLujvo.slice(-1);
  if (!currentBest[isTosmabru].has(lujvoFinal) || 
      currentBest[isTosmabru].get(lujvoFinal)![1] > resScore) {
    currentBest[isTosmabru].set(lujvoFinal, [resLujvo, resScore]);
  }
}

function getLujvo(
  tanru: string | string[], 
  generateCmene=false
): [string, number] {
  return getLujvo2(processTanru(tanru), generateCmene);
}

function getLujvo2(
  valsiList: string[], 
  generateCmene = false
): [string, number] {
  const rafsiListList = getRafsiListList(valsiList, generateCmene);
  let currentBest: [bestLujvoMap, bestLujvoMap] = [new Map(), new Map()];
  rafsiListList[0].forEach((rafsi0) => {
    rafsiListList[1].forEach((rafsi1) => {
      const isTosmabru = 
        tarmiIgnoringHyphen(rafsi0) == Tarmi.CVC 
        && rafsi0.slice(-1) !== "y" 
        && !generateCmene ? 1 : 0;
      const result = combine(
        rafsi0,
        rafsi1,
        score(rafsi0),
        isTosmabru,
        generateCmene,
        rafsiListList.length
      );
      updateCurrentBest(result, currentBest);
    });
  });
  let previousBest = currentBest;
  rafsiListList.slice(2).forEach((rafsiList) => {
    currentBest = [new Map(), new Map()];
    rafsiList.forEach((rafsi) => {
      for (let isTosmabru = 0; isTosmabru < 2; isTosmabru++) {
        previousBest[isTosmabru].forEach((lujvoAndScore, _) => {
          const result = combine(
            lujvoAndScore[0],
            rafsi,
            lujvoAndScore[1],
            isTosmabru,
            generateCmene
          );
          updateCurrentBest(result, currentBest);
        });
      }
    });
    previousBest = currentBest;
  });
  let bestLujvo = "";
  let bestScore = 10**100;
  previousBest[0].forEach((lujvoAndScore, lerfu) => {
    if ((isVowel(lerfu) && !generateCmene) 
        || (!isVowel(lerfu) && generateCmene)) {
      if (lujvoAndScore[1] < bestScore)
        [bestLujvo, bestScore] = lujvoAndScore;
    }
  });
  if (bestLujvo === "")
    throw new Error("No lujvo found for {" + valsiList.join(" ") + "}");
  return [bestLujvo, bestScore];
}

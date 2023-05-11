/*
Copyright (c) 2023 latkerlo (https://github.com/latkerlo)
Licensed under the MIT License
*/
function isGismuOrLujvo(aString) {
    if (isGismu(aString))
        return true;
    try {
        jvokaha(aString);
        return true;
    }
    catch (error) {
        return false;
    }
}
// Assumes the string does need to be split and the remainder will be valid lojban
function splitOneCmavo(aString) {
    let i = 0;
    let willEnd = false;
    while (i < aString.length) {
        if (["ai", "ei", "oi", "au"].includes(aString.slice(i, i + 2))
            && i + 2 < aString.length
            && !"aeiouy".includes(aString[i + 2])) {
            i += 2;
            willEnd = true;
        }
        else if ("iu".includes(aString[i])
            && i + 1 < aString.length
            && "aeiouy".includes(aString.slice(i + 1, i + 2))) {
            if (willEnd)
                break;
            i += 2;
            willEnd = true;
        }
        else if ("aeiouy".includes(aString[i])) {
            i += 1;
            willEnd = true;
        }
        else if (aString[i] === "'") {
            i += 1;
            willEnd = false;
        }
        else if ("bcdfgjklmnprstvxz".includes(aString[i])) {
            if (i === 0) {
                i += 1;
                continue;
            }
            else {
                break;
            }
        }
        else {
            throw new Error(`Non-lojban character {${aString[i]}} in 
      {${aString}} at index {${i}}`);
        }
    }
    return [aString.slice(0, i), aString.slice(i)];
}
function splitWords(aString) {
    if (aString.length === 0)
        return [];
    if (isConsonant(aString.slice(-1)))
        return [aString];
    const firstFive = aString.replace(/[y']/g, '').slice(0, 5);
    const consonantCluster = firstFive.search(/[bcdfgjklmnprstvxz][bcdfgjklmnprstvxz]/);
    if (consonantCluster === -1) {
        const [cmavo, remainder] = splitOneCmavo(aString);
        return [cmavo].concat(splitWords(remainder));
    }
    if (isGismuOrLujvo(aString))
        return [aString];
    let i = 0;
    for (const char of aString) {
        if (i >= consonantCluster)
            break;
        if (!['y', "'"].includes(char))
            i += 1;
    }
    if (isGismuOrLujvo(aString.slice(i)))
        return [aString.slice(0, i), aString.slice(i)];
    else
        return [aString];
}

console.log(getLujvo("mlatu kerlo"));
console.log(getLujvo(["mlatu", "kerlo"]));
// -> "latkerlo"

console.log(getLujvo("mlatu kerlo", {generateCmevla: true}));
console.log(getLujvo(["mlatu", "kerlo"], {generateCmevla: true}));
// -> "latker"

console.log(getVeljvo("latkerlo"));  // -> ["mlatu", "kerlo"]

console.log(isBrivla("latkerlo"));  // -> true
console.log(isBrivla("bisladru"));  // -> false
console.log(isBrivla("latker"));    // -> false

console.log(analyseBrivla("tcanyja'a"));  // -> ["LUJVO", ["tcan", "y", "ja'a"]]
console.log(analyseBrivla("re'ertren"));  // -> ["CMEVLA", ["re'e", "r", "tren"]]
console.log(analyseBrivla("mlatu"));      // -> ["GISMU", ["mlatu"]]
try {
  console.log(analyseBrivla("latkello"));   // -> throws NotBrivlaError
} catch(e) {
  if (e instanceof NotBrivlaError)
    console.log(`Error: ${e.message}`);
}

console.log(getLujvoWithAnalytics("mlatu kerlo"));
// -> ["latkerlo", 7937, [[0, 3], [3, 8]]]

console.log(getLujvoWithAnalytics("tcana jatna"));
// -> ["tcanyja'a", 8597, [[0, 4], [5, 9]]]

console.log(getRafsiIndices(["lat", "kerlo"]));
// -> [[0, 3], [3, 8]]

let [bType, decomp] = analyseBrivla("tcanyja'a");
console.log(getRafsiIndices(decomp));
// -> [[0, 4], [5, 9]]

console.log(getVeljvo("le'e'e'ygimzu", {
  yHyphens: "ALLOW_Y", 
  expRafsiShapes: true, 
  consonants: "TWO_CONSONANTS", 
  glides: true, 
  allowMZ: true
}));
// -> ["-le'e'e-", "gimzu"]

try {
  console.log(getVeljvo("le'e'e'ygimzu", {allowMZ: true}));
} catch(e) {
  if (e instanceof NotBrivlaError)
    console.log(`Error: ${e.message}`);
}

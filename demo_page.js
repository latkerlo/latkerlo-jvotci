/*
Copyright (c) 2023-2024 latkerlo (https://github.com/latkerlo)
Licensed under the MIT License
*/

window.addEventListener("DOMContentLoaded", () => {
  const source = document.getElementById("source");
  
  let interval = undefined;
  let doCmevla = false;
  let isEzMode = false;

  const brivlaButton = document.getElementById("brivla");
  const cmevlaButton = document.getElementById("cmevla");
  brivlaButton.addEventListener("click", () => {
    if (doCmevla) {
      doCmevla = false;
      brivlaButton.className = "active";
      cmevlaButton.className = "";
      go();
    }
  })
  cmevlaButton.addEventListener("click", () => {
    if (!doCmevla) {
      doCmevla = true;
      brivlaButton.className = "";
      cmevlaButton.className = "active";
      go();
    }
  })

  const docsButton = document.getElementById("docs-button");
  const docsSpan = document.getElementById("docs-span");

  const ezModeButton = document.getElementById("ezmode");
  const hardTable = document.getElementById("hard-mode");
  const easyTable = document.getElementById("easy-mode");
  ezModeButton.addEventListener("click", () => {
    isEzMode = !isEzMode;
    ezModeButton.className = isEzMode ? "active" : "";
    go();
  })

  function setResult(resultContainer, aResult, yHyphen, moreRafsi, consonants, glides, mz) {

  }

  const defaultSettings = [[YHyphenSetting.STANDARD], [false], [ConsonantSetting.CLUSTER], [false], [false]]
  const settingsNames = [
    new Map([
      [YHyphenSetting.STANDARD, "STANDARD"],
      [YHyphenSetting.ALLOW_Y, "ALLOW_Y"],
      [YHyphenSetting.FORCE_Y, "FORCE_Y"]
    ]),
    new Map([
      [false, "STD_RAF"],
      [true, "EXP_RAF"]
    ]),
    new Map([
      [ConsonantSetting.CLUSTER, "CLUSTER"],
      [ConsonantSetting.TWO_CONSONANTS, "2_CON"],
      [ConsonantSetting.ONE_CONSONANT, "1_CON"]
    ]),
    new Map([
      [false, "NO_GLIDE"],
      [true, "GLIDE"],
    ]),
    new Map([
      [false, "NOMZ"],
      [true, "MZ"]
    ])
  ];

  function go() {
    const userInput = source.value.trim();
    const numWords = userInput.split(/\s+/).length;

    const settings = isEzMode ? defaultSettings : SETTINGS;

    const allResults = new Map();
    settings[0].forEach((value0) => {
      allResults.set(value0, new Map())
      settings[1].forEach((value1) => {
        allResults.get(value0).set(value1, new Map())
        settings[2].forEach((value2) => {
          allResults.get(value0).get(value1).set(value2, new Map())
          settings[3].forEach((value3) => {
            allResults.get(value0).get(value1).get(value2).set(value3, new Map())
            settings[4].forEach((value4) => {
              allResults.get(value0).get(value1).get(value2).get(value3).set(value4, new Map())
            })
          })
        })
      })
    })

    const iter = makeSettingsIterator(settings);
    let result = iter.next();
    while (!result.done) {
      const yHyphen = result.value[0];
      const moreRafsi = result.value[1];
      const consonants = result.value[2];
      const glides = result.value[3];
      const mz = result.value[4];
      result = iter.next();

      let cellContent = "";
      let cellClass = "";
      if (numWords === 1 && userInput.length > 0) {
        try {
          const [bType, pieces] = analyseBrivla(
            userInput, 
            {
              yHyphens: yHyphen,
              expRafsiShapes: moreRafsi,
              consonants: consonants,
              glides: glides,
              allowMZ: mz
            }
          );
          cellContent = `${bType}<br>[${pieces.join(", ")}]`;
          if ([BrivlaType.LUJVO, BrivlaType.EXTENDED_LUJVO, BrivlaType.CMEVLA].includes(bType))
            cellContent += `<br>${selrafsiListFromRafsiList(
              pieces, 
              {
                yHyphens: yHyphen,
                expRafsiShapes: moreRafsi,
                consonants: consonants,
                glides: glides,
                allowMZ: mz
              }).join(" ")}`;
          cellClass = bType === BrivlaType.CMEVLA ? "yellow" : "green";
        } catch (error) {
          cellContent = "NOT BRIVLA";
          cellClass = "red";
        }

      } else if (numWords > 1) {
        try {
          const lujvo = getLujvo(
            userInput,
            {
              generateCmevla: doCmevla,
              yHyphens: yHyphen,
              expRafsiShapes: moreRafsi,
              consonants: consonants,
              glides: glides,
              allowMZ: mz
            }
          );
          cellContent = lujvo;
          cellClass = doCmevla ? "yellow" : "green";
        } catch (error) {
          cellContent = "NO LUJVO FOUND";
          cellClass = "red";
        }
      }

      allResults.get(yHyphen).get(moreRafsi).get(consonants).get(glides).set(mz, [cellContent, cellClass]);
    }

    const invariants = settings.map(x => true);
    
    for (let i = 0; i < settings.length; i++) {
      const otherSettings = settings.slice(); 
      otherSettings.splice(i, 1);

      const iter = makeSettingsIterator(otherSettings);
      let result = iter.next();
      while (!result.done && invariants[i]) {
        const others = result.value;
        result = iter.next();

        const alls = others.slice();
        alls.splice(i, 0, settings[i][0]);
        const defaultResult = allResults.get(alls[0]).get(alls[1]).get(alls[2]).get(alls[3]).get(alls[4])[0];
        
        for (let j = 1; j < settings[i].length; j++) {
          alls[i] = settings[i][j];
          const thisResult = allResults.get(alls[0]).get(alls[1]).get(alls[2]).get(alls[3]).get(alls[4])[0];
          if (thisResult !== defaultResult) {
            invariants[i] = false;
            break;
          }
        }
      }
    }

    const xSettings = [];
    const ySettings = [];
    let yWasLast = false;
    
    for (let i = 0; i < settings.length; i++) {
      if (!invariants[i]) {
        (yWasLast ? xSettings : ySettings).push(i);
        yWasLast = !yWasLast;
      }
    }

    hardTable.innerHTML = "";
    let numColumns = 1;
    xSettings.forEach(i => {
      numColumns *= settings[i].length;
    })
    let numRows = 1;
    ySettings.forEach(i => {
      numRows *= settings[i].length;
    })

    let headHTML = "<thead>";
    let someNumber = 1;
    xSettings.forEach(i => {
      let rowHTML = "";
      for (let j = 0; j < ySettings.length; j++) {
        rowHTML += '<th style="width:5%"></th>';
      }
      for (let k = 0; k < someNumber; k++) {
        for (let j = 0; j < settings[i].length; j++) {
          rowHTML += `<th style="width:20%" colspan="${numColumns / someNumber / settings[i].length}">`;
          rowHTML += `${settingsNames[i].get(settings[i][j])}</th>`;
        }
      }
      someNumber *= settings[i].length;
      headHTML += "<tr>" + rowHTML + "</tr>";
    })
    headHTML += "</thead>";

    let bodyHTML = "<tbody>";
    for (let i = 0; i < numRows; i++) {
      let rowHTML = "<tr>";
      const allSettings = defaultSettings.slice();

      someNumber = numRows;
      ySettings.forEach(j => {
        someNumber /= settings[j].length;
        const thisSetting = settings[j][Math.floor(i / someNumber) % settings[j].length];
        allSettings[j] = [thisSetting];
        if ((i / someNumber) % 1 === 0) {
          rowHTML += `<td${someNumber > 1 ? ` rowspan="${someNumber}"` : ""}>${settingsNames[j].get(thisSetting)}</td>`;
        }
      })

      for (let k = 0; k < numColumns; k++) {
        someNumber = numColumns;
        xSettings.forEach(j => {
          someNumber /= settings[j].length;
          const thisSetting = settings[j][Math.floor(k / someNumber) % settings[j].length];
          allSettings[j] = [thisSetting];
        })
        const [cellContent, cellClass] = allResults.get(allSettings[0][0]).get(allSettings[1][0]).get(allSettings[2][0]).get(allSettings[3][0]).get(allSettings[4][0]);
        rowHTML += `<td class="${cellClass}">${cellContent}</td>`;
      }
      bodyHTML += rowHTML + "</tr>";
    }
    bodyHTML += "<tbody>";
    hardTable.innerHTML = headHTML + bodyHTML;
  }

  function goDebounced() {
    window.clearTimeout(interval);
    interval = window.setTimeout(go, 15);
  }
  source.addEventListener("input", goDebounced);
})

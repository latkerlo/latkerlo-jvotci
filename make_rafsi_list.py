"""
Copyright (c) 2023-2024 latkerlo (https://github.com/latkerlo)
Licensed under the MIT License
"""

import sys
sys.path.append("py")

import xml.etree.ElementTree as ET
import urllib.request
from latkerlo_jvotci.tarmi import *
from latkerlo_jvotci.data import INITIAL

GIMSTE = set()

URL = "https://jbovlaste.lojban.org/export/xml-export.html?lang=en&positive_scores_only=0&bot_key=z2BsnKYJhAB0VNsl"
with urllib.request.urlopen(URL) as response:
    root = ET.fromstring(response.read())
for valsi in root.iter("valsi"):
    if valsi.get("type") == "gismu":
        word = valsi.get("word")
        GIMSTE.add(word[:4])

rafsi_list = {}
exp_rafsi_list = {}
priority_rafsi = set()
for valsi in root.iter("valsi"):
    word = valsi.get("word")

    word_type = valsi.get('type')
    if word_type in ["gismu", "experimental gismu"]:
        if word_type == "experimental gismu" and word[:4] in GIMSTE:
            continue
        rafsi_list[word] = set()
    elif word_type not in ["cmavo", "experimental cmavo"]:
        continue

    for rafsi_tag in valsi.findall("rafsi"):
        rafsi = rafsi_tag.text
        priority_rafsi.add(rafsi)
        try:
            rafsi_list[word].add(rafsi)
        except KeyError:
            rafsi_list[word] = {rafsi}

    notes = valsi.findtext("notes") or ""
    if "rafsi" not in notes.lower():
        continue
    m = re.findall('([^a-z]|.)-([bcdfgjklmnprstvxz][a-z]\'?[a-z])-([^a-z]|$)', notes)
    proposed_rafsi_list = [x[1] for x in m]
    for rafsi in proposed_rafsi_list:
        if is_consonant(rafsi[1]) and rafsi[:2] not in INITIAL:
            continue  # zucna: zna
        try:
            exp_rafsi_list[word].add(rafsi)
        except KeyError:
            exp_rafsi_list[word] = {rafsi}

# A few by-hand exceptions and tie-breakers
EXCEPTIONS = {
    "xi": ["xix"],
    "ditcu": ["dit"],
    "jidge": ["jid"],
    "ronci": ["roc"],
    "supso": ["sus"],
    "zai'e": ["zam"],
    "gelse": ["ge'e", "ges"],
    "tsako": ["tso"],
    "kilma": ["kim"],

    "je'ebzi": ["jeb"],
    "mu'umgu": ["mug"],
    "va'arga": ["va'a"],
    # "bom": ["bom"],
    # "nom": ["nom"],
}
for selrafsi, raf_list in EXCEPTIONS.items():
    for rafsi in raf_list:
        priority_rafsi.add(rafsi)
        try:
            rafsi_list[selrafsi].add(rafsi)
        except KeyError:
            rafsi_list[selrafsi] = {rafsi}

for selrafsi, proposed_rafsi_list in exp_rafsi_list.items():
    for rafsi in proposed_rafsi_list:
        if rafsi not in priority_rafsi:
            try:
                rafsi_list[selrafsi].add(rafsi)
            except KeyError:
                rafsi_list[selrafsi] = {rafsi}

# .u'u abhorrently long dictionary comprehension
rafsi_list = {selrafsi: sorted(list(rafsi), key=lambda x: rafsi_tarmi(x) % 9) for selrafsi, rafsi in rafsi_list.items()}

# py
with open("py/latkerlo_jvotci/rafsi.py", "w") as opf:
    opf.write("RAFSI_LIST = {\n")
    for selrafsi, rafsi in rafsi_list.items():
        opf.write(f'    "{selrafsi}": {str(rafsi)},\n')
    opf.write("}\n")

# ts
with open("js/src/rafsi.ts", "w") as opf:
    opf.write("const RAFSI_LIST: Map<string, string[]> = new Map([\n")
    for selrafsi, rafsi in rafsi_list.items():
        opf.write(f'  ["{selrafsi}", {str(rafsi)}],\n')
    opf.write("]);\n")

# js
with open("js/docs/rafsi.js", "w") as opf:
    opf.write("const RAFSI_LIST = new Map([\n")
    for selrafsi, rafsi in rafsi_list.items():
        opf.write(f'    ["{selrafsi}", {str(rafsi)}],\n')
    opf.write("]);\n")

# rs
with open("rs/src/rafsi.rs", "w") as opf:
    opf.write("//! Contains the static RAFSI, a map from words to their affixes.")
    opf.write("\nuse std::{" + "collections::HashMap, sync::LazyLock};\n/// Big giant rafsi list.\npub static RAFSI: LazyLock<HashMap<&'static str, Vec<&'static str>>> = LazyLock::new(|| {\n    HashMap::from([\n")
    for selrafsi, rafsi in rafsi_list.items():
        opf.write('        ("' + selrafsi + '", vec![' + ('"' if len(rafsi) else "") + '", "'.join(rafsi) + ('"' if len(rafsi) else "") + "]" + "),\n")
    opf.write("    ])\n});\n")

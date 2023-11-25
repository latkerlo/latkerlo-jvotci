"""
Copyright (c) 2023 latkerlo (https://github.com/latkerlo)
Licensed under the MIT License
"""

import json
import xml.etree.ElementTree as ET

import sys
sys.path.append("py")
from py.tarmi import *
from py.data import INITIAL

GIMSTE = set()

root = ET.parse(f"jbovlaste-en.xml").getroot()
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
            continue
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

rafsi_list = {selrafsi: list(rafsi) for selrafsi, rafsi in rafsi_list.items()}

with open("py/rafsi_list.json", "w") as opf:
    json.dump(rafsi_list, opf)
with open("js/src/rafsi.ts", "w") as opf:
    opf.write("const RAFSI: Map<string, string[]> = new Map([\n")
    for selrafsi, rafsi in rafsi_list.items():
        opf.write(f'  ["{selrafsi}", {str(rafsi)}],\n')
    opf.write("]);\n")
with open("js/docs/rafsi.js", "w") as opf:
    opf.write("const RAFSI = new Map([\n")
    for selrafsi, rafsi in rafsi_list.items():
        opf.write(f'    ["{selrafsi}", {str(rafsi)}],\n')
    opf.write("]);\n")

"""
Copyright (c) 2023-2024 latkerlo (https://github.com/latkerlo)
Licensed under the MIT License
"""

import xml.etree.ElementTree as ET
import json

all_words = {
    "GISMU": [],
    "LUJVO": [],
    "ZIhEVLA": [],
    "OTHER": []
}

root = ET.parse(f"jbovlaste-en.xml").getroot()
for valsi in root.iter("valsi"):
    word = valsi.get("word")
    match valsi.get("type"):
        case "gismu":
            key = "GISMU"
        case "experimental gismu":
            key = "GISMU"
        case "lujvo":
            key = "LUJVO"
        case "fu'ivla":
            key = "ZIhEVLA"
        case _:
            key = "OTHER"
    all_words[key].append(word)

with open("tests/jvs_words.json", "w") as opf:
    json.dump(all_words, opf)

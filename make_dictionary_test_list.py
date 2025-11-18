"""
Copyright (c) 2023-2024 latkerlo (https://github.com/latkerlo)
Licensed under the MIT License
"""

import urllib.request
import xml.etree.ElementTree as ET
import json

all_words = {
    "GISMU": [],
    "LUJVO": [],
    "ZIhEVLA": [],
    "OTHER": []
}
URL = "https://jbovlaste.lojban.org/export/xml-export.html?lang=en&positive_scores_only=0&bot_key=z2BsnKYJhAB0VNsl"
with urllib.request.urlopen(URL) as response:
    root = ET.fromstring(response.read())
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

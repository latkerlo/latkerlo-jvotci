"""
Copyright (c) 2023-2024 latkerlo (https://github.com/latkerlo)
Licensed under the MIT License
"""

import csv
import json

TEST_LISTS = [
    "basic_test_list",
    "jvozba_test_list",
    "katna_test_list",
]

with open(f"tests/js_tests.js", "w", encoding="utf-8") as opf:
    for test_name in TEST_LISTS:
        with open(f"tests/{test_name}.tsv", encoding="utf-8") as ipf:
            opf.write(f"const {test_name.upper()} = [\n")
            reader = csv.reader(ipf, delimiter="\t")
            for row in reader:
                try:
                    if (not row[0] and not row[1]) or row[0][0] == "#":
                        continue
                except IndexError:
                    continue

                opf.write(f"""  ["{'", "'.join(row)}"],\n""")
            opf.write("]\n\n")

    with open(f"tests/jvs_words.json", encoding="utf-8") as ipf:
        jvs_words = json.load(ipf)

    opf.write("const JVS_WORDS = new Map([\n")
    for b_type, words in jvs_words.items():
        opf.write(f'  ["{b_type}", [\n')
        for word in words:
            opf.write(f'    "{word}",\n')
        opf.write(f'  ]],\n')
    opf.write("])\n")

"""
Copyright (c) 2023 latkerlo (https://github.com/latkerlo)
Licensed under the MIT License
"""

import unittest
from tools import split_words

SPLIT_WORDS_TESTS = [
    ["latkerlo", ["latkerlo"]],
    ["mido", ["mi", "do"]],
    ["mi'elalatkerlo", ["mi'e", "la", "latkerlo"]],
    ["kosmabru", ["ko", "smabru"]],
    ["minelci", ["mi", "nelci"]],
    ["lekerlo", ["le", "kerlo"]],
    ["belemlatu", ["be", "le", "mlatu"]],
    ["lapabimumoicumelbi", ["la", "pa", "bi", "mu", "moi", "cu", "melbi"]],
    ["akti", ["akti"]],
    ["ankau", ["ankau"]],
    ["uidoge'ecidjrspageti", ["ui", "do", "ge'e", "cidjrspageti"]],
    ["pujebajenaicakumidoprami", ["pu", "je", "ba", "je", "nai", "ca", "ku", "mi", "do", "prami"]],
    ["mi'imymle", ["mi'imymle"]],
    ["mi'imomle", ["mi'i", "momle"]],
    ["tcaui", ["tcaui"]],
    ["plukauaii", ["plukauaii"]],
    ["fa'e'i'o'umiklama", ["fa'e'i'o'u", "mi", "klama"]],
    ["oi'oimiklama", ["oi'oi", "mi", "klama"]],
    ["uidoge'ecidjrspageti", ["ui", "do", "ge'e", "cidjrspageti"]],
    ["zo'elonuco'ecuco'eie'e'", ["zo'e", "lo", "nu", "co'e", "cu", "co'e", "ie'e'"]],
    ["ieuaui", ["ie", "ua", "ui"]],
    ["joi", ["joi"]],
]


class TestOther(unittest.TestCase):
    def testSplitWords(self):
        for string, words in SPLIT_WORDS_TESTS:
            self.assertEqual(words, split_words(string))

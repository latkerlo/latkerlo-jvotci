"""
Copyright (c) 2023 latkerlo (https://github.com/latkerlo)
Licensed under the MIT License
"""

import sys
sys.path.append("py")
from py.jvozba_test_list import *

TEST_LISTS = {
    "JVOZBAJVEKAhA_TESTS": JVOZBAJVEKAhA_TESTS,
    "CMENE_TESTS": CMENE_TESTS,
    "JVOZBA_ONLY_TESTS": JVOZBA_ONLY_TESTS,
    "JVOZBA_FAIL_TESTS": JVOZBA_FAIL_TESTS,
    "JVOKAhA_ONLY_TESTS": JVOKAhA_ONLY_TESTS,
    "JVOKAhA_FAIL_TESTS": JVOKAhA_FAIL_TESTS,
    "LUJVO_SCORE_TESTS": LUJVO_SCORE_TESTS
}

with open("js/docs/jvozba_test_list.js", "w", encoding="utf-8") as opf:
    for list_name, test_list in TEST_LISTS.items():
        opf.write(f"const {list_name} = [\n")
        if "FAIL" in list_name:
            for test in test_list:
                opf.write(f"  {test[0:1]},\n")
        else:
            for test in test_list:
                opf.write(f"  {test},\n")
        opf.write("]\n\n")

with open("rs/src/test_list.rs", "w", encoding="utf-8") as opf:
    opf.write("#[cfg(test)]\nuse crate::*;\n\n")
    for list_name, test_list in TEST_LISTS.items():
        if list_name == "JVOZBA_ONLY_TESTS":
            test_list.pop(0)
        opf.write(f"#[test]\nfn {list_name.lower()}() {{\n")
        if "FAIL" in list_name:
            print(0)
        #     for test in test_list:
        #         opf.write(f"    assert_err!({test[0:1]});\n")
        else:
            opf.write("    let tests = [\n")
            for test in test_list:
                opf.write("        [\"" + test[0] + "\", \"" + f"{test[1]}" + "\"],\n")
            opf.write("    ];\n")
            opf.write("    for test in tests {\n")
            opf.write("        println!(\"{} / {}\", test[0], test[1]);\n")
            if "Z" in list_name or "M" in list_name:
                opf.write("        assert_eq!(test[0], jvozba::get_lujvo(test[1], " + str("C" in list_name).lower() + ").unwrap().0);\n")
                opf.write("        println!(\"zbasu: pass\");\n")
            if "K" in list_name or "M" in list_name:
                opf.write("        assert_eq!(katna::get_veljvo(test[0]).join(\" \"), test[1]);\n")
                opf.write("        println!(\"katna: pass\");\n")
            opf.write("    }\n")
        opf.write("}\n\n")
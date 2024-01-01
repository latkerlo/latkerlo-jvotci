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
    opf.write("#[cfg(test)]\nuse crate::*;\n")
    for list_name, test_list in TEST_LISTS.items():
        opf.write(f"#[test]\nfn {list_name.lower()}() {{\n")
        if "FAIL" in list_name:
            print(0)
        #     for test in test_list:
        #         opf.write(f"    assert_err!({test[0:1]});\n")
        else:
            for test in test_list:
                match list_name:
                    case "JVOZBAJVEKAhA_TESTS":
                        opf.write("    assert_eq!(\"" + test[0] + "\", jvozba::get_lujvo(\"" + f"{test[1]}" + "\", false).unwrap().0);\n")
        opf.write("}\n\n")
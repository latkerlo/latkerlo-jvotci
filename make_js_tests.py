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
# TEST_LISTS.pop("LUJVO_SCORE_TESTS")
with open("rs/src/test_list.rs", "w", encoding="utf-8") as opf:
    opf.write("#[cfg(test)]\nuse crate::*;\n\n")
    for list_name, test_list in TEST_LISTS.items():
        if list_name == "JVOZBA_ONLY_TESTS":
            test_list.pop(0)
        opf.write("#[test]\n" + f"fn {list_name.lower()}() {{\n")
        opf.write("    let tests = [\n")
        for test in test_list:
            opf.write(f"        (\"{test[0]}\"" + (f", {test[1]}" if "R" in list_name else f", \"{test[1]}\"" if "F" not in list_name else "") + "),\n")
        opf.write("    ];\n")
        opf.write("    for test in tests {\n")
        if "FAIL" in list_name:
            opf.write("        println!(\"{}\", test);\n")
            if "Z" in list_name:
                opf.write(f"        let r = std::panic::catch_unwind(|| {{\n            let _ = jvozba::get_lujvo(test, false);\n        }});\n")
                opf.write(f"        assert!(r.is_err() || jvozba::get_lujvo(test, false).is_err());\n")
            if "K" in list_name:
                opf.write(f"        let r = std::panic::catch_unwind(|| {{\n            let _ = katna::get_veljvo(test);\n        }});\n")
                opf.write(f"        assert!(r.is_err() || katna::get_veljvo(test).is_err());\n")
        else:
            opf.write("        println!(\"{} / {}\", test.0, test.1);\n")
            if "Z" in list_name or "M" in list_name:
                opf.write("        assert_eq!(test.0, jvozba::get_lujvo(test.1, " + str("M" in list_name).lower() + ").unwrap().0);\n")
                opf.write("        println!(\"zbasu: pass\");\n")
            if "K" in list_name or "M" in list_name:
                opf.write("        assert_eq!(katna::get_veljvo(test.0).unwrap().join(\" \"), test.1);\n")
                opf.write("        println!(\"katna: pass\");\n")
            if "R" in list_name:
                opf.write("        assert_eq!(jvozba::get_lujvo(test.0, false).unwrap().1, test.1);\n")
                opf.write("        println!(\"score: pass\");\n")
        opf.write("    }\n}\n")
from latkerlo_jvotci import *

print(get_lujvo("mlatu kerlo"))
print(get_lujvo(["mlatu", "kerlo"]))
# -> "latkerlo"

print(get_lujvo("mlatu kerlo", generate_cmevla=True))
print(get_lujvo(["mlatu", "kerlo"], generate_cmevla=True))
# -> "latker"

print(get_veljvo("latkerlo"))  # -> ["mlatu", "kerlo"]

print(is_brivla("latkerlo"))  # -> True
print(is_brivla("bisladru"))  # -> False
print(is_brivla("latker"))    # -> False

print(analyse_brivla("tcanyja'a"))  # -> ("LUJVO", ["tcan", "y", "ja'a"])
print(analyse_brivla("re'ertren"))  # -> ("CMEVLA", ["re'e", "r", "tren"])
print(analyse_brivla("mlatu"))      # -> ("GISMU", ["mlatu"])
try:
    print(analyse_brivla("latkello"))   # -> raises NotBrivlaError
except NotBrivlaError as e:
    print(f"Error: {e}")

print(get_lujvo_with_analytics("mlatu kerlo"))
# -> ["latkerlo", 7937, [(0, 3), (3, 8)]]

print(get_lujvo_with_analytics("tcana jatna"))
# -> ["tcanyja'a", 8597, [(0, 4), (5, 9)]]

print(get_rafsi_indices(["lat", "kerlo"]))
# -> [(0, 3), (3, 8)]

b_type, decomp = analyse_brivla("tcanyja'a")
print(get_rafsi_indices(decomp))
# -> [(0, 4), (5, 9)]

print(get_veljvo("le'e'e'ygimzu",
                 y_hyphens="ALLOW_Y",
                 exp_rafsi_shapes=True,
                 consonants="TWO_CONSONANTS",
                 glides=True,
                 allow_mz=True))
# -> ["-le'e'e-", "gimzu"]

print(get_veljvo("le'e'e'ygimzu", "FORCE_Y", True, "CLUSTER", False, True))
# -> ["-le'e'e-", "gimzu"]

try:
    print(get_veljvo("le'e'e'ygimzu", allow_mz=True))
except NotBrivlaError as e:
    print(f"Error: {e}")

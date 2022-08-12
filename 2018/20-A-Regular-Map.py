with open("20_input.txt", "r") as fp:
    input_raw = fp.read().strip()


def expand_longest_path(pattern: str) -> int:
    pattern = pattern[1:-1]
    exp_end: int = pattern.find(")")
    while exp_end != -1:
        exp_start = pattern[:exp_end].rfind("(")
        split = pattern[exp_start + 1:exp_end].split("|")
        split.sort(key=len)
        replace: str = ""
        if split[0] != "":
            replace = split[-1]
        pattern = pattern[:exp_start] + replace + pattern[exp_end + 1:]
        exp_end = pattern.find(")")
    return len(pattern)


input_test1 = "^WNE$"
input_test2 = "^ENWWW(NEEE|SSE(EE|N))$"
input_test3 = "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$"
input_test4 = "^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$"
input_test5 = "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$"

print(f"Test  (3): {expand_longest_path(input_test1)}")
print(f"Test (10): {expand_longest_path(input_test2)}")
print(f"Test (18): {expand_longest_path(input_test3)}")
print(f"Test (23): {expand_longest_path(input_test4)}")
print(f"Test (31): {expand_longest_path(input_test5)}")

print(f"Part One: {expand_longest_path(input_raw)}")

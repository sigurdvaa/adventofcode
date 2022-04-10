input_raw = "To continue, please consult the code grid in the manual.  Enter the code at row 2981, column 3075."


def parse_code_xy(string: str) -> tuple:
    split = string.split()
    return (int(split[-1][:-1]), int(split[-3][:-1]))


def find_code(x_end: int, y_end: int) -> int:
    code = 20151125
    multiply = 252533
    mod = 33554393

    steps = sum(range(x_end + 1)) + sum(range(x_end, y_end + x_end - 1)) - 1
    for s in range(steps):
        code = (code * multiply) % mod

    return code


code_xy = parse_code_xy(input_raw)
print(f"Part One: { find_code(code_xy[0], code_xy[1]) }")

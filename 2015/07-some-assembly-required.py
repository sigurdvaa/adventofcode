with open("07-input.txt", "r") as f:
    input_raw = [x.strip() for x in f.readlines()]


def parse_gates(gates_description: list):
    gates = dict()
    for gate in gates_description:
        split = gate.split("->")
        wire_out = split[1][1:]
        wire_in_split = split[0].split()

        gates[wire_out] = {
            "input_a": "",
            "input_b": "",
            "logic": "",
            "signal": None,
        }

        if len(wire_in_split) == 1:
            gates[wire_out]["input_a"] = wire_in_split[0]
        elif len(wire_in_split) == 2:
            gates[wire_out]["input_a"] = wire_in_split[1]
            gates[wire_out]["logic"] = wire_in_split[0]
        elif len(wire_in_split) == 3:
            gates[wire_out]["input_a"] = wire_in_split[0]
            gates[wire_out]["input_b"] = wire_in_split[2]
            gates[wire_out]["logic"] = wire_in_split[1]

    return gates


def get_signal(gates: dict, wire: str):

    if gates[wire]["signal"] != None:
        return gates[wire]["signal"]

    if gates[wire]["input_a"] != "":
        if gates[wire]["input_a"].isdigit():
            input_a = int(gates[wire]["input_a"])
        else:
            input_a = get_signal(gates, gates[wire]["input_a"])
    if gates[wire]["input_b"] != "":
        if gates[wire]["input_b"] != "" and gates[wire]["input_b"].isdigit():
            input_b = int(gates[wire]["input_b"])
        else:
            input_b = get_signal(gates, gates[wire]["input_b"])

    if gates[wire]["logic"] == "":
        signal = input_a
    elif gates[wire]["logic"] == "AND":
        signal = input_a & input_b
    elif gates[wire]["logic"] == "OR":
        signal = input_a | input_b
    elif gates[wire]["logic"] == "LSHIFT":
        signal = input_a << int(input_b)
    elif gates[wire]["logic"] == "RSHIFT":
        signal = input_a >> int(input_b)
    elif gates[wire]["logic"] == "NOT":
        signal = abs(~ input_a)

    gates[wire]["signal"] = signal
    return signal


gates = parse_gates(input_raw)
signal_a = get_signal(gates, 'a')
print(f"Part One: {signal_a}")

gates = parse_gates(input_raw)
gates["b"]["signal"] = signal_a
print(f"Part Two: {get_signal(gates, 'a')}")

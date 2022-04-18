with open("07_input.txt", "r") as f:
    input_raw = f.read()


_input_raw = """Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin."""


def parse_steps(string: str) -> dict:
    from collections import namedtuple
    Step = namedtuple("Step", ["parents", "children"])
    steps = {}
    for line in string.splitlines():
        split = line.split()
        parent = split[1]
        child = split[7]

        if parent not in steps:
            steps[parent] = Step([], [child])
        else:
            steps[parent].children.append(child)

        if child not in steps:
            steps[child] = Step([parent], [])
        else:
            steps[child].parents.append(parent)
    
    return steps


def step_available(step: tuple, order: list):
    for p in step.parents:
        if p not in order:
            return False
    return True


def steps_order(steps: dict) -> str:
    next_steps = []
    for s in steps:
        if len(steps[s].parents) == 0:
            next_steps.append(s)
    next_steps.sort()

    order = []
    while len(next_steps):
        s = next_steps.pop(0)
        if step_available(steps[s], order):
            order.append(s)
            for c in steps[s].children:
                if c not in next_steps:
                    next_steps.append(c)
            next_steps.sort()
        else:
            next_steps.append(s)

    return "".join(order)


steps = parse_steps(input_raw)
print(f"Part One: { steps_order(steps) }")

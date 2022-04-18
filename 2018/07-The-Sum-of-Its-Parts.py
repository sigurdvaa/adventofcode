from string import ascii_uppercase
from collections import namedtuple

Step = namedtuple("Step", ["parents", "children", "duration"])
Worker = namedtuple("Worker", ["duration", "step"])

with open("07_input.txt", "r") as f:
    input_raw = f.read()


def parse_steps(string: str, duration: int = 60) -> dict:
    Step = namedtuple("Step", ["parents", "children", "duration"])
    steps = {}
    for line in string.splitlines():
        split = line.split()
        parent = split[1]
        child = split[7]

        if parent not in steps:
            steps[parent] = Step(
                [], [child], duration + ascii_uppercase.index(parent) + 1
            )
        else:
            steps[parent].children.append(child)

        if child not in steps:
            steps[child] = Step(
                [parent], [], duration + ascii_uppercase.index(child) + 1
            )
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


def add_worker(steps: dict, next_steps: list, order: list) -> tuple:
    for s in next_steps:
        if step_available(steps[s], order):
            next_steps.remove(s)
            return Worker(steps[s].duration, s)
    return None


def steps_duration(steps: dict, int_workers: int = 5) -> int:
    next_steps = []
    for s in steps:
        if len(steps[s].parents) == 0:
            next_steps.append(s)
    next_steps.sort()

    order = []
    workers = []
    total_duration = 0
    while True:
        if len(workers) < int_workers:
            new_worker = add_worker(steps, next_steps, order)
            if new_worker:
                workers.append(new_worker)
                continue

            if len(workers) == 0:
                break

        for i in reversed(range(len(workers))):
            workers[i] = Worker(workers[i].duration - 1, workers[i].step)
            if workers[i].duration == 0:
                step = workers[i].step
                order.append(step)
                for child in steps[step].children:
                    if child not in next_steps:
                        next_steps.append(child)
                next_steps.sort()
                del workers[i]

        total_duration += 1

    return total_duration


steps = parse_steps(input_raw)
print(f"Part One: { steps_order(steps) }")
print(f"Part Two: { steps_duration(steps) }")

input_raw = """Disc #1 has 13 positions; at time=0, it is at position 11.
Disc #2 has 5 positions; at time=0, it is at position 0.
Disc #3 has 17 positions; at time=0, it is at position 11.
Disc #4 has 3 positions; at time=0, it is at position 0.
Disc #5 has 7 positions; at time=0, it is at position 2.
Disc #6 has 19 positions; at time=0, it is at position 17."""

discslots = []
discpos = []
for line in input_raw.splitlines():
  split = line.split()
  discslots += [int(split[3])]
  discpos += [int(split[11][:-1])]

def reach_bottom(discslots:list, discpos:list, time:int):
  for i in range(len(discslots)):
    if (discpos[i]+1+i+time) % discslots[i] != 0:
      return False
  return True

def timing_capsule(discslots:list, discpos:list):
  time = 0
  while True:
    if reach_bottom(discslots, discpos, time):
      return time
    time += 1

print(f"Part One: {timing_capsule(discslots, discpos)}")

discslots += [11]
discpos += [0]
print(f"Part Two: {timing_capsule(discslots, discpos)}")


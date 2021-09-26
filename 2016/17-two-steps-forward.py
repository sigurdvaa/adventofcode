from hashlib import md5

input_raw = "lpvhkcbi"

def valid_pos(grid:tuple, pos:tuple):
  if not -1 < pos[0] <= grid[0]:
    return False
  if not -1 < pos[1] <= grid[1]:
    return False
  return True

def next_valid_states(queue:list, dooropen:list, grid:tuple, state:list):
  pos = state[1]
  doors = md5(state[0].encode()).hexdigest()[:4]
  
  if doors[0] in dooropen:
    new_pos = (pos[0], pos[1]-1)
    if valid_pos(grid, new_pos):
      queue += [[ state[0]+"U", new_pos ]]
  if doors[1] in dooropen:
    new_pos = (pos[0], pos[1]+1)
    if valid_pos(grid, new_pos):
      queue += [[ state[0]+"D", new_pos ]]
  if doors[2] in dooropen:
    new_pos = (pos[0]-1, pos[1])
    if valid_pos(grid, new_pos):
      queue += [[ state[0]+"L", new_pos ]]
  if doors[3] in dooropen:
    new_pos = (pos[0]+1, pos[1])
    if valid_pos(grid, new_pos):
      queue += [[ state[0]+"R", new_pos ]]

def find_path(password:str, longest:bool=False):
  dooropen = "bcdef"
  grid = (3,3)
  target = (3,3)
  start_state = [password, (0,0)]
  queue = [start_state]
  longest_path = 0

  while len(queue):
    state = queue.pop(0)
    if state[1] == target:
      if longest:
        longest_path = len(state[0]) - len(password)
      else:
        return state[0][len(password):]
    else:
      next_valid_states(queue, dooropen, grid, state)
  
  if longest:
    return longest_path
  else:
    return "No solution"

print(f"Part One: {find_path(input_raw)}")
print(f"Part Two: {find_path(input_raw, longest=True)}")

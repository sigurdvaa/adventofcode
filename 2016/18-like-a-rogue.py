input_raw = "......^.^^.....^^^^^^^^^...^.^..^^.^^^..^.^..^.^^^.^^^^..^^.^.^.....^^^^^..^..^^^..^^.^.^..^^..^^^.."

def get_next_row(consider_trap:set, row:list):
  next_row = list()
  # leftside
  if "."+row[:2] in consider_trap:
    next_row.append("^")
  else:
    next_row.append(".")
  # middle"
  for i in range(len(row)-2):
    if row[i:i+3] in consider_trap:
      next_row.append("^")
    else:
      next_row.append(".")
  # rightside
  if row[-2:]+"." in consider_trap:
    next_row.append("^")
  else:
    next_row.append(".")

  return "".join(next_row)

def safe_tiles(first_row:list, totalrows:int):
  consider_trap = { "^^.", ".^^", "^..", "..^" }
  rows = list()
  rows.append(first_row)
  for i in range(totalrows-1):
    rows.append(get_next_row(consider_trap, rows[i]))
  
  return sum([y.count(".") for y in rows])

print(f"Part One: {safe_tiles(input_raw, 40)}")
print(f"Part Two: {safe_tiles(input_raw, 400000)}")

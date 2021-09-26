input_raw = "01111001100111011"
disk_size_p1 = 272
disk_size_p2 = 35651584

def checksum(data:str):
  consider = list(data)
  while (len(consider) % 2) == 0:
    new_consider = list()
    for i in range(0, len(consider), 2):
      if consider[i] == consider[i+1]:
        new_consider.append("1")
      else:
        new_consider.append("0")
    consider = new_consider
  return "".join(consider)

def dragon_curve(data:str):
  reverse = list(data)
  reverse.reverse()
  for i in range(len(reverse)):
    if reverse[i] == "0":
      reverse[i] = "1"
    else:
      reverse[i] = "0"
  return data + "0" + "".join(reverse)

def random_data(data:str, size:int):
  while len(data) < size:
    data = dragon_curve(data)
  return checksum(data[:size])

print(f"Part One: {random_data(input_raw, disk_size_p1)}")
print(f"Part Two: {random_data(input_raw, disk_size_p2)}")

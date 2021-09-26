from hashlib import md5

input_raw = "jlmsuwbz"

def valid_key(keyhashes:list, idx:int, streamlen:int):
  key = keyhashes[idx]
  for k in range(len(key) - 2):
    if key[k]*3 == key[k:k+3]:
      fivechar = key[k]*5
      for s in range(idx + 1, idx + 1 + streamlen):
        if fivechar in keyhashes[s]:
          return True
      return False
  return False

def generate_keys(salt:str, amount:int, stretch:int=0):
  idx = 0
  streamlen = 1000
  keyhashes = []
  validkeys = []
  while len(validkeys) < amount:
    key = md5((salt + str(idx)).encode("utf8")).hexdigest()
    for _ in range(stretch):
      key = md5(key.encode("utf8")).hexdigest()
    keyhashes.append(key)
    if len(keyhashes) > streamlen:
      if valid_key(keyhashes, idx - streamlen, streamlen):
        validkeys.append(keyhashes[idx - streamlen])
    idx += 1
  return idx - streamlen - 1

print(f"Part One: {generate_keys(input_raw, 64)}")
print(f"Part Two: {generate_keys(input_raw, 64, 2016)}")

#steps = 3
steps = 343

def spinlock(iterations):
    buffer = [0]
    pos, step = 0, steps + 1
    for i in range(1, iterations+1):
        buffer.insert(pos+1, i)
        pos = (pos + step) % (i+1)
    return buffer     

def spinlockvalueafter(iterations, index):
    # steps+1 and index-1 instead of (pos + step) % i+1 for less actions inside loop
    pos, value, step, index = 0, 0, steps + 1, index - 1
    for i in range(1,iterations+1):
        pos = (pos + step) % i
        if pos == index:
            value = i
    return value

print("Part 1")
buffer = spinlock(2017)
print(buffer[buffer.index(2017)+1])

print("Part 2")
# value 0 is always at index 0, so we want index 1
print(spinlockvalueafter(50000000, 1))

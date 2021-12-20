with open("04_input.txt", "r") as f:
    input_raw = f.read()


def isAnagram(word1, word2):
    length = len(word1)
    word1 = list(word1)
    word2 = list(word2)
    i1 = 0
    i2 = 0
    while True:
        if word1[i1] == word2[i2]:
            word1.pop(i1)
            word2.pop(i2)
            length = len(word1)
            if length == 0:
                return True
            i1 = 0
            i2 = 0
        else:
            i2 += 1
            if i2 >= length:
                i1 += 1
                i2 = 0
                if i1 >= length:
                    return False


print("Part 1")
numValid = 0
for string in input_raw.splitlines():
    valid = True
    wordList = string.split(" ")
    length = len(wordList)
    for w1 in range(length):
        for w2 in range(w1 + 1, length):
            if wordList[w1] == wordList[w2]:
                valid = False
    if valid:
        numValid += 1
print(numValid)

print("Part 2")
numValid = 0
for string in input_raw.splitlines():
    valid = True
    wordList = string.split(" ")
    length = len(wordList)
    for w1 in range(length):
        for w2 in range(w1 + 1, length):
            if len(wordList[w1]) == len(wordList[w2]):
                if isAnagram(wordList[w1], wordList[w2]):
                    valid = False
    if valid:
        numValid += 1
print(numValid)

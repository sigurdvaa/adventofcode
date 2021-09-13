import sys
sys.setrecursionlimit(10000)

with open('09-input.txt', 'r') as f:
    input_raw = f.readline().strip()

decompressed = []
input_length = len(input_raw)
i = 0
while i < input_length:
    if input_raw[i] == "(":
        
        m = 4
        marker_found = False
        while not marker_found:
            if input_raw[i+m] == ")":
                marker_found = True
            else:
                m += 1

        marker_split = input_raw[i+1:i+m].split("x")
        seq_length = int(marker_split[0])
        repeat = int(marker_split[1])
    
        seq = input_raw[i+m+1:i+m+1+seq_length] * repeat
        decompressed += [seq]
        
        i += 1 + m + seq_length

    else:
        decompressed += [input_raw[i]]
        i += 1

print("Part One: "+str(len("".join(decompressed))))

def decompressed_length(string):
    
    decomp_len = 0
    string_length = len(string)
    i = 0
    while i < string_length:
        if string[i] == "(":
            
            m = 4
            marker_found = False
            while not marker_found:
                if string[i+m] == ")":
                    marker_found = True
                else:
                    m += 1

            marker_split = string[i+1:i+m].split("x")
            seq_length = int(marker_split[0])
            repeat = int(marker_split[1])
        
            substring = string[i+m+1:i+1+m+seq_length] * repeat
            decomp_len += decompressed_length(substring)

            i += 1 + m + seq_length

        else:
            decomp_len += 1
            i += 1
    return decomp_len

print("Part Two: "+str(decompressed_length(input_raw)))

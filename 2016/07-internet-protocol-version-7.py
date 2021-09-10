import re

with open('07-input.txt', 'r') as f:
    input_ips = [x.strip() for x in f.readlines()]

def abba_in_str(string):
    for i in range(len(string)-3):
        if string[i] == string[i+3]:
            if string[i] != string[i+1]:
                if string[i+1] == string[i+2]:
                    return True
    return False
    
def aba_in_str(string):
    aba = []
    for i in range(len(string)-2):
        if string[i] == string[i+2]:
            if string[i] != string[i+1]:
                aba += [string[i:i+3]]
    return aba

int_tls = 0
for ip in input_ips:
    ip_parts = re.findall("[a-z]+", ip)
    ip_supernets = ip_parts[::2]
    ip_hypernets = ip_parts[1::2]

    abba_in_hypernet = False
    for hypernet in ip_hypernets:
        if abba_in_str(hypernet):
            abba_in_hypernet = True
            break

    if not abba_in_hypernet:
        for supernet in ip_supernets:
            if abba_in_str(supernet):
                int_tls += 1
                break

print(f"Part One: {int_tls}")

int_ssl = 0
for ip in input_ips:
    ip_parts = re.findall("[a-z]+", ip)
    ip_supernets = ip_parts[::2]
    ip_hypernets = ip_parts[1::2]

    for supernet in ip_supernets:
        break_out = False
        supernet_aba = aba_in_str(supernet)
        for aba in supernet_aba:
            for hypernet in ip_hypernets:
                if aba[1]+aba[0]+aba[1] in hypernet:
                    int_ssl += 1
                    break_out = True
                    break
            if break_out:
                break
        if break_out:
            break

print(f"Part Two: {int_ssl}")

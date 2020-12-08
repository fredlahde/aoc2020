import random

with open("input", 'r') as fd:
    input = [d for d in fd.read().split('\n') if d.rstrip() != '']

def check_pw(min, max, c, pw):
    count = 0
    pos1 = pw[min-1] == c
    pos2 = pw[max-1] == c
    if pos1 and pos2:
        return False
    if pos1:
        return True
    if pos2:
        return True
    return False

correct = 0
for ii in input:
    split = ii.split(' ')
    minmax = str(split[0]).split('-')
    min,max = minmax[0], minmax[1]
    c = split[1][:1]
    b = check_pw(int(min), int(max), c,  split[2])
    if b:
        correct += 1
print(correct)



import random

with open("input", 'r') as fd:
    input = [d for d in fd.read().split('\n') if d.rstrip() != '']

def check_pw(min, max, c, pw):
    count = 0
    for cc in pw:
        if cc == c:
            count += 1
    print(count)
    if count < min:
        return False
    if count > max:
        return False
    return True

import re
correct = 0
for ii in input:
    split = ii.split(' ')
    minmax = str(split[0]).split('-')
    min,max = minmax[0], minmax[1]
    c = split[1][:1]
    print(int(min), int(max), c,  split[2])
    b = check_pw(int(min), int(max), c,  split[2])
    if b:
        correct += 1
print(correct)



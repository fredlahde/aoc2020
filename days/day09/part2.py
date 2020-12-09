import random
import re
import sys
import itertools

with open(sys.argv[1], 'r') as fd:
    input = [int(d) for d in fd.read().split('\n') if d.rstrip() != ""]

part1_result = 400480901

ptr1 = 0
ptr2 = len(input) -1

while ptr1 < len(input):
    while ptr2 > 0:
        check = input[ptr1:ptr2]
        if len(check) == 0:
            break
        s = sum(check)
        if s == part1_result:
            print("found", check)
            sor = sorted(check)
            print(sor[0] + sor[len(sor) -1])
            exit(0)
        ptr2 -= 1
    ptr1 += 1
    if input[ptr1] > part1_result:
        break
    ptr2 = len(input)-1

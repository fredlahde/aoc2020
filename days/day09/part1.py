import random
import re
import sys
import itertools

with open(sys.argv[1], 'r') as fd:
    input = [int(d) for d in fd.read().split('\n') if d.rstrip() != ""]

def check_add_up(nums, x):
    for ii in nums:
        for jj in nums:
            if (ii + jj) == x and ii != jj:
                return (ii, jj)
    return None

for ii in range(25, len(input)):
    num_to_check = input[ii]

    prev_5 = input[ii-25: ii]
    print(num_to_check, prev_5, check_add_up(prev_5, num_to_check))
    if check_add_up(prev_5, num_to_check) is None:
        print(num_to_check)
        break

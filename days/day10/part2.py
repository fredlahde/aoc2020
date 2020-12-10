import random
import re
import sys
import itertools
import collections

with open(sys.argv[1], 'r') as fd:
    input = [int(d) for d in fd.read().split('\n') if d.rstrip() != '']


sorted_outlets = sorted(input)
print(sorted_outlets)

combinations = collections.defaultdict(int, {0: 1})
for outlet in sorted_outlets:
    combinations[outlet] = combinations[outlet - 1] + combinations[outlet - 2]\
            + combinations[outlet - 3]

print(combinations[sorted_outlets[-1]])


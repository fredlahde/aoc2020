import random
import sys

with open("input", 'r') as fd:
    input = [d for d in fd.read().split('\n') if d.rstrip() != '']

#input = sys.argv[1:]
ROWS = ("F", "B")
COLS = ("L", "R")
class Index():
    def __init__(self, low, high, operands):
        self.low = low
        self.high = high
        self.range = high-low
        self.low_op = operands[0]
        self.high_op = operands[1]

    def step(self, c):
        new_range = self.range//2

        #print("new range", new_range, c)
        if c == self.low_op:
            new_low = self.low
            new_high = self.high - new_range -1
        if c == self.high_op:
            new_low = new_range + 1 + self.low
            new_high = self.high
        #print("%d - %d" % (new_low, new_high))
        return Index(new_low, new_high, (self.low_op, self.high_op))


heighest = 0
ids = []
for line in input:
    idx = Index(0, 127, ROWS)
    if ("L" in line) and ("R" in line):
        switch = min(line.index("L"),  line.index("R"))
    elif "L" in line:
        switch = line.index("L")
    elif "R" in line:
        switch = line.index("R")
    else:
        switch = len(line)
    for c in line[:switch]:
        idx = idx.step(c)
    row = idx.low
    idx = Index(0,7, COLS)
    for c in line[switch:]:
        idx = idx.step(c)
    col = idx.low

    idx = (row * 8) + col
    ids.append(idx)
    heighest = max(heighest, idx)


#print(heighest)

sorted_ids = sorted(list(set(ids)))

for s in range(sorted_ids[0], sorted_ids[len(sorted_ids)-1]):
    if s not in sorted_ids:
        if s+1 in sorted_ids and s-1 in sorted_ids:
            print(s)

#print(sorted_ids)
#print(all_ids)

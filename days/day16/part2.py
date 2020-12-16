import random
import re
import sys
from collections import OrderedDict

rules_re = re.compile(r"^([a-z\s]*):\s([0-9-]*)\sor\s([0-9-]*$)")

with open(sys.argv[1], 'r') as fd:
    chunks = [d.rstrip() for d in fd.read().split('\n\n')]

class Rule:
    def __init__(self, name, rules):
        self.name = name
        self.rules = rules

    def validate(self, xx):
        for rule in self.rules:
            if rule[0] <= xx <= rule[1]:
                return True
        return False

rules = []
for line in chunks[0].split("\n"):
    matcher = rules_re.match(line)
    name = matcher.group(1)
    ranges = []
    for xx in [matcher.group(2), matcher.group(3)]:
        rr = [int(x) for x in xx.split('-')]
        rr = (rr[0], rr[1])
        ranges.append(rr)
    rules.append(Rule(name, ranges))

invalid_sum = 0
valid_tickets = []
for line in chunks[2].split('\n'):
    if "nearby" in line:
        continue
    valid = True
    for value in [int(v) for v in line.split(',')]:
        value_valid = False
        for rule in rules:
            if rule.validate(value):
                value_valid = True
                break
        if not value_valid:
            valid = False
    if valid:
        valid_tickets.append(line)

cols = OrderedDict()
for ticket in valid_tickets:
    split = ticket.split(',')
    for (ii,ss) in enumerate(split):
        if ii in cols:
            cols[ii].append(ss)
        else:
            cols[ii] = [ss]

names = {}
for col in cols:
    numbers = cols[col]
    for rule in rules:
        valid =  True
        for nn in numbers:
            if not rule.validate(int(nn)):
                valid = False
        if valid:
            if col in names:
                names[col].append(rule.name)
            else:
                names[col] = [rule.name]

tmp = []
for nn in names.items():
    tmp.append((nn[0], len(nn[1])))
tmp.sort(key = lambda x: x[1])

# did exclusion of rules by hand with this output, see notes and part2_intemediate_output
for tt in tmp:
    print(tt[0], names[tt[0]])

for line in chunks[1].split('\n'):
    if "your" in line:
        continue

    numbers = [int(n) for n in line.split(',')]
    # offsets defined by hand with above output
    print(numbers[0] * numbers[10] * numbers[5] * numbers[4] * numbers[12] * numbers[3])

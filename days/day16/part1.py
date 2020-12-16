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
        print(rr)
        ranges.append(rr)
    rules.append(Rule(name, ranges))

invalid_sum = 0
for line in chunks[2].split('\n'):
    if "nearby" in line:
        continue
    for value in [int(v) for v in line.split(',')]:
        valid = False
        for rule in rules:
            if rule.validate(value):
                valid = True
                break
        if not valid:
            invalid_sum += value

print(invalid_sum)

    



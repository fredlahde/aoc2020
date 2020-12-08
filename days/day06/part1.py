import random
import re
import sys

# read the input into chunks
with open(sys.argv[1], 'r') as fd:
    input = [d for d in fd.read().split('\n\n')]

sum_answers = 0
for chunk in input:
    answers = list()
    for l in chunk.split('\n'):
        for c in l:
            answers.append(c)
    sum_answers += len(set(answers))

print(sum_answers)

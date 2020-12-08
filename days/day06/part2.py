import random
import re
import sys

# read the input into chunks
with open(sys.argv[1], 'r') as fd:
    input = [d for d in fd.read().split('\n\n')]

sum_answers = 0
for chunk in input:
    answers = dict()
    size_group = 0
    for l in chunk.split('\n'):
        size_group+= 1
        for c in l:
            if c in answers:
                answers[c] = answers[c] + 1
            else:
                answers[c] = 1
    for k,v in answers.items():
        if v == size_group:
            sum_answers += 1

print(sum_answers)

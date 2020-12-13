import random
import re
import sys
import copy

with open(sys.argv[1], 'r') as fd:
    input = [d for d in fd.read().split('\n') if d.rstrip() != '']

arrival = int(input[0])

busses = [int(b) for b in input[1].split(",") if b != "x"]

start_wait = arrival
while True:
    for b in busses:
        if start_wait % b == 0:
            wait = start_wait - arrival
            print(wait * b)
            exit(0)
    start_wait += 1



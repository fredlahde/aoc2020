import random
import re
import sys
import copy
from math import prod

with open(sys.argv[1], 'r') as fd:
    input = [d for d in fd.read().split('\n') if d.rstrip() != '']

busses = [b for b in input[1].split(",")]

bus_deps = [(int(b), i) for (i, b) in enumerate(busses) if b != "x"]

# TODO fully understand this... 
def solve_with_CRT(bus_deps):
    # https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
    invm = lambda a, b: 0 if a==0 else 1 if b%a==0 else b - invm(b%a,a)*b//a
    # https://en.wikipedia.org/wiki/Chinese_remainder_theorem
    N = prod([bs[0] for bs in bus_deps])
    x = sum([bs[1]*(N//bs[0])*invm(N//bs[0], bs[0]) for bs in bus_deps])
    return N - x % N

print(solve_with_CRT(bus_deps))

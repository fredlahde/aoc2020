import random
import sys

with open("input", 'r') as fd:
    input = [d for d in fd.read().split('\n') if d.rstrip() != '']

heighest = 0
translator = str.maketrans("FBLR", "0101")

for line in input:
    line = line.translate(translator)
    row = int(line[:7], 2)
    col = int(line[7:], 2)
    seat_id = (row * 8) + col
    heighest = max(heighest, seat_id)

print(heighest)

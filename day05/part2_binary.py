import random
import sys

with open("input", 'r') as fd:
    input = [d for d in fd.read().split('\n') if d.rstrip() != '']

translator = str.maketrans("FBLR", "0101")

ids = []
for line in input:
    line = line.translate(translator)
    row = int(line[:7], 2)
    col = int(line[7:], 2)
    seat_id = (row * 8) + col
    ids.append(seat_id)

sorted_ids = sorted(ids)

for ii in range(len(sorted_ids)-1):
    if ii not in sorted_ids and ii-1 in sorted_ids and ii+1 in sorted_ids:
        print(ii)
        break

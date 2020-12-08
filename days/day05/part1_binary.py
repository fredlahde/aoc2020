import random
import sys

with open("input", 'r') as fd:
    input = [d for d in fd.read().split('\n') if d.rstrip() != '']

# keep track of the highest seat id
highest = 0

# create translator to convert seats from the boarding passes to binary strings
translator = str.maketrans("FBLR", "0101")

# go over each seat in the input
for seat in input:
    # translate the seat into a binary string
    binary_seat = seat.translate(translator)

    # the first 7 bits define the row
    row = int(binary_seat[:7], 2)

    # the remainder of the bits define the column of the seat
    col = int(binary_seat[7:], 2)

    # calculate the seat_id
    seat_id = (row * 8) + col

    # track the highest seat_id
    highest = max(highest, seat_id)

print(highest)

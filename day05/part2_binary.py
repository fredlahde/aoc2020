import random
import sys

with open("input", 'r') as fd:
    input = [d for d in fd.read().split('\n') if d.rstrip() != '']

# create translator to convert seats from the boarding passes to binary strings
translator = str.maketrans("FBLR", "0101")

# vector of all seat_ids with a boarding pass
boarded_ids = []

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

    # keep track of the seat_id as boarded
    boarded_ids.append(seat_id)


# go over the range of boarded seat_ids
sorted_ids = sorted(boarded_ids)
for seat_id in range(len(sorted_ids)-1):
    # if the seat_id is not boarded, but the seats right next to it are, we have found our 
    # 'missing' seat
    if seat_id not in sorted_ids and seat_id-1 in sorted_ids and seat_id+1 in sorted_ids:
        print(seat_id)
        break

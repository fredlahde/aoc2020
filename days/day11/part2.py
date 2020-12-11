import random
import re
import sys
import copy

with open(sys.argv[1], 'r') as fd:
    input = [[c for c in d] for d in fd.read().split('\n') if d.rstrip() != '']

"""
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
"""
FLOOR = '.'
EMPTY_SEAT = 'L'
OCCUPIED_SEAT = '#'

grid = (len(input), len(input[0]))
#exit(0)

def print_grid(g):
    for xx in g:
        for yy in xx:
            print(yy, end="")
        print()
    print()

def compare_grids(g1, g2):
    for xx in range(len(g1)):
        for yy in range(len(g1[0])):
            if g1[xx][yy] != g2[xx][yy]:
                return False
    return True

def count_occupied(g):
    occupied = 0
    for xx in range(len(g)):
        for yy in range(len(g[0])):
            if g[xx][yy] == OCCUPIED_SEAT:
                occupied += 1
    return occupied

def get_first_seat_in_direction(gg, pos, d):
    new_pos = (pos[0]+d[0], pos[1]+d[1])
    if new_pos[0] >= 0 and new_pos[1] >= 0 and new_pos[0] < grid[0] and new_pos[1] < grid[1]:
        if gg[new_pos[0]][new_pos[1]] != FLOOR:
            return gg[new_pos[0]][new_pos[1]]
    while True:
        new_pos = (new_pos[0]+d[0], new_pos[1]+d[1])
        print(new_pos)
        if new_pos[0] >= 0 and new_pos[1] >= 0 and new_pos[0] < grid[0] and new_pos[1] < grid[1]:
            if gg[new_pos[0]][new_pos[1]] != FLOOR:
                return gg[new_pos[0]][new_pos[1]]
        else:
            return None

def check_adjacent(pos, read_grid, write_grid):
    occupied = 0

    directions = [
        (-1,-1),
        (1,1),
        (-1, 1),
        (1, -1),
        (0, 1),
        (1, 0),
        (-1, 0),
        (0, -1)
    ]
    assert len(directions)== 8
    for new_dir in directions:
        if get_first_seat_in_direction(read_grid, pos, new_dir) == OCCUPIED_SEAT:
            occupied += 1

    if read_grid[pos[0]][pos[1]] == EMPTY_SEAT and occupied == 0:
        write_grid[pos[0]][pos[1]] = OCCUPIED_SEAT
        return

    if read_grid[pos[0]][pos[1]] == OCCUPIED_SEAT and occupied >= 5:
        write_grid[pos[0]][pos[1]] = EMPTY_SEAT


read_grid = copy.deepcopy(input)
write_grid = copy.deepcopy(input)
while True:
    for xx in range(grid[0]):
        for yy in range(grid[1]):
            check_adjacent((xx,yy), read_grid, write_grid)

    #print_grid(write_grid)
    if compare_grids(read_grid, write_grid):
        break
    read_grid = copy.deepcopy(write_grid)
    write_grid = copy.deepcopy(read_grid)

print(count_occupied(read_grid))


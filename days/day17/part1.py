# today i wasn't quite able to wrap my head around the puzzle and also low on time
# i saw this solution on reddit https://www.reddit.com/r/adventofcode/comments/keqsfa/2020_day_17_solutions/gg5bwoe/?utm_source=reddit&utm_medium=web2x&context=3 
# so I thought i'd be a good idea to take that as a base to learn about convoultions and numpy

import numpy as np
from scipy.signal import convolve
np.set_printoptions(precision=4,
                       threshold=10000,
                       linewidth=150)

DIMS = 4
lines = [[(1 if x == '#' else 0) for x in x.strip()] for x in open("inputs/input")]

grid = np.array(lines, dtype=np.byte)
grid = np.expand_dims(grid, axis=tuple(range(DIMS-2)))
kernel = np.ones((3, )*DIMS, dtype=np.byte)
kernel[(1, )*DIMS] = 0

for _ in range(6):
    grid = np.pad(grid, pad_width=1, mode='constant', constant_values=0)
    neighbors = convolve(grid, kernel, mode='same')
    set_inactive = np.logical_and(grid == 1, np.floor_divide(neighbors, 2) != 1)
    set_active = np.logical_and(grid == 0, neighbors == 3)
    grid[set_inactive] = 0
    grid[set_active] = 1

print(np.sum(grid))

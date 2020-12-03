import random

with open("input", 'r') as fd:
    input = [d for d in fd.read().split('\n') if d.rstrip() != '']

trees = 0
x,y = (0,0)
while True:
    try:
        x += 3
        x = (x % len(input[0]))
        y += 1
        square = input[y][x]
        if square == '#':
            trees+= 1
    except IndexError:
        break

print(trees)

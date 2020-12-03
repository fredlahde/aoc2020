import random

with open("input", 'r') as fd:
    input = [d for d in fd.read().split('\n') if d.rstrip() != '']

print(input)

trees_acc = []

slopes = [(1,1), (3,1), (5,1), (7,1), (1,2)]
for slope in slopes:
    trees = 0
    x,y = (0,0)
    while True:
        try:
            x += slope[0]
            x = (x % len(input[0]))
            y += slope[1]
            square = input[y][x]
            if square == '#':
                trees += 1
        except IndexError:
            trees_acc.append(trees)
            print(trees_acc)
            break

s = 1
for t in trees_acc:
    s *= t

print(s)

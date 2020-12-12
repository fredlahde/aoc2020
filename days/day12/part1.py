import random
import re
import sys
import copy

with open(sys.argv[1], 'r') as fd:
    input = [d for d in fd.read().split('\n') if d.rstrip() != '']

#input = ["F10", "N3", "F7", "R90", "F11"]

class Ship():
    def __init__(self):
        self.facing = 0
        self.east = 0
        self.north = 0

    def move(self, cmd):
        if (cmd[0] == "N"):
            self.north += int(cmd[1:])

        if (cmd[0] == "S"):
            self.north -= int(cmd[1:])

        if (cmd[0] == "E"):
            self.east += int(cmd[1:])

        if (cmd[0] == "W"):
            self.east -= int(cmd[1:])

        if (cmd[0] == "L"):
            self.facing -= int(cmd[1:])

        if (cmd[0] == "R"):
            self.facing += int(cmd[1:])

        if self.facing >= 360 or self.facing <= -360:
            self.facing = self.facing % 360
        
        if self.facing > 270 or self.facing < -270:
            print("Error", self.facing, cmd)
            exit(0)

        if (cmd[0] ==  "F"):
            if self.facing == 0:
                self.east += int(cmd[1:])
            if self.facing == 90 or self.facing == -270:
                self.north -= int(cmd[1:])
            if self.facing == 180 or self.facing == -180:
                self.east -= int(cmd[1:])
            if self.facing == 270 or self.facing == -90:
                self.north += int(cmd[1:])

    def __str__(self):
        return "facing %d east %d north %d" % (self.facing, self.east, self.north)

ship = Ship()
for l in input:
    ship.move(l)
    print(ship)

print(abs(ship.east) + abs(ship.north))



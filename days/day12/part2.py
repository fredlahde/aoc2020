import random
import re
import sys
import copy
import math

with open(sys.argv[1], 'r') as fd:
    input = [d for d in fd.read().split('\n') if d.rstrip() != '']

def rotate(origin, point, angle):
    ox, oy = origin
    px, py = point
    qx = ox + math.cos(angle) * (px - ox) - math.sin(angle) * (py - oy)
    qy = oy + math.sin(angle) * (px - ox) + math.cos(angle) * (py - oy)
    return int(round(qx)), int(round(qy))

class Ship():
    def __init__(self):
        self.east = 0
        self.north = 0

        self.waypoint_east = 10
        self.waypoint_north = 1

    def move(self, cmd):
        if (cmd[0] == "N"):
            self.waypoint_north += int(cmd[1:])

        if (cmd[0] == "S"):
            self.waypoint_north -= int(cmd[1:])

        if (cmd[0] == "E"):
            self.waypoint_east += int(cmd[1:])

        if (cmd[0] == "W"):
            self.waypoint_east -= int(cmd[1:])

        if cmd[0] in ["L", "R"]:
            val = int(cmd[1:])
            if cmd[0] == "R":
                val  = -val

            self.waypoint_east, self.waypoint_north = rotate(\
                    (0,0), (self.waypoint_east, self.waypoint_north), math.radians(val))


        if (cmd[0] ==  "F"):
            val = int(cmd[1:])
            self.east += self.waypoint_east * val
            self.north += self.waypoint_north * val


    def __str__(self):
        return "east %d north %d waypint north %d waypoint east %d"\
                % (self.east, self.north, self.waypoint_north, self.waypoint_east)

ship = Ship()
for l in input:
    ship.move(l)
    print(ship)

print(abs(ship.east) + abs(ship.north))



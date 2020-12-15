import random
import re
import sys
from collections import OrderedDict

spoken = {}
lastNumber = None

turn = 0 

#starting = [0,3,6]
starting = [18,11,9,0,5,1]
debug = lambda turn, to_speak: turn
while turn != 30000000:
    if (turn +1) <= len(starting):
        to_speak = starting[turn]
        debug(turn, to_speak)
        if to_speak in spoken:
            spoken[to_speak].append(turn)
        else:
            spoken[to_speak] = [turn]
        lastNumber = starting[turn]
        turn += 1
        continue

    if lastNumber in spoken:
        turns_before = spoken[lastNumber]
        if len(turns_before) == 1:
            spoken[0].append(turn)
            lastNumber = 0
            debug(turn, to_speak)
        else:
            to_speak = turns_before[-1] - turns_before[-2]
            if to_speak in spoken:
                spoken[to_speak].append(turn)
            else:
                spoken[to_speak] = [turn]
            lastNumber = to_speak
            debug(turn, to_speak)
    else:
        spoken[lastNumber] = [turn]
        lastNumber = 0
    turn += 1

print(lastNumber)

import random
import re
import sys

with open(sys.argv[1], 'r') as fd:
    input = [int(d) for d in fd.read().split('\n') if d.rstrip() != '']

jolt_outlet = 0
sorted_input = sorted(input)
jolt_device = sorted_input[len(sorted_input) -1] + 3
max_delta_jolt = 3

print("device: ", jolt_device)
deltas = []

curr_jolt =0
deltas = []
adapter = []

while len(input) > 0:
        choices = {}
        for ii in range(len(input)):
            x = input[ii]
            dx = abs(x - curr_jolt)
            if dx in [1,2,3]:
                choices[dx] = x

        print(choices)
        choices_keys = sorted(list(choices.keys()))
        lowest_choices_key = choices_keys[0]
        choosen_adapter = choices[lowest_choices_key]
        curr_jolt = choosen_adapter
        input.remove(choosen_adapter)
        deltas.append(lowest_choices_key)
        adapter.append(choosen_adapter)


          
        print(curr_jolt, len(input), adapter, deltas, input)


ones = 0
threes = 0
for d in deltas:
    if d == 1:
        ones+= 1
    elif d == 3:
        threes += 1
print(deltas)
print(ones, threes, ones*(threes+1))

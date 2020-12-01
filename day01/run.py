import random

with open("input", 'r') as fd:
    input = [int(d) for d in fd.read().split('\n') if d.rstrip() != '']

while True:
    a1 = random.choice(input)
    a2 = random.choice(input)
    a3 = random.choice(input)
    
    if (a1 + a2 + a3 == 2020):
        print(a1*a2*a3)
        exit(0)




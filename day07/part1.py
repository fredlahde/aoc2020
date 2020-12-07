import random
import re
import sys

carry_pattern = re.compile(r"\s(\d{1,2})\s(.*)bag")

# read the input into chunks
with open(sys.argv[1], 'r') as fd:
    input = [d for d in fd.read().split('\n') if d.rstrip() != '']

def get_bag_of_color(bags, color):
    for b in bags:
        if color == b.color:
            return b
    return None

class Bag(object):
    def __init__(self, color, carry):
        self.color = color
        self.carry = carry

    def can_hold_shiny_gold_bag(self, bags):
        if "shiny gold" in color:
            return False
        for cc in self.carry:
            if "shiny gold" in cc:
                return True
            
            #print("%s is checking %s" % (self.color, cc))
            other_bag = get_bag_of_color(bags, cc)
            if other_bag is None:
                #print("failed to find other bag: %s" % cc)
                continue

            other_bag_can_hold = other_bag.can_hold_shiny_gold_bag(bags)
            if other_bag_can_hold:
                return True

        return False

bags = []
for line in input:
    if "no other bags" in line:
        continue;
    color_self, color_carry = line.split("bags contain")
    color_self = color_self.rstrip()

    carry_split = color_carry.rstrip().split(",")
    carry_dict = dict()
    for carry in carry_split:
        cc = carry.rstrip()
        cc_match = carry_pattern.match(cc)
        
        count,color= (cc_match.group(1), cc_match.group(2))
        carry_dict[color.rstrip()] = int(count)
    bag = Bag(color_self, carry_dict)
    bags.append(bag)

count = 0
for b in bags:
    if b.can_hold_shiny_gold_bag(bags):
        count += 1

print(count)

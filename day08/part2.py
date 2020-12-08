import random
import re
import sys

carry_pattern = re.compile(r"\s(\d{1,2})\s(.*)bag")

# read the input into chunks
with open(sys.argv[1], 'r') as fd:
    input = [d for d in fd.read().split('\n') if d.rstrip() != '']

class Computer():
    def __init__(self, r0=0, pc=0):
        self.r0 = r0
        self.pc = pc
        self.ran_instructions = []

    def run_instructions(self, instructions):
        while True:
            if self.pc in self.ran_instructions:
                """
                print("trace")
                count = 0
                for i in reversed(self.ran_instructions) :
                    count += 1
                    if count > 15:
                        break
                    print(instructions[i])
                """
                print("want to run '%s' twice, r0 is %d" % (inst, self.r0))
                return -2
            if self.pc == len(instructions):
                print("reached the end %d" % self.r0)
                return -1
            inst = instructions[self.pc]
            inst_split = inst.split()
            inst_op = inst_split[0]
            inst_arg_op = inst_split[1][:1]
            inst_arg = inst_split[1][1:]

            #print("running (%s %s %s)" % (inst_op, inst_arg_op, inst_arg))

            if inst_op == "nop":
                self.ran_instructions.append(self.pc)
                self.pc += 1

            if inst_op == "acc":
                self.ran_instructions.append(self.pc)

                if inst_arg_op == "+":
                    self.r0 += int(inst_arg)
                else:
                    self.r0 -= int(inst_arg)
                self.pc += 1

            if inst_op == "jmp":
                self.ran_instructions.append(self.pc)

                if inst_arg_op == "+":
                    self.pc += int(inst_arg)
                else:
                    self.pc -= int(inst_arg)

def alter_inst_set(instructions, inst_to_alter):
    jmps = []
    for ii,inst in enumerate(instructions):
        if inst_to_alter in inst:
            jmps.append(ii)

    if inst_to_alter == "jmp":
        new_inst = "nop"
    else:
        new_inst = "jmp"
    inst_sets =[]
    for jmp in jmps:
        print("changing jmp at %d: %s" % (jmp, instructions[jmp]))
        cp = instructions.copy()
        cp[jmp] = cp[jmp].replace(inst_to_alter, new_inst)
        inst_sets.append(cp)
        print("jmp is now at %d: %s" % (jmp, cp[jmp]))
    return inst_sets

sets = alter_inst_set(input, "jmp")
for s in sets:
    computer = Computer()
    ret = computer.run_instructions(s)
    if ret == -1:
        break
"""
sets = alter_inst_set(input, "nop")
assert sets[0] != sets[1]
for s in sets:
    computer = Computer()
    ret = computer.run_instructions(s)
    if ret == -1:
        break
"""

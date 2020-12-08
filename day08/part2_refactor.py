import random
import re
import sys

TRACE = False
DEBUG = False

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
                if TRACE:
                    print("trace")
                    count = 0
                    for i in reversed(self.ran_instructions) :
                        count += 1
                        if count > 15:
                            break
                        print(instructions[i])
                if DEBUG:
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

# TODO: refactor into generator function
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
        if DEBUG:
            print("changing %s at %d: %s" % (inst_to_alter, jmp, instructions[jmp]))
        cp = instructions.copy()
        cp[jmp] = cp[jmp].replace(inst_to_alter, new_inst)
        inst_sets.append(cp)
    return inst_sets

done = False
for inst in ["jmp", "nop"]:
    if done:
        break
    sets = alter_inst_set(input, inst)
    for s in sets:
        computer = Computer()
        ret = computer.run_instructions(s)
        if ret == -1:
            done = True
            break

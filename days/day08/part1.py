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
            inst = instructions[self.pc]
            if self.pc in self.ran_instructions:
                print("trace")
                count = 0
                for i in reversed(self.ran_instructions) :
                    count += 1
                    if count > 15:
                        break
                    print(instructions[i])
                print("want to run '%s' twice, r0 is %d" % (inst, self.r0))
                return -2
            if self.pc == len(instructions):
                print("reached the end %d" % self.r0)
                return -1
            inst_split = inst.split()
            inst_op = inst_split[0]
            inst_arg_op = inst_split[1][:1]
            inst_arg = inst_split[1][1:]

            print("running (%s %s %s)" % (inst_op, inst_arg_op, inst_arg))

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

                old_pc = self.pc
                if inst_arg_op == "+":
                    self.pc += int(inst_arg)
                else:
                    self.pc -= int(inst_arg)

                #if self.pc in self.ran_instructions:
                    #self.pc = old_pc



computer = Computer()
computer.run_instructions(input)

import random

with open("input", 'r') as fd:
    input = [d.rstrip() for d in fd.read().split('\n')]

passport = []
passports = []
for line in input:
    if line:
        passport.append(line)
    else:
        passports.append(passport)
        passport = []
if len(passport) > 0:
    passports.append(passport)

needed = ["byr",
   "iyr", 
   "eyr", 
   "hgt", 
   "hcl", 
   "ecl", 
   "pid"]
   #"cid"]
class Passport():
    def __init__(self, fields):
        self.fields = fields

    def validate(self):
        for n in needed:
            if not n in self.fields:
                print(n, self.fields)
                return False
        return True

parsed = []
for passport in passports:
    fields = {}
    for line in passport:
        ls = line.split()
        for f in ls:
            ff = f.split(":")
            fields[ff[0]] = ff[1]
    parsed.append(Passport(fields))

correct = 0
for p in parsed:
    if p.validate():
        print("found a correct one")
        correct+=1

print(correct)

import random
import re
import sys

with open(sys.argv[1], 'r') as fd:
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
                return False


        failed_message = lambda f: print("failed in field %s: '%s'" % (f, self.fields[f]))
        byr = int(self.fields["byr"])
        if byr < 1920 or byr > 2002:
            failed_message("byr")
            return False
        iyr = int(self.fields["iyr"])
        if iyr < 2010 or iyr > 2020:
            failed_message("iyr")
            return False
        eyr = int(self.fields["eyr"])
        if eyr < 2020 or eyr > 2030:
            failed_message("eyr")
            return False
        
        hgt = self.fields["hgt"]
        if "cm" in hgt:
            val = int(hgt.replace("cm", ""))
            if val < 150 or val > 193:
                failed_message("hgt")
                return False
        if "in" in hgt:
            val = int(hgt.replace("in", ""))
            if val < 59 or val > 76:
                failed_message("hgt")
                return False
        if not "cm" in hgt and not "in" in hgt:
            failed_message("hgt")
            return False

        hcl = self.fields["hcl"]
        hcl_regex = re.compile(r"^#[0-9a-f]{6}$")
        if not hcl_regex.match(hcl):
            failed_message("hcl")
            return False

        ecl = self.fields["ecl"]
        valid = ['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth']
        if not ecl in valid:
            failed_message("ecl")
            return False

        pid = self.fields["pid"]
        pid_regex = re.compile(r"^\d{9}$")
        if not pid_regex.match(pid):
            failed_message("pid")
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
        correct+=1

print(correct)

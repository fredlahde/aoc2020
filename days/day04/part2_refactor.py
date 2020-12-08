import random
import re
import sys

HAIR_COLOR_REGEX = re.compile(r"^#[0-9a-f]{6}$")
PID_REGEX        = re.compile(r"^\d{9}$")
VALID_EYE_COLORS = ['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth']
PRINT_DEBUG      = False

class Passport(object):
    """ 
    Represents a passport object. 

    Example:
    hcl:#ae17e1 iyr:2013
    eyr:2024
    ecl:brn pid:760753108 byr:1931
    hgt:179cm
    """
    def __init__(self):
        self.fields = dict()

    def parse(self, textinput):
        """
        parses a block of multiple lines into a passport
        """
        for line in textinput.split('\n'):
            linesplit = line.split()
            for ls in linesplit:
                split = ls.split(":")
                self.fields[split[0]] = split[1]

    def try_get_field(self, k):
        """
        Tries to get a field from this passport. Returns None if the key is not 
        set in this passport
        """
        try:
            return self.fields[k]
        except KeyError:
            return None

    def __str__(self):
         return self.fields.__str__()

class Validation(object):
    """
    Represents a validation on a single field of a passport. 

    A field is considered valid if and only if:
    - It is set (unless it is not marked as required)
    - It passes the given validation criteria
    """

    def __init__(self, field_name, validation_func, required=True):
        self.field_name = field_name
        self.validation_func = validation_func
        self.required = required

    def validate(self, passport):
        """
        Validate the given passport
        """
        # if the field is not even required, we do not need to validate it
        if not self.required:
            return True
        # try to get the value
        value = passport.try_get_field(self.field_name)

        # if we can't find the value, return False
        if value is None:
            return False

        # run the given validation function
        assert (self.validation_func is not None),\
               "no validation func given but a required value: '%s'" % self.field_name
        return self.validation_func(value)

class PassportValidator(object):
    """
    validates a given passport against a set of rules
    """
    def __init__(self, validations):
        self.validations = validations

    def validate(self, passport):
        """
        validate a given passport
        """

        # get a dict {field_name: passed} for each validation
        results = dict([(v.field_name, v.validate(passport)) for v in self.validations])

        # figure out if all validations have passed
        passed =  all(results[k] for k in results)

        # if not, print debug output
        if not passed and PRINT_DEBUG:
            print("failed for")
            print(passport)
            for name, passed in results.items():
                print(name, "'%s'" % passport.try_get_field(name), passed)
            print()

        # return result
        return passed

def validate_height(hgt):
    """
    validates the height value of a passport:

    hgt (Height) - a number followed by either cm or in:
        If cm, the number must be at least 150 and at most 193.
        If in, the number must be at least 59 and at most 76.
    """
    unit = hgt[-2:]
    val = hgt[:-2]
    if unit == "cm":
        return 150 <= int(val) <= 193
    if unit == "in":
        return 59 <= int(val) <= 76
    return False

"""
Define validations:

    byr (Birth Year) - four digits; at least 1920 and at most 2002.
    iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    hgt (Height) - a number followed by either cm or in:
        If cm, the number must be at least 150 and at most 193.
        If in, the number must be at least 59 and at most 76.
    hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    pid (Passport ID) - a nine-digit number, including leading zeroes.
    cid (Country ID) - ignored, missing or not.
"""
validations = [
        Validation("cid", None, required=False),
        Validation("byr", lambda x: 1920 <= int(x) <= 2002),
        Validation("iyr", lambda x: 2010 <= int(x) <= 2020),
        Validation("eyr", lambda x: 2020 <= int(x) <= 2030),
        Validation("hgt", validate_height),
        Validation("hcl", lambda x: HAIR_COLOR_REGEX.match(x)),
        Validation("ecl", lambda x: x in VALID_EYE_COLORS),
        Validation("pid", lambda x: PID_REGEX.match(x))
]

# create passport validator
validator = PassportValidator(validations)

# read the input into chunks
with open(sys.argv[1], 'r') as fd:
    input = [d for d in fd.read().split('\n\n')]

# parse each chunck into a passport and validate it 
passed_sum = 0
for chunk in input:
    passport = Passport()
    passport.parse(chunk)
    passed = validator.validate(passport)
    if passed:
        passed_sum += 1

print(passed_sum)

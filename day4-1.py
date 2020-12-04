import sys

def parse_key_value_pair(x):
    return x.split(':', maxsplit=1)

expected_fields = {'byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid'}

n_valid = 0
passport = {}
for line in sys.stdin.readlines() + ['']:
    line = line.strip()
    if line:
        passport.update(dict(parse_key_value_pair(x) for x in line.split()))
    else:
        n_valid += expected_fields.issubset(passport.keys())
        passport = {}

print(n_valid)

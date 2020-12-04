import re
import sys

def parse_key_value_pair(x):
    return x.split(':', maxsplit=1)

def validate_height(hgt):
    m = re.match(r'^(\d+)(in|cm)$', hgt)
    if m and m.group(2) == 'cm':
        return 150 <= int(m.group(1)) <= 193
    elif m and m.group(2) == 'in':
        return 59 <= int(m.group(1)) <= 76
    else:
        return False

validators = {
    'byr': lambda v: len(v) == 4 and 1920 <= int(v) <= 2002,
    'iyr': lambda v: len(v) == 4 and 2010 <= int(v) <= 2020,
    'eyr': lambda v: len(v) == 4 and 2020 <= int(v) <= 2030,
    'hgt': validate_height,
    'hcl': lambda v: re.match(r'^#[0-9a-f]{6}$', v),
    'ecl': lambda v: v in {'amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth'},
    'pid': lambda v: re.match(r'^\d{9}$', v)
}

n_valid = 0
passport = {}
for line in sys.stdin.readlines() + ['']:
    line = line.strip()
    if line:
        passport.update(dict(parse_key_value_pair(x) for x in line.split()))
    else:
        is_valid = all(
            key in passport and validate(passport[key])
            for key, validate in validators.items()
        )
        n_valid += is_valid
        passport = {}

print(n_valid)

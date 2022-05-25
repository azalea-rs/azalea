import re


def to_snake_case(name: str):
    s = re.sub('([A-Z])', r'_\1', name)
    return s.lower().strip('_')


def to_camel_case(name: str):
    s = re.sub('_([a-z])', lambda m: m.group(1).upper(), name)
    return s[0].upper() + s[1:]


def padded_hex(n: int):
    return f'0x{n:02x}'

import urllib.request
import gzip
import json
import re
import io


def to_snake_case(name):
    s = re.sub('([A-Z])', r'_\1', name)
    return s.lower()


def to_camel_case(name):
    s = re.sub('_([a-z])', lambda m: m.group(1).upper(), name)
    return s[0].upper() + s[1:]

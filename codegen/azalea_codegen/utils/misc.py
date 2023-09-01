import os
import re
import subprocess
import traceback
from typing import TypeVar, Callable

T = TypeVar('T')


def run_command(command: list[str], **kwargs):
    process = subprocess.run(
        command,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        **kwargs
    )

    if process.returncode != 0:
        # Quote command arguments with spaces in them
        command_nice = ' '.join(f'"{part}"' if ' ' in part else part for part in command)
        error_message = f'`{command_nice}` failed with exit code {process.returncode}'

        # Add output to the error message if available.
        output = process.stdout.decode('utf-8').strip()

        if len(output):
            error_message += '\n\nOutput:\n'

            for line in output.splitlines():
                error_message += f'    {line}\n'

        raise Exception(error_message)


def upper_first_letter(name: str):
    return name[0].upper() + name[1:]


def to_camel_case(name: str):
    s = re.sub('[_ ](\\w)', lambda m: m.group(1).upper(), name.replace('.', '_').replace('/', '_'))
    s = upper_first_letter(s)

    # if the first character is a number, we need to add an underscore
    # maybe we could convert it to the number name (like 2 would become "two")?
    if s[0].isdigit():
        s = f'_{s}'

    return s


def to_snake_case(name: str):
    s = re.sub('([A-Z])', r'_\1', name)
    return s.lower().strip('_')


def strip_suffix(s: str, suffix: str) -> str:
    if s.endswith(suffix):
        return s[:-len(suffix)]

    return s


def strip_prefix(s: str, prefix: str) -> str:
    if s.startswith(prefix):
        return s[len(prefix):]

    return s


def exception_to_string(exc: Exception) -> str:
    return ''.join(traceback.format_exception(type(exc), exc, exc.__traceback__))


def get_root_path(*path: str) -> str:
    return os.path.join(os.path.dirname(os.path.dirname(__file__)), '..', '..', *path)


def padded_hex(n: int):
    return f'0x{n:02x}'


def find(lst: list[T], f: Callable[[T], bool]) -> T | None:
    for item in lst:
        if f(item):
            return item

    return None

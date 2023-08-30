from azalea_codegen.utils import strip_suffix


class Mappings:
    """Container and parser for Mojang-format mappings."""
    __slots__ = ('class_names', 'field_names', 'method_names', 'field_types', 'method_return_types')

    def __init__(self, class_names, field_names, method_names, field_types, method_return_types):
        self.class_names = class_names
        self.field_names = field_names
        self.field_types = field_types
        self.method_names = method_names
        self.method_return_types = method_return_types

    @staticmethod
    def parse(mappings_txt: str):
        class_names = {}
        field_names = {}
        method_names = {}
        field_types = {}
        method_return_types = {}

        current_obfuscated_class_name = None

        for line in mappings_txt.splitlines():
            # Skip comments and blank lines.
            if line.startswith('#') or line.strip() == '':
                continue

            # Lines starting with 4 spaces are methods or fields.
            if line.startswith('    '):
                before, obf_name = line.split(' -> ', 1)

                if before.endswith(')'):
                    # Lines ending with a ) are methods
                    signature, parameters = strip_suffix(before, ')').split('(', 1)

                    # Strip anything before the last : (not all methods have this)
                    signature = signature.rsplit(':', 1)[-1]
                    return_type, method_name = signature.split(' ', 1)

                    if current_obfuscated_class_name not in method_names:
                        method_names[current_obfuscated_class_name] = {}
                        method_return_types[current_obfuscated_class_name] = {}

                    method_names[current_obfuscated_class_name][f'{obf_name}({parameters})'] = method_name
                    method_return_types[current_obfuscated_class_name][f'{obf_name}({parameters})'] = return_type

                else:
                    # Otherwise they're fields.
                    field_type, field_name = before.lstrip().split(' ', 1)

                    if current_obfuscated_class_name not in field_names:
                        field_names[current_obfuscated_class_name] = {}
                        field_types[current_obfuscated_class_name] = {}

                    field_names[current_obfuscated_class_name][obf_name] = field_name
                    field_types[current_obfuscated_class_name][obf_name] = field_type

            else:
                # Otherwise they're a class definition.
                name, obf_name = line.split(' -> ', 1)
                obf_name = strip_suffix(obf_name, ':')

                current_obfuscated_class_name = obf_name
                class_names[obf_name] = name

        return Mappings(class_names, field_names, method_names, field_types, method_return_types)

    def get_field_name(self, obf_class_name: str, obf_field_name: str) -> str | None:
        """Gets the mapped name of a field."""
        return self.field_names.get(obf_class_name, {}).get(obf_field_name)

    def get_class_name(self, obf_class_name: str) -> str:
        """Gets the mapped name of a class."""
        if '<' in obf_class_name:
            first_part, args = obf_class_name.split('<')
            args = args.rstrip('>').strip(';').split(';')
            print(args)
            assert len(args) == 1
            arg = self.get_class_name(args[0][1:])
            return f'{first_part}<{arg}>'

        return self.class_names[obf_class_name]

    def get_method_name(self, obfuscated_class_name: str, obfuscated_method_name: str, obfuscated_signature: str) -> str | None:
        """Gets the mapped name of a method."""
        return self.method_names.get(obfuscated_class_name, {}).get(f'{obfuscated_method_name}({obfuscated_signature})')

    def get_field_type(self, obfuscated_class_name: str, obfuscated_field_name: str) -> str | None:
        """Gets the mapped type of a field."""
        return self.field_types.get(obfuscated_class_name, {}).get(obfuscated_field_name)

    def get_method_return_type(self, obfuscated_class_name: str, obfuscated_method_name: str, obfuscated_signature: str) -> str | None:
        """Gets the mapped return type of a method."""
        return self.method_return_types.get(obfuscated_class_name, {}).get(f'{obfuscated_method_name}({obfuscated_signature})')

    def get_obfuscated_class_name(self, deobfuscated_name: str) -> str | None:
        """Gets the obfuscated name of a class."""
        for obfuscated_name, real_name in self.class_names.items():
            if real_name == deobfuscated_name:
                return obfuscated_name

        return None

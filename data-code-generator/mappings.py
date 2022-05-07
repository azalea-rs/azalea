class Mappings:
    __slots__ = ('classes', 'fields', 'methods')

    def __init__(self, classes, fields, methods):
        self.classes = classes
        self.fields = fields
        self.methods = methods

    @staticmethod
    def parse(mappings_txt):
        classes = {}
        fields = {}
        methods = {}

        current_obfuscated_class_name = None

        for line in mappings_txt.splitlines():
            if line.startswith('#') or line == '':
                continue

            if line.startswith('    '):
                # if a line starts with 4 spaces, that means it's a method or a field
                if '(' in line:
                    # if it has an opening parenthesis, it's a method
                    real_name_with_parameters_and_line, obfuscated_name = line.strip().split(' -> ')
                    real_name_with_parameters = real_name_with_parameters_and_line.split(
                        ':')[-1]

                    real_name = real_name_with_parameters.split('(')[0]
                    parameters = real_name_with_parameters.split('(')[1]

                    if current_obfuscated_class_name not in methods:
                        methods[current_obfuscated_class_name] = {}
                    methods[current_obfuscated_class_name][
                        f'{obfuscated_name}({parameters})'] = real_name
                else:
                    # otherwise, it's a field
                    real_name_with_type, obfuscated_name = line.strip().split(' -> ')
                    real_name = real_name_with_type.split(' ')[1]

                    if current_obfuscated_class_name not in fields:
                        fields[current_obfuscated_class_name] = {}
                    fields[current_obfuscated_class_name][obfuscated_name] = real_name
            else:
                # otherwise it's a class
                real_name, obfuscated_name = line.strip(':').split(' -> ')
                current_obfuscated_class_name = obfuscated_name

                classes[obfuscated_name] = real_name

        return Mappings(classes, fields, methods)

    def get_field(self, obfuscated_class_name, obfuscated_field_name):
        return self.fields.get(obfuscated_class_name, {}).get(obfuscated_field_name)

    def get_class(self, obfuscated_class_name):
        return self.classes[obfuscated_class_name]

    def get_method(self, obfuscated_class_name, obfuscated_method_name, obfuscated_signature):
        return self.methods[obfuscated_class_name][f'{obfuscated_method_name}({obfuscated_signature})']

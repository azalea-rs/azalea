from jawa.cf import ClassFile
from jawa.classloader import ClassLoader
from jawa.constants import MethodReference, FieldReference
from jawa.fields import Field
from jawa.methods import Method


def field_reference_eq(a: FieldReference, b: FieldReference) -> bool:
    return a.class_.name.value == b.class_.name.value and \
        a.name_and_type.name.value == b.name_and_type.name.value and \
        a.name_and_type.descriptor.value == b.name_and_type.descriptor.value


def find_called_method(classloader: ClassLoader, target_method: MethodReference) -> (ClassFile | None, Method | None):
    clazz_name = target_method.class_.name.value

    while True:
        if clazz_name is None:
            return None, None

        clazz = classloader[clazz_name]

        if clazz is None:
            return None, None

        method = clazz.methods.find_one(
            name=target_method.name_and_type.name.value,
            f=lambda f: f.descriptor.value == target_method.name_and_type.descriptor.value,
        )

        if method is not None:
            return clazz, method

        clazz_name = clazz.super_.name.value


def find_accessed_field(classloader: ClassLoader, target_field: FieldReference) -> (ClassFile | None, Field | None):
    clazz = classloader[target_field.class_.name.value]

    if clazz is None:
        return None

    field = clazz.fields.find_one(
        name=target_field.name_and_type.name.value,
        type_=target_field.name_and_type.descriptor.value,
    )

    return clazz, field

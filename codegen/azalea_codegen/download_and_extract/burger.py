"""Runs Burger for a given client JAR."""
import importlib
import itertools
import json
import logging
import os
import pkgutil
from typing import Type
from zipfile import ZipFile

from burger.roundedfloats import transform_floats
from burger.toppings.topping import Topping
from jawa.classloader import ClassLoader
from jawa.transforms import simple_swap, expand_constants

from azalea_codegen.download_and_extract import versioned_cache
from azalea_codegen.download_and_extract.launcher_meta import get_client_jar

_LOGGER = logging.getLogger(__name__)
_CACHE = {}


def _search_toppings(package: str, exclusions: set[str]) -> dict[str, Type[Topping]]:
    package_module = importlib.import_module(package, package)

    toppings_names = [name for _, name, _ in pkgutil.iter_modules([
        os.path.dirname(package_module.__file__)
    ])]

    toppings = {}

    for name in toppings_names:
        if name in exclusions:
            continue

        module = importlib.import_module(f'{package}.{name}', package)
        found_toppings = set()

        for key in module.__dict__:
            item = getattr(module, key)

            if not isinstance(item, type):
                continue

            if issubclass(item, Topping) and item != Topping:
                found_toppings.add(item)

        if len(found_toppings) == 0:
            _LOGGER.warning(f'Topping {package}.{name} does not contain any toppings, skipping')

        elif len(found_toppings) >= 2:
            _LOGGER.warning(f'Topping {package}.{name} contains more than 1 topping, skipping')

        else:
            toppings[name] = found_toppings.pop()

    return toppings


class _DependencyNode:
    def __init__(self, topping):
        self.topping = topping
        self.provides = topping.PROVIDES
        self.depends = topping.DEPENDS
        self.childs = []

    def __repr__(self):
        return str(self.topping)


def _run_burger(client_jar: ZipFile, verbose_toppings: bool = False) -> dict:
    # Load all toppings
    all_toppings = _search_toppings('burger.toppings', {'topping'})
    all_toppings.update(_search_toppings('azalea_codegen.download_and_extract.toppings', set()))
    all_toppings = all_toppings.values()

    # Order topping execution by building dependency tree
    topping_nodes = []
    topping_provides = {}

    for topping in all_toppings:
        topping_node = _DependencyNode(topping)
        topping_nodes.append(topping_node)
        for provides in topping_node.provides:
            topping_provides[provides] = topping_node

    # Find dependency childs
    for topping in topping_nodes:
        for dependency in topping.depends:
            if dependency not in topping_provides:
                raise Exception(f'Topping {topping} requires {dependency} but that dependency was not provided')

            if not topping_provides[dependency] in topping.childs:
                topping.childs.append(topping_provides[dependency])

    # Run leaves first
    to_be_run = []
    while len(topping_nodes) > 0:
        stuck = True
        for topping in topping_nodes:
            if len(topping.childs) == 0:
                stuck = False
                for parent in topping_nodes:
                    if topping in parent.childs:
                        parent.childs.remove(topping)
                to_be_run.append(topping.topping)
                topping_nodes.remove(topping)

        if stuck:
            raise Exception(f'Can\'t resolve topping dependencies due to a cycle')

    classloader = ClassLoader(max_cache=0, bytecode_transforms=[simple_swap, expand_constants])
    classloader.path_map.update(zip(client_jar.namelist(), itertools.repeat(client_jar)))

    available = []
    aggregate = {}

    for topping in to_be_run:
        missing = [dep for dep in topping.DEPENDS if dep not in available]

        if len(missing) != 0:
            _LOGGER.warning(f'Missing dependencies for {topping}: {", ".join(missing)}')
            continue

        orig_aggregate = aggregate.copy()
        try:
            topping.act(aggregate, classloader, verbose_toppings)
            available.extend(topping.PROVIDES)

        except:
            aggregate = orig_aggregate  # If the topping failed, don't leave things in an incomplete state
            _LOGGER.error(f'Failed to run {topping}', exc_info=True)

    return transform_floats(aggregate)


def get_burger_data(version_id: str) -> dict:
    def fetch_burger() -> bytes:
        client_jar = get_client_jar(version_id)
        burger_result = _run_burger(client_jar, verbose_toppings=False)
        return json.dumps(burger_result).encode('utf-8')

    def decode_burger(buf: bytes) -> dict:
        return json.loads(buf.decode('utf-8'))

    return versioned_cache(
        version_id,
        'burger.json',
        _CACHE,
        fetch_burger,
        decode_burger
    )

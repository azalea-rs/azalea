import logging
import os.path
import typing
from typing import Callable

_LOGGER = logging.getLogger(__name__)
CACHE_PATH = './__codegen_cache__'


def get_version_cache_path(version_id: str) -> str:
    path = os.path.join(CACHE_PATH, 'versions', version_id)
    os.makedirs(path, exist_ok=True)

    return path


_T = typing.TypeVar('_T')


def versioned_cache(version_id: str, cache_filename: str, cache: dict[str, _T], fetch: Callable[[], bytes],
                    decode: Callable[[bytes], _T]) -> _T:
    # Check for a copy in cache first.
    if version_id not in cache:
        # Then check for a copy on disk.
        cache_path = os.path.join(get_version_cache_path(version_id), cache_filename)

        if os.path.exists(cache_path):
            with open(cache_path, 'rb') as f:
                buf = f.read()
                _LOGGER.info(f'Using disk-cached version of {cache_filename} for {version_id} ({len(buf)} bytes)')

                cache[version_id] = decode(buf)

        else:
            # And finally try and fetch from the network.
            buf = fetch()

            # Write to disk.
            with open(cache_path, 'wb') as f:
                f.write(buf)

            cache[version_id] = decode(buf)

    return cache[version_id]

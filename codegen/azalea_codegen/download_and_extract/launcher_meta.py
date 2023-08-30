"""Extracts information from Mojang's launcher meta"""
import hashlib
import io
import logging
import zipfile

import requests

from azalea_codegen.download_and_extract import versioned_cache
from azalea_codegen.utils import Mappings

_LOGGER = logging.getLogger(__name__)

_VERSION_MANIFEST = None
_CLIENT_JSONS = {}
_CLIENT_JARS = {}
_CLIENT_MAPPINGS = {}
_SERVER_JARS = {}
_SERVER_MAPPINGS = {}


def get_version_manifest():
    """Returns the latest version manifest JSON, fetching it if needed."""
    global _VERSION_MANIFEST

    if _VERSION_MANIFEST is None:
        req = requests.get('https://launchermeta.mojang.com/mc/game/version_manifest.json')
        req.raise_for_status()

        _VERSION_MANIFEST = req.json()

    return _VERSION_MANIFEST


def fetch_version_manifest_info(version_id):
    for version in get_version_manifest()['versions']:
        if version['id'] == version_id:
            return version

    raise Exception(f'Couldn\'t find version {version_id} in version_manifest.json')


def get_client_json(version_id: str):
    """Gets the client.json file for a specific version ID (e.g. 23w35a or 1.20.1)"""
    # Check if we have a cached copy in memory.
    if version_id not in _CLIENT_JSONS:
        # If not, fetch one.
        info = fetch_version_manifest_info(version_id)
        req = requests.get(info['url'])
        req.raise_for_status()

        _CLIENT_JSONS[version_id] = req.json()

    return _CLIENT_JSONS[version_id]


def _fetch_from_mojang(version_id: str, download_name: str) -> bytes:
    # Download from Mojang.
    info = get_client_json(version_id)

    if download_name not in info['downloads']:
        raise Exception(
            f'Tried to download {download_name} from {version_id} but that download is not available')

    download_info = info['downloads'][download_name]

    _LOGGER.info(f'Downloading {download_name} from {version_id} ({download_info["url"]} with SHA1 '
                 f'{download_info["sha1"]} - {download_info["size"]} bytes)')

    req = requests.get(download_info['url'])
    req.raise_for_status()

    buf = req.content

    # Check response size.
    if len(buf) != download_info['size']:
        raise Exception(f'Downloaded {download_name} for {version_id} but size differs from expected (expected '
                        f'{download_info["size"]}, got {len(buf)})')

    # Check response hash.
    digest = hashlib.new('sha1', buf).hexdigest()

    if digest != download_info['sha1']:
        raise Exception(f'Downloaded {download_name} for {version_id} but hash differs from expected (expected '
                        f'{download_info["sha1"]}, got {digest}')

    return buf


def get_client_jar(version_id: str) -> zipfile.ZipFile:
    """Fetches the client JAR for a specific version ID"""
    return versioned_cache(
        version_id,
        'client.jar',
        _CLIENT_JARS,
        lambda: _fetch_from_mojang(version_id, 'client'),
        lambda b: zipfile.ZipFile(io.BytesIO(b))
    )


def get_client_mappings(version_id: str) -> Mappings:
    """Fetches the client mappings for a specific version ID"""
    return versioned_cache(
        version_id,
        'client.txt',
        _CLIENT_MAPPINGS,
        lambda: _fetch_from_mojang(version_id, 'client_mappings'),
        lambda b: Mappings.parse(b.decode('utf-8'))
    )


def get_server_jar(version_id: str) -> zipfile.ZipFile:
    """Fetches the server JAR for a specific version ID"""
    return versioned_cache(
        version_id,
        'server.jar',
        _SERVER_JARS,
        lambda: _fetch_from_mojang(version_id, 'server'),
        lambda b: zipfile.ZipFile(io.BytesIO(b))
    )


def get_server_mappings(version_id: str) -> Mappings:
    """Fetches the server mappings for a specific version ID"""
    return versioned_cache(
        version_id,
        'server.txt',
        _SERVER_MAPPINGS,
        lambda: _fetch_from_mojang(version_id, 'server_mappings'),
        lambda b: Mappings.parse(b.decode('utf-8'))
    )

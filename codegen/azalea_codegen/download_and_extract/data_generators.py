"""Runs Minecraft data generators with Java"""
import functools
import json
import os.path
import shutil

from azalea_codegen.download_and_extract import get_version_cache_path
from azalea_codegen.download_and_extract.launcher_meta import get_server_jar
from azalea_codegen.utils import run_command


@functools.cache
def run_data_generators(version_id: str) -> str:
    cache_path = get_version_cache_path(version_id)
    output_path = os.path.abspath(os.path.join(cache_path, 'data_generators'))

    if os.path.exists(output_path):
        return output_path

    # Fetch the server JAR.
    get_server_jar(version_id)
    jar_path = os.path.abspath(os.path.join(cache_path, 'server.jar'))

    # Run in a temporary directory for libraries and such.
    run_path = os.path.join(cache_path, 'server_run')
    os.makedirs(run_path)

    try:
        run_command(
            ['java', '-DbundlerMainClass=net.minecraft.data.Main', '-jar', jar_path, '--all', '--output', output_path],
            cwd=run_path
        )

    finally:
        shutil.rmtree(run_path)

    return output_path


@functools.cache
def get_registry_report(version_id: str) -> dict:
    path = os.path.join(run_data_generators(version_id), 'reports', 'registries.json')

    with open(path, 'r') as f:
        return json.load(f)

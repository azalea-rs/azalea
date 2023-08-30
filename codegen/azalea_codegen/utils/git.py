import logging
import os

from azalea_codegen import utils

_LOGGER = logging.getLogger(__name__)
GIT_BINARY = os.environ.get('AZALEA_CODEGEN_GIT_BINARY', 'git')


def clone(repo_url: str, path: str):
    _LOGGER.info(f'Cloning {repo_url} into {path}')
    utils.run_command([GIT_BINARY, 'clone', repo_url, path])


def update(path: str):
    _LOGGER.info(f'Updating repository at {path}')
    utils.run_command([GIT_BINARY, 'pull', '--ff', '--force'], cwd=path)


def clone_or_update(repo_url: str, path: str):
    if os.path.exists(path):
        update(path)

    else:
        clone(repo_url, path)

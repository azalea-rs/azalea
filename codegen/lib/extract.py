# Extracting data from the Minecraft jars

from lib.download import get_server_jar, get_burger, get_client_jar, get_pixlyzer, get_yarn_data, get_fabric_api_versions, get_fabric_loader_versions
from lib.utils import get_dir_location
from zipfile import ZipFile
import subprocess
import json
import sys
import re
import os


def generate_data_from_server_jar(version_id: str):
    if os.path.exists(get_dir_location(f'downloads/generated-{version_id}')):
        return

    get_server_jar(version_id)
    os.system(
        f'cd {get_dir_location(f"downloads")} && java -DbundlerMainClass=net.minecraft.data.Main -jar {get_dir_location(f"downloads/server-{version_id}.jar")} --all --output \"{get_dir_location(f"downloads/generated-{version_id}")}\"'
    )


def get_block_states_report(version_id: str):
    generate_data_from_server_jar(version_id)
    with open(get_dir_location(f'downloads/generated-{version_id}/reports/blocks.json'), 'r') as f:
        return json.load(f)


def get_registries_report(version_id: str):
    generate_data_from_server_jar(version_id)
    with open(get_dir_location(f'downloads/generated-{version_id}/reports/registries.json'), 'r') as f:
        return json.load(f)


def get_block_states_burger(version_id: str):
    burger_data = get_burger_data_for_version(version_id)
    return burger_data[0]['blocks']['block']


def get_ordered_blocks_burger(version_id: str):
    burger_data = get_burger_data_for_version(version_id)
    return burger_data[0]['blocks']['ordered_blocks']


python_command = None


def determine_python_command():
    global python_command
    if python_command:
        return python_command

    def try_python_command(version):
        return os.system(f'{version} --version') == 0

    for version in ('python3.9', 'python3.8', 'python3', 'python'):
        if try_python_command(version):
            python_command = version
            return version
    raise Exception(
        'Couldn\'t determine python command to use to run burger with!')


def run_python_command_and_download_deps(command):
    print('>', command)
    for _ in range(10):
        p = subprocess.Popen(
            [command],
            stderr=subprocess.PIPE,
            shell=True
        )

        stderr = b''
        while True:
            data = p.stderr.read()
            if data == b'':
                break
            print(data.decode(), end='', flush=True)
            stderr += data

        regex_match = re.search(
            r'ModuleNotFoundError: No module named \'(\w+?)\'', stderr.decode())
        if not regex_match:
            break
        missing_lib = regex_match.group(1)
        print('Missing required lib:', missing_lib)
        os.system(
            f'{determine_python_command()} -m pip install {missing_lib}')
    print('ok')


def get_burger_data_for_version(version_id: str):
    if not os.path.exists(get_dir_location(f'downloads/burger-{version_id}.json')):
        get_burger()
        get_client_jar(version_id)

        run_python_command_and_download_deps(
            f'cd {get_dir_location("downloads/Burger")} && {determine_python_command()} munch.py ../client-{version_id}.jar --output ../burger-{version_id}.json'
        )
    with open(get_dir_location(f'downloads/burger-{version_id}.json'), 'r') as f:
        return json.load(f)


def get_pixlyzer_data(version_id: str, category: str):
    '''
    Gets data from Pixlyzer. Note that this requires Yarn to release updates first.
    '''

    target_dir = get_dir_location(f'downloads/pixlyzer-{version_id}')

    if not os.path.exists(get_dir_location(target_dir)):
        pixlyzer_dir = get_pixlyzer()

        yarn_data = get_yarn_data(version_id)
        if not yarn_data:
            raise Exception(
                'Fabric/Yarn hasn\'t been updated to this version yet.')

        # for some reason pixlyzer doesn't work right unless the mvn clean
        # instruction looks like that
        # and pixlyzer.py doesn't do it right

        # map jar + download dependencies
        run_python_command_and_download_deps(
            f'cd {pixlyzer_dir}/wrapper && {determine_python_command()} PixLyzer.py --only-version={version_id} --dont-compile --only-map'
        )
        # compile
        os.system(
            f'cd {pixlyzer_dir} && mvn clean -Dmaven.repo.local=. verify')
        # run pixlyzer.py again lol
        run_python_command_and_download_deps(
            f'cd {pixlyzer_dir}/wrapper && {determine_python_command()} PixLyzer.py --only-version={version_id} --no-compile'
        )

        source_dir = get_dir_location(
            f'{pixlyzer_dir}/wrapper/data/version/{version_id}')

        if not os.path.exists(source_dir):
            print('PixLyzer failed, no output!')
            exit()
        if os.path.exists(target_dir):
            os.unlink(target_dir)
        os.rename(
            source_dir,
            target_dir
        )

        with open(f'{target_dir}/{category}.min.json', 'r') as f:
            return json.load(f)



def get_file_from_jar(version_id: str, file_dir: str):
    get_client_jar(version_id)
    with ZipFile(get_dir_location(f'downloads/client-{version_id}.jar')) as z:
        with z.open(file_dir) as f:
            return f.read()


def get_en_us_lang(version_id: str):
    return json.loads(
        get_file_from_jar(version_id, 'assets/minecraft/lang/en_us.json')
    )

# Extracting data from the Minecraft jars

from typing import TYPE_CHECKING
from lib.download import get_mappings_for_version, get_pumpkin_extractor, get_server_jar, get_burger, get_client_jar
from lib.utils import get_dir_location, to_camel_case, upper_first_letter
from zipfile import ZipFile
import subprocess
import requests
import json
import sys
import re
import os


def generate_data_from_server_jar(version_id: str):
    if os.path.exists(get_dir_location(f'__cache__/generated-{version_id}')):
        return

    get_server_jar(version_id)
    os.system(
        f'cd {get_dir_location(f"__cache__")} && java -DbundlerMainClass=net.minecraft.data.Main -jar {get_dir_location(f"__cache__/server-{version_id}.jar")} --all --output \"{get_dir_location(f"__cache__/generated-{version_id}")}\"'
    )


def get_block_states_report(version_id: str):
    return get_report(version_id, 'blocks')
def get_registries_report(version_id: str):
    return get_report(version_id, 'registries')
def get_packets_report(version_id: str):
    return get_report(version_id, 'packets')
def get_report(version_id: str, name: str):
    generate_data_from_server_jar(version_id)
    with open(get_dir_location(f'__cache__/generated-{version_id}/reports/{name}.json'), 'r') as f:
        return json.load(f)

def get_registry_tags(version_id: str, name: str):
    generate_data_from_server_jar(version_id)
    tags_directory = get_dir_location(f'__cache__/generated-{version_id}/data/minecraft/tags/{name}')
    if not os.path.exists(tags_directory):
        return {}
    tags = {}
    for root, dirs, files in os.walk(tags_directory, topdown=False):
        for name in files:
            file = os.path.join(root, name)
            relative_path = file.replace(tags_directory, '')[1:]
            if not file.endswith('.json'):
                continue
            with open(file, 'r') as f:
                tags[relative_path[:-5]] = json.load(f)
    return tags

python_command = None


def determine_python_command():
    return 'venv/bin/python'
    


def run_python_command_and_download_deps(command):
    print('>', command)
    for _ in range(10):
        p = subprocess.Popen(
            command,
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
            out, err = p.communicate()
            if out:
                print(out)
            if err:
                print(err)
            break
        missing_lib = regex_match.group(1)
        print('Missing required lib:', missing_lib)
        subprocess.run(f'venv/bin/pip install {missing_lib}', cwd=os.path.dirname(os.path.dirname(__file__)))
    print('ok')


def get_burger_data_for_version(version_id: str):
    if not os.path.exists(get_dir_location(f'__cache__/burger-{version_id}.json')):
        get_burger()
        get_client_jar(version_id)
        get_mappings_for_version(version_id)

        print('\033[92mRunning Burger...\033[m')
        run_python_command_and_download_deps(
            f'cd {get_dir_location("__cache__/Burger")} && '\
            f'venv/bin/python munch.py {get_dir_location("__cache__")}/client-{version_id}.jar '\
            f'--output {get_dir_location("__cache__")}/burger-{version_id}.json '\
            f'--mappings {get_dir_location("__cache__")}/mappings-{version_id}.txt'
        )
    with open(get_dir_location(f'__cache__/burger-{version_id}.json'), 'r') as f:
        return json.load(f)


def get_pumpkin_data(version_id: str, category: str):
    assert '/' not in version_id
    assert '\\' not in version_id
    target_parent_dir = get_dir_location(f'__cache__/pumpkin-{version_id}')
    category_dir = f'{target_parent_dir}/{category}.json'

    if os.path.exists(category_dir):
        with open(category_dir, 'r') as f:
            return json.load(f)

    pumpkin_dir = get_pumpkin_extractor()
    os.makedirs(f'{pumpkin_dir}/run', exist_ok=True)
    with open(f'{pumpkin_dir}/run/eula.txt', 'w') as f:
        f.write('eula=true')

    # run ./gradlew runServer until it logs "(pumpkin_extractor) Done"
    p = subprocess.Popen(
        f'cd {pumpkin_dir} && ./gradlew runServer',
        stderr=subprocess.PIPE,
        stdout=subprocess.PIPE,
        shell=True
    )

    while True:
        data = p.stdout.readline().decode()
        print('>' + data, end='', flush=True)
        if '[Server thread/INFO] (pumpkin_extractor) Done' in data:
            print('Pumpkin extractor done')
            break
        if data == b'':
            break

    p.terminate()

    # move the run/pumpkin_extractor_output directory to target_parent_dir
    # delete target_parent_dir if it's empty
    if os.path.exists(target_parent_dir):
        os.rmdir(target_parent_dir)
    os.rename(f'{pumpkin_dir}/run/pumpkin_extractor_output', target_parent_dir)

    with open(category_dir, 'r') as f:
        return json.load(f)


def get_file_from_jar(version_id: str, file_dir: str):
    get_client_jar(version_id)
    with ZipFile(get_dir_location(f'__cache__/client-{version_id}.jar')) as z:
        with z.open(file_dir) as f:
            return f.read()


def get_en_us_lang(version_id: str):
    return json.loads(
        get_file_from_jar(version_id, 'assets/minecraft/lang/en_us.json')
    )

# burger packet id extraction is broken since 1.20.5 (always returns -1, so we have to determine packet id ourselves from the mappings).
# this is very much not ideal.

if TYPE_CHECKING: from codegen.lib.mappings import Mappings
def get_packet_list(version_id: str):
    if version_id != '1.21':
        return []

    generate_data_from_server_jar(version_id)
    with open(get_dir_location(f'__cache__/generated-{version_id}/reports/packets.json'), 'r') as f:
        packets_report = json.load(f)
    packet_list = []
    for state, state_value in packets_report.items():
        for direction, direction_value in state_value.items():
            for packet_resourcelocation, packet_value in direction_value.items():
                assert packet_resourcelocation.startswith('minecraft:')
                packet_resourcelocation = upper_first_letter(to_camel_case(packet_resourcelocation[len('minecraft:'):]))
                packet_list.append({
                    'state': state,
                    'direction': direction,
                    'name': packet_resourcelocation,
                    'id': packet_value['protocol_id']
                })

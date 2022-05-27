from lib.utils import get_dir_location
from .mappings import Mappings
import requests
import json
import os

# make sure the downloads directory exists
if not os.path.exists(get_dir_location('downloads')):
    os.mkdir(get_dir_location('downloads'))


def get_burger():
    if not os.path.exists(get_dir_location('downloads/Burger')):
        print('\033[92mDownloading Burger...\033[m')
        os.system(
            f'cd {get_dir_location("downloads")} && git clone https://github.com/pokechu22/Burger && cd Burger && git pull')

        print('\033[92mInstalling dependencies...\033[m')
        os.system('cd downloads/Burger && pip install six jawa')


def get_version_manifest():
    if not os.path.exists(get_dir_location(f'downloads/version_manifest.json')):
        print(
            f'\033[92mDownloading version manifest...\033[m')
        version_manifest_data = requests.get(
            'https://launchermeta.mojang.com/mc/game/version_manifest.json').json()
        with open(get_dir_location(f'downloads/version_manifest.json'), 'w') as f:
            json.dump(version_manifest_data, f)
    else:
        with open(get_dir_location(f'downloads/version_manifest.json'), 'r') as f:
            version_manifest_data = json.load(f)
    return version_manifest_data


def get_version_data(version_id: str):
    if not os.path.exists(get_dir_location(f'downloads/{version_id}.json')):
        version_manifest_data = get_version_manifest()

        print(
            f'\033[92mGetting data for \033[1m{version_id}..\033[m')
        try:
            package_url = next(
                filter(lambda v: v['id'] == version_id, version_manifest_data['versions']))['url']
        except StopIteration:
            raise ValueError(
                f'No version with id {version_id} found. Maybe delete downloads/version_manifest.json and try again?')
        package_data = requests.get(package_url).json()
        with open(get_dir_location(f'downloads/{version_id}.json'), 'w') as f:
            json.dump(package_data, f)
    else:
        with open(get_dir_location(f'downloads/{version_id}.json'), 'r') as f:
            package_data = json.load(f)
    return package_data


def get_client_jar(version_id: str):
    if not os.path.exists(get_dir_location(f'downloads/client-{version_id}.jar')):
        package_data = get_version_data(version_id)
        print('\033[92mDownloading client jar...\033[m')
        client_jar_url = package_data['downloads']['client']['url']
        with open(get_dir_location(f'downloads/client-{version_id}.jar'), 'wb') as f:
            f.write(requests.get(client_jar_url).content)


def get_server_jar(version_id: str):
    if not os.path.exists(get_dir_location(f'downloads/server-{version_id}.jar')):
        package_data = get_version_data(version_id)
        print('\033[92mDownloading server jar...\033[m')
        server_jar_url = package_data['downloads']['server']['url']
        with open(get_dir_location(f'downloads/server-{version_id}.jar'), 'wb') as f:
            f.write(requests.get(server_jar_url).content)


def get_mappings_for_version(version_id: str):
    if not os.path.exists(get_dir_location(f'downloads/mappings-{version_id}.txt')):
        package_data = get_version_data(version_id)

        client_mappings_url = package_data['downloads']['client_mappings']['url']

        mappings_text = requests.get(client_mappings_url).text

        with open(get_dir_location(f'downloads/mappings-{version_id}.txt'), 'w') as f:
            f.write(mappings_text)
    else:
        with open(get_dir_location(f'downloads/mappings-{version_id}.txt'), 'r') as f:
            mappings_text = f.read()
    return Mappings.parse(mappings_text)

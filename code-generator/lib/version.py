from mappings import Mappings
import requests
import json
import os


def download_burger():
    print('\033[92mDownloading Burger...\033[m')
    os.system(
        'cd downloads && git clone https://github.com/pokechu22/Burger && cd Burger && git pull')

    print('\033[92mInstalling dependencies...\033[m')
    os.system('cd downloads/Burger && pip install six jawa')


def get_version_manifest():
    if not os.path.exists(f'downloads/version_manifest.json'):
        print(
            f'\033[92mDownloading version manifest...\033[m')
        version_manifest_data = requests.get(
            'https://launchermeta.mojang.com/mc/game/version_manifest.json').json()
        with open(f'downloads/version_manifest.json', 'w') as f:
            json.dump(version_manifest_data, f)
    else:
        with open(f'downloads/version_manifest.json', 'r') as f:
            version_manifest_data = json.load(f)
    return version_manifest_data


def get_version_data(version_id: str):
    if not os.path.exists(f'downloads/{version_id}.json'):
        version_manifest_data = get_version_manifest()

        print(
            f'\033[92mGetting data for \033[1m{version_id}..\033[m')
        package_url = next(
            filter(lambda v: v['id'] == version_id, version_manifest_data['versions']))['url']
        package_data = requests.get(package_url).json()
        with open(f'downloads/{version_id}.json', 'w') as f:
            json.dump(package_data, f)
    else:
        with open(f'downloads/{version_id}.json', 'r') as f:
            package_data = json.load(f)
    return package_data


def get_client_jar(version_id: str):
    if not os.path.exists(f'downloads/client-{version_id}.jar'):
        package_data = get_version_data(version_id)
        print('\033[92mDownloading client jar...\033[m')
        client_jar_url = package_data['downloads']['client']['url']
        with open('client.jar', 'wb') as f:
            f.write(requests.get(client_jar_url).content)


def get_burger_data_for_version(version_id: str):
    if not os.path.exists(f'downloads/burger-{version_id}.json'):
        get_client_jar(version_id)

        os.system(
            f'cd downloads/Burger && python munch.py ../client-{version_id}.jar --output ../burger-{version_id}.json'
        )
    with open(f'burger-{version_id}.json', 'r') as f:
        return json.load(f)


def get_mappings_for_version(version_id: str):
    if not os.path.exists(f'downloads/mappings-{version_id}.json'):
        package_data = get_version_data(version_id)

        client_mappings_url = package_data['downloads']['client_mappings']['url']

        mappings_text = requests.get(client_mappings_url).text

        with open(f'downloads/mappings-{version_id}.json', 'w') as f:
            f.write(mappings_text)
    else:
        with open(f'downloads/mappings-{version_id}.json', 'r') as f:
            mappings_text = f.read()
    return Mappings.parse(mappings_text)

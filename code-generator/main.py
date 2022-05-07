from mappings import Mappings
import packetcodegen
import requests
import json
import sys
import os

print(
    f'\033[92mFinding Minecraft version...\033[m')
version_manifest_data = requests.get(
    'https://launchermeta.mojang.com/mc/game/version_manifest.json').json()
minecraft_version = version_manifest_data['latest']['release']
print(
    f'\033[92mUsing \033[1m{minecraft_version}..\033[m')
package_url = next(
    filter(lambda v: v['id'] == minecraft_version, version_manifest_data['versions']))['url']
package_data = requests.get(package_url).json()
client_jar_url = package_data['downloads']['client']['url']

skipping_burger = False
try:
    with open('burger.json', 'r') as f:
        burger_data = json.load(f)[0]
        if burger_data['version']['id'] == minecraft_version:
            skipping_burger = True
            print(
                f'\033[92mSkipping Burger step because the burger.json is up-to-date.\033[m')
except FileNotFoundError:
    pass

if not skipping_burger:
    print('\033[92mDownloading Burger...\033[m')
    r = os.system('git clone https://github.com/pokechu22/Burger')
    os.system('cd Burger && git pull')
    print('\033[92mDownloading client jar...\033[m')
    with open('client.jar', 'wb') as f:
        f.write(requests.get(client_jar_url).content)

    print(f'\033[92mExtracting data with Burger...\033[m')
    os.system(
        'cd Burger && python munch.py ../client.jar --output ../burger.json')

client_mappings_url = package_data['downloads']['client_mappings']['url']
mappings = Mappings.parse(requests.get(client_mappings_url).text)

with open('burger.json', 'r') as f:
    burger_data = json.load(f)

burger_packets_data = burger_data[0]['packets']['packet']
packet_id, direction, state = int(sys.argv[1]), sys.argv[2], sys.argv[3]
print(
    f'Generating code for packet id: {packet_id} with direction {direction} and state {state}')
packetcodegen.generate(burger_packets_data, mappings,
                       packet_id, direction, state)

os.system('cd .. && cargo fmt')

print('Done!')

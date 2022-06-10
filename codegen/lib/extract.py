# Extracting data from the Minecraft jars

from lib.download import get_server_jar, get_burger, get_client_jar
from lib.utils import get_dir_location
import json
import os


def generate_data_from_server_jar(version_id: str):
    if os.path.exists(get_dir_location(f'downloads/generated-{version_id}')):
        return

    get_server_jar(version_id)
    os.system(
        f'cd {get_dir_location(f"downloads")} && java -DbundlerMainClass=net.minecraft.data.Main -jar {get_dir_location(f"downloads/server-{version_id}.jar")} --all --output \"{get_dir_location(f"downloads/generated-{version_id}")}\"'
    )


# the minecraft server jar doesn't give enough useful info so we use burger instead
# def get_block_states(version_id: str):
#     generate_data_from_server_jar(version_id)
#     with open(get_dir_location(f'downloads/generated-{version_id}/reports/blocks.json'), 'r') as f:
#         return json.load(f)
def get_block_states(version_id: str):
    burger_data = get_burger_data_for_version(version_id)
    return burger_data[0]['blocks']['block']


def get_burger_data_for_version(version_id: str):
    if not os.path.exists(get_dir_location(f'downloads/burger-{version_id}.json')):
        get_burger()
        get_client_jar(version_id)

        os.system(
            f'cd {get_dir_location("downloads/Burger")} && python munch.py ../client-{version_id}.jar --output ../burger-{version_id}.json'
        )
    with open(get_dir_location(f'downloads/burger-{version_id}.json'), 'r') as f:
        return json.load(f)

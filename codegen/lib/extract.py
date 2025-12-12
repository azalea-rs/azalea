# Extracting data from the Minecraft jars

import shutil
from lib.download import (
    get_latest_fabric_api_version,
    get_latest_fabric_kotlin_version,
    get_latest_fabric_loom_version,
    get_mappings_for_version,
    get_pumpkin_extractor,
    get_server_jar,
    get_burger,
    get_client_jar,
    get_fabric_data,
)
from lib.utils import get_dir_location, to_camel_case, upper_first_letter
from zipfile import ZipFile
import subprocess
import json
import re
import os


def generate_data_from_server_jar(version_id: str):
    if os.path.exists(get_dir_location(f"__cache__/generated-{version_id}")):
        return

    get_server_jar(version_id)
    os.system(
        f'cd {get_dir_location("__cache__")} && java -DbundlerMainClass=net.minecraft.data.Main -jar {get_dir_location(f"__cache__/server-{version_id}.jar")} --all --output "{get_dir_location(f"__cache__/generated-{version_id}")}"'
    )


def get_block_states_report(version_id: str):
    return get_report(version_id, "blocks")


def get_builtin_registries_report(version_id: str):
    return get_report(version_id, "registries")


def get_packets_report(version_id: str):
    return get_report(version_id, "packets")


def get_items_report(version_id: str):
    return get_report(version_id, "items")


def get_report(version_id: str, name: str):
    generate_data_from_server_jar(version_id)
    with open(
        get_dir_location(f"__cache__/generated-{version_id}/reports/{name}.json"), "r"
    ) as f:
        return json.load(f)


def get_registry_tags(version_id: str, name: str):
    generate_data_from_server_jar(version_id)
    tags_directory = get_dir_location(
        f"__cache__/generated-{version_id}/data/minecraft/tags/{name}"
    )
    if not os.path.exists(tags_directory):
        return {}
    tags = {}
    for root, dirs, files in os.walk(tags_directory, topdown=False):
        for name in files:
            file = os.path.join(root, name)
            relative_path = file.replace(tags_directory, "")[1:]
            if not file.endswith(".json"):
                continue
            with open(file, "r") as f:
                tags[relative_path[:-5]] = json.load(f)
    return tags


# note that these are different from "builtin" registries
def get_data_registries(version_id: str):
    generate_data_from_server_jar(version_id)
    data_registries_dir = get_dir_location(
        f"__cache__/generated-{version_id}/data/minecraft"
    )
    registries = {}

    def add_entries_in_dir(parent_dir, registry_name):
        entries = []
        for variant_dir in os.listdir(os.path.join(parent_dir, registry_name)):
            if not variant_dir.endswith(".json"):
                continue
            entries.append(variant_dir[:-5])
        if len(entries) > 0:
            registries[registry_name] = entries

    for registry_name in os.listdir(data_registries_dir):
        add_entries_in_dir(data_registries_dir, registry_name)
    for registry_name in os.listdir(os.path.join(data_registries_dir, "worldgen")):
        if registry_name != "biome":
            continue
        add_entries_in_dir(data_registries_dir, os.path.join("worldgen", registry_name))

    return registries


python_command = None


def determine_python_command():
    return "venv/bin/python"


def run_python_command_and_download_deps(command):
    print(">", command)
    for _ in range(10):
        p = subprocess.Popen(command, stderr=subprocess.PIPE, shell=True)

        stderr = b""
        while True:
            data = p.stderr.read()
            if data == b"":
                break
            print(data.decode(), end="", flush=True)
            stderr += data

        regex_match = re.search(
            r"ModuleNotFoundError: No module named \'(\w+?)\'", stderr.decode()
        )
        if not regex_match:
            out, err = p.communicate()
            if out:
                print(out)
            if err:
                print(err)
            break
        missing_lib = regex_match.group(1)
        print("Missing required lib:", missing_lib)
        subprocess.run(
            f"venv/bin/pip install {missing_lib}",
            cwd=os.path.dirname(os.path.dirname(__file__)),
        )
    print("ok")


def get_burger_data_for_version(version_id: str):
    if not os.path.exists(get_dir_location(f"__cache__/burger-{version_id}.json")):
        get_burger()
        get_client_jar(version_id)
        get_mappings_for_version(version_id)

        print("\033[92mRunning Burger...\033[m")
        run_python_command_and_download_deps(
            f"cd {get_dir_location('__cache__/Burger')} && "
            f"venv/bin/python munch.py {get_dir_location('__cache__')}/client-{version_id}.jar "
            f"--output {get_dir_location('__cache__')}/burger-{version_id}.json "
            f"--mappings {get_dir_location('__cache__')}/mappings-{version_id}.txt"
        )
    with open(get_dir_location(f"__cache__/burger-{version_id}.json"), "r") as f:
        return json.load(f)


def get_pumpkin_data(version_id: str, category: str):
    assert "/" not in version_id
    assert "\\" not in version_id
    target_parent_dir = get_dir_location(f"__cache__/pumpkin-{version_id}")
    category_dir = f"{target_parent_dir}/{category}.json"

    if os.path.exists(category_dir):
        with open(category_dir, "r") as f:
            return json.load(f)

    pumpkin_dir = get_pumpkin_extractor()

    pumpkin_run_directory = f"{pumpkin_dir}/run"

    if os.path.exists(pumpkin_run_directory):
        shutil.rmtree(pumpkin_run_directory)
    os.makedirs(pumpkin_run_directory, exist_ok=True)
    with open(f"{pumpkin_run_directory}/eula.txt", "w") as f:
        f.write("eula=true")
    with open(f"{pumpkin_run_directory}/server.properties", "w") as f:
        f.write("server-port=0")

    fabric_data = get_fabric_data(version_id)[0]
    fabric_api_version = get_latest_fabric_api_version()
    fabric_kotlin_version = get_latest_fabric_kotlin_version()
    fabric_loom_version = get_latest_fabric_loom_version()

    gradle_properties = f"""# Done to increase the memory available to gradle.
org.gradle.jvmargs=-Xmx1G
org.gradle.parallel=true
# Fabric Properties
# check these on https://modmuss50.me/fabric.html
minecraft_version={version_id}
yarn_mappings={fabric_data["mappings"]["version"]}
loader_version={fabric_data["loader"]["version"]}
kotlin_loader_version={fabric_kotlin_version}
# Mod Properties
mod_version=1.0-SNAPSHOT
maven_group=de.snowii
archives_base_name=extractor
fabric_version={fabric_api_version}
"""
    with open(f"{pumpkin_dir}/gradle.properties", "w") as f:
        f.write(gradle_properties)

    # update the minecraft version dependency in src/main/resources/fabric.mod.json
    fabric_mod_json_path = f"{pumpkin_dir}/src/main/resources/fabric.mod.json"
    with open(fabric_mod_json_path, "r") as f:
        fabric_mod_json = f.read()
    with open(fabric_mod_json_path, "w") as f:
        fabric_mod_json = fabric_mod_json.replace(
            '"minecraft": "${minecraft_version}"', '"minecraft": "*"'
        )
        f.write(fabric_mod_json)
    with open(f"{pumpkin_dir}/build.gradle.kts", "r") as f:
        build_gradle_kts = f.read()
    with open(f"{pumpkin_dir}/build.gradle.kts", "w") as f:
        build_gradle_kts = re.sub(
            r'(id\("fabric-loom"\) version )"[^"]+"',
            rf'\1"{fabric_loom_version}"',
            build_gradle_kts,
        )
        # kotlin complains about nullable types if we don't add this
        build_gradle_kts = re.sub(
            r'(to project.property\("\w+"\))([\n,])', r"\1!!\2", build_gradle_kts
        )
        f.write(build_gradle_kts)

    # run ./gradlew runServer until it logs "(pumpkin_extractor) Done"
    p = subprocess.Popen(
        # the gradle wrapper (./gradlew) is sometimes on the wrong version so just prefer the system's gradle installation
        f"cd {pumpkin_dir} && gradle clean && gradle runServer",
        stderr=subprocess.PIPE,
        stdout=subprocess.PIPE,
        shell=True,
    )

    while True:
        data = p.stdout.readline().decode()
        print(">" + data, end="", flush=True)
        if "[Server thread/INFO] (pumpkin_extractor) Done" in data:
            print("Pumpkin extractor done")
            break
        if data == "":
            break

    p.terminate()

    # move the run/pumpkin_extractor_output directory to target_parent_dir
    # delete target_parent_dir if it's empty
    if os.path.exists(target_parent_dir):
        os.rmdir(target_parent_dir)
    os.rename(f"{pumpkin_dir}/run/pumpkin_extractor_output", target_parent_dir)

    with open(category_dir, "r") as f:
        return json.load(f)


def get_file_from_jar(version_id: str, file_dir: str):
    get_client_jar(version_id)
    with ZipFile(get_dir_location(f"__cache__/client-{version_id}.jar")) as z:
        with z.open(file_dir) as f:
            return f.read()


def get_en_us_lang(version_id: str):
    return json.loads(get_file_from_jar(version_id, "assets/minecraft/lang/en_us.json"))


# burger packet id extraction is broken since 1.20.5 (always returns -1, so we have to determine packet id ourselves from the mappings).
# this is very much not ideal.


def get_packet_list(version_id: str):
    if version_id != "1.21":
        return []

    generate_data_from_server_jar(version_id)
    with open(
        get_dir_location(f"__cache__/generated-{version_id}/reports/packets.json"), "r"
    ) as f:
        packets_report = json.load(f)
    packet_list = []
    for state, state_value in packets_report.items():
        for direction, direction_value in state_value.items():
            for packet_identifier, packet_value in direction_value.items():
                assert packet_identifier.startswith("minecraft:")
                packet_identifier = upper_first_letter(
                    to_camel_case(packet_identifier[len("minecraft:") :])
                )
                packet_list.append(
                    {
                        "state": state,
                        "direction": direction,
                        "name": packet_identifier,
                        "id": packet_value["protocol_id"],
                    }
                )

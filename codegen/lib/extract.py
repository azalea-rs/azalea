# Extracting data from the Minecraft jars

from lib.download import get_server_jar, get_burger, get_client_jar, get_pixlyzer, get_yarn_data, get_fabric_api_versions, get_fabric_loader_versions
from lib.utils import get_dir_location
from zipfile import ZipFile
import subprocess
import requests
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
        os.system(
            f'{determine_python_command()} -m pip install {missing_lib}')
    print('ok')


def get_burger_data_for_version(version_id: str):
    if not os.path.exists(get_dir_location(f'downloads/burger-{version_id}.json')):
        get_burger()
        get_client_jar(version_id)

        print('\033[92mRunning Burger...\033[m')
        run_python_command_and_download_deps(
            f'cd {get_dir_location("downloads/Burger")} && {determine_python_command()} munch.py {get_dir_location("downloads")}/client-{version_id}.jar --output {get_dir_location("downloads")}/burger-{version_id}.json'
        )
    with open(get_dir_location(f'downloads/burger-{version_id}.json'), 'r') as f:
        return json.load(f)


def get_pixlyzer_data(version_id: str, category: str):
    '''
    Gets data from Pixlyzer. Note that this requires Yarn to release updates first.
    '''

    target_dir = get_dir_location(f'downloads/pixlyzer-{version_id}')

    # TODO: right now this False is hard-coded, it should retry with this
    # enabled if # initially getting the data fails
    if True or (os.path.exists(target_dir) and not os.path.exists(f'{target_dir}/{category}.min.json')):
        print('Downloading', category, 'from pixlyzer-data.')
        data = requests.get(f'https://gitlab.com/Bixilon/pixlyzer-data/-/raw/master/version/{version_id}/{category}.min.json?inline=false').text
        try:
            os.mkdir(target_dir)
        except:
            pass
        with open(f'{target_dir}/{category}.min.json', 'w') as f:
            f.write(data)
        return json.loads(data)

    if not os.path.exists(target_dir):
        pixlyzer_dir = get_pixlyzer()

        # for some reason pixlyzer doesn't work right unless the mvn clean
        # instruction looks like that
        # and pixlyzer.py doesn't do it right

        # map jar + download dependencies
        run_python_command_and_download_deps(
            f'cd {pixlyzer_dir}/wrapper && {determine_python_command()} PixLyzer.py --only-version={version_id} --dont-compile --only-map'
        )
        # update the pom.xml <dependencies>
        # list directories in pixlyzer/wrapper/data/data/dependencies/libraries
        pom_xml_dependencies = '''<dependency>
            <groupId>org.jetbrains.kotlin</groupId>
            <artifactId>kotlin-test-junit</artifactId>
            <version>1.7.21</version>
            <scope>test</scope>
        </dependency>
        <dependency>
            <groupId>org.jetbrains.kotlin</groupId>
            <artifactId>kotlin-stdlib-jdk8</artifactId>
            <version>1.7.21</version>
        </dependency>

        <dependency>
            <groupId>net.minecraft</groupId>
            <artifactId>client</artifactId>
            <version>${minecraft.version}</version>
            <scope>system</scope>
            <systemPath>${project.basedir}/wrapper/data/data/${minecraft.version}_yarn/${minecraft.version}-exhibitionism.jar</systemPath>
        </dependency>
        <dependency>
            <groupId>de.bixilon</groupId>
            <artifactId>mbf-kotlin</artifactId>
            <version>0.2.1</version>
        </dependency>
        <dependency>
            <groupId>org.objenesis</groupId>
            <artifactId>objenesis</artifactId>
            <version>3.3</version>
        </dependency>
        <dependency>
            <groupId>org.apache.commons</groupId>
            <artifactId>commons-lang3</artifactId>
            <version>3.12.0</version>
        </dependency>
        <dependency>
            <groupId>com.fasterxml.jackson.core</groupId>
            <artifactId>jackson-databind</artifactId>
            <version>2.14.0</version>
        </dependency>
        <dependency>
            <groupId>de.bixilon</groupId>
            <artifactId>kutil</artifactId>
            <version>1.17.1</version>
        </dependency>'''
        # walk dir f'{pixlyzer_dir}/wrapper/data/data/dependencies/libraries'
        for root, dirs, files in os.walk(f'{pixlyzer_dir}/wrapper/data/data/dependencies/libraries'):
            for file in files:
                full_path = os.path.join(
                    root.replace('\\', '/').replace(
                        f'{pixlyzer_dir}/wrapper/data/data/dependencies/libraries/'.replace('\\', '/'), ''),
                    file
                ).replace('\\', '/')
                print(full_path)
                if not full_path.endswith('.jar'):
                    continue
                split_path = full_path.split('/')
                group = ''
                for group_index in range(0, len(split_path) - 3):
                    group += split_path[group_index] + '.'
                if group.endswith('.'):
                    group = group[:-1]
                artifact = split_path[-3]
                version = split_path[-2]
                path = '${project.basedir}/wrapper/data/data/dependencies/libraries/' + full_path
                pom_xml_dependencies += """
                    <dependency>
                        <groupId>""" + group + """</groupId>
                        <artifactId>""" + artifact + """</artifactId>
                        <version>""" + version + """</version>
                        <scope>system</scope>
                        <systemPath>""" + path + """</systemPath>
                    </dependency>
                    """
        print('pom_xml_dependencies', pom_xml_dependencies)
        assert pom_xml_dependencies != ''
        pom_xml = open(f'{pixlyzer_dir}/pom.xml', 'r').read()
        pom_xml = re.sub(
            '<dependencies>.*?</dependencies>', f'<dependencies>{pom_xml_dependencies}</dependencies>', pom_xml, flags=re.DOTALL)
        open(f'{pixlyzer_dir}/pom.xml', 'w').write(pom_xml)

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
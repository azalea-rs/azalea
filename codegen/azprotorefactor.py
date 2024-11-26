import os

path = 'azalea-protocol/src/packets/status'

# rename all the files from like "c_ping_packet.rs" to "c_ping.rs"

for filename in os.listdir(path):
    if filename.endswith('_packet.rs'):
        new_filename = filename.replace('c_', 'c_').replace('s_', 's_').replace('_packet', '')
        print(filename, new_filename)
        os.rename(os.path.join(path, filename), os.path.join(path, new_filename))

        with open(os.path.join(path, new_filename), 'r') as f:
            contents = f.read()
            contents = contents.replace('Packet {', ' {')
        with open(os.path.join(path, new_filename), 'w') as f:
            f.write(contents)


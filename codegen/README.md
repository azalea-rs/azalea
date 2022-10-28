Tools for automatically generating code to help with updating Minecraft versions.

The directory name doesn't start with `azalea-` because it's not a Rust crate.

## Requirements

- Python 3.8+
- Java 17+
- Gradle

## Usage

Generate packet:\
`python newpacket.py [packet id] [clientbound or serverbound] \[game/handshake/login/status\]`\
This will create a new file in the `azalea-protocol/src/packets/\[state\] directory`. You will probably have to manually fix up the auto generated code.

Migrate to a new Minecraft version:\
`python migrate.py [new version]`\
This updates all the packet ids in `azalea-protocol/src/packets/mod.rs` and creates all the new packets.

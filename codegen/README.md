Tools for automatically generating code to help with updating Minecraft versions.

The directory name doesn't start with `azalea-` because it's not a Rust crate.

## Requirements

- Python 3.8+
- Java 17+
- Maven

## Usage

Generate packet:\
`python newpacket.py [packet id] [clientbound or serverbound] [game/handshake/login/status]`\
This will create a new file in the `azalea-protocol/src/packets/[state] directory`. You will probably have to manually fix up the auto generated code.

## Updating to a new Minecraft version

First, run `python migrate.py [new version]`. This will run a script that automatically updates as much as it can, including:
- Adding, removing, and updating packets in azalea-protocol (limited)
- Updating supported version in README.md
- Updating the `PROTOCOL_VERSION` variable in azalea-protocol
- Generating blocks in azalea-block
- Generating block shapes in azalea-physics
- Generating registries in azalea-registries
- Updating en_us.json in azalea-language
- Generating entity metadata structs and parsers in azalea-world

If you're lucky, that's all you're going to have to do.
Look at the diff (`git diff`) and type-check the code (`cargo check`) to make sure everything is right. In the diff, specifically look for new comments that have "TODO".

If a packet is incorrect, you'll want to find it in the Minecraft source. The name of the struct should be the same or similar as the class in the vanilla source. Now, you'll have to manually write the struct for the packet. If the packet existed in the version before and it's just being updated, you can compare against that to see what was updated. Note that if a packet is particularly complicated, you may have to implement McBufReadable and McBufWritable, but most of the time the `#[derive(McBuf)]` macro will be able to generate the impls correctly. Look at other existing packets as reference if you're confused.

Finally, test by making a bot join a world. Specifically, you'll want to test the things that were updated in the version. Setting the RUST_LOG environment variable to `debug` or `trace` may help you find the source of crashes (trace shows the first few hundred bytes for every packet received so it's typically more useful, but it may log more than you want).

If it all works, make a pull request. If the version you updated to is a snapshot, make it a draft PR (the main branch is for release versions).

## Extracting new data

At the time of writing, the following data generators are used:

- [Vanilla data generator](https://wiki.vg/Data_Generators)
- [Burger](https://github.com/mat-1/Burger)
- [PixLyzer](https://gitlab.bixilon.de/bixilon/pixlyzer)

Some things can be obtained from multiple generators. You should prefer them by the order above (the vanilla generator is the most reliable).


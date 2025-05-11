# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
Due to the complexity of Azalea and the fact that almost every Minecraft version
is breaking anyways, semantic versioning is not followed.

## [Unreleased]

### Added

- This changelog. To see changes before this update, look at the git commits.
- azalea and azalea-client now have a `packet-event` feature, which can be disabled for efficiency if you're not using `Event::Packet`.
- `StartJoinServerEvent` can now be used to join servers exclusively from the ECS without a Tokio runtime.
- `FormattedText::to_html` and `FormattedText::to_custom_format`. (@Kumpelinus)
- Non-standard legacy hex colors like `§#ff0000` are now supported in azalea-chat.
- Chat signing.
- Add auto-reconnecting which is enabled by default.
- `client.start_use_item()`.
- The pathfinder no longer avoids slabs, stairs, and dirt path blocks.
- The reach distance for the pathfinder `ReachBlockPosGoal` is now configurable. (@x-osc)

### Changed

- [BREAKING] `Client::goto` is now async and completes when the client reaches its destination. `Client::start_goto` should be used if the old behavior is desired.
- [BREAKING] The `BlockState::id` field is now private, use `.id()` instead.
- [BREAKING] Update to [Bevy 0.16](https://bevyengine.org/news/bevy-0-16/).
- [BREAKING] Rename `InstanceContainer::insert` to `get_or_insert`.
- [BREAKING] Replace `BlockInteractEvent` with the more general-purpose `StartUseItemEvent`.
- `ClientBuilder` and `SwarmBuilder` are now Send.

### Fixed

- Clients now validate incoming packets using the correct `MAXIMUM_UNCOMPRESSED_LENGTH` value.
- Several protocol fixes, including for ClientboundSetPlayerTeam and a few data components.
- No more chunk errors when the client joins another world with the same name but different height.
- Mining now aborts correctly and doesn't flag Grim.
- Update the `InstanceName` component correctly when we receive a respawn or second login packet.
- azalea-chat now handles legacy color codes correctly when parsing from NBT.
- Send the correct UUID to servers in `ClientboundHello` when we're joining in offline-mode.
- Block shapes and some properties were using data from `1.20.3-pre4` due to using an old data generator (Pixlyzer), which has now been replaced with the data generator from [Pumpkin](https://github.com/Pumpkin-MC/Extractor).

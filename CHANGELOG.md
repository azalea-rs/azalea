# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
Due to the complexity of Azalea and the fact that almost every Minecraft version
is breaking anyways, semantic versioning is not followed.

## [Unreleased]

### Added

- `HitResult` now contains the entity that's being looked at.
- A `QueuedServerBlockUpdates` component that keeps track of block updates per `Update`.
- Local clients now have a `TicksConnected` component. (@Kumpelinus)
- There is now a `azalea_inventory::default_components::get_default_component` function to get the default value of a component for a registry item.
- `ItemStack` now has a `get_component` function that supports default components.

### Changed

- Update to Minecraft 1.21.8.
- Renamed `azalea_entity::EntityKind` to `EntityKindComponent` to disambiguate with `azalea_registry::EntityKind`.
- Moved functions and types related to hit results from `azalea::interact` to `azalea::interact::pick`.
- `Client::attack` now takes `Entity` instead of `MinecraftEntityId`.
- `ItemStackData::components` was renamed to `component_patch`.
- The fields in `LookDirection` have been replaced with getters.

### Fixed

- Fix packet order for loading (`PlayerLoaded`/`MovePlayerPos`) and sprinting (`PlayerInput`/`PlayerCommand`).
- Clients no longer send invalid look directions if the server teleports us with one.
- Movement code was updated with the changes from 1.21.5, so it no longer flags Grim.
- `azalea-chat` now handles arrays of integers in the `with` field. (@qwqawawow)
- `azalea-chat` no longer incorrectly persists styles of components in the "extra" field.
- Inventories now use the correct max stack sizes.
- Clients now send the correct data component checksums when interacting with items.
- Fix parsing some metadata fields of Display entities.
- Look directions are now rounded based on the default Minecraft sensitivity, which may help avoid flagging anticheats.

## [0.13.0+mc1.21.5] - 2025-06-15

### Added

- This changelog. To see changes before this update, look at the git commits.
- azalea and azalea-client now have a `packet-event` feature, which can be disabled for efficiency if you're not using `Event::Packet`.
- `StartJoinServerEvent` can now be used to join servers exclusively from the ECS without a Tokio runtime.
- Add `FormattedText::to_html` and `FormattedText::to_custom_format`. (@Kumpelinus)
- Non-standard legacy hex colors like `§#ff0000` are now supported in azalea-chat.
- Chat signing.
- Add auto-reconnecting which is enabled by default.
- `ClientBuilder` and `SwarmBuilder` are now Send.
- Add `Client::start_use_item`.
- The pathfinder no longer avoids slabs, stairs, and dirt path blocks.
- The pathfinder now immediately recalculates if blocks are placed in its path.
- Bots that use custom pathfinder moves can now keep arbitrary persistent state by using the `CustomPathfinderState` component and `PathfinderCtx::custom_state`.
- The reach distance for the pathfinder `ReachBlockPosGoal` is now configurable. (@x-osc)
- There is now a `retry_on_no_path` option in `GotoEvent` that can be set to false to make the pathfinder give up if no path could be found.
- azalea-brigadier now supports suggestions, command contexts, result consumers, and returning errors with `ArgumentBuilder::executes_result`.
- Proper support for getting biomes at coordinates.
- Add a new `Client::entities_by` which sorts entities that match a criteria by their distance to the client.
- New client event `Event::ReceiveChunk`.
- Several new functions for interacting with inventories (`Client::get_inventory`, `get_held_item`, `ContainerHandleRef::left_click`, `shift_click`, `right_click`, `slots`).
- Add `Client::mine_with_auto_tool`.
- Add `Client::set_selected_hotbar_slot` and `Client::selected_hotbar_slot`.
- Add `Client::attack_cooldown_remaining_ticks` to complement `has_attack_cooldown`.
- Add `BlockPos::length`, `distance_to`, and `center_bottom`.

### Changed

- `Client::goto` is now async and completes when the client reaches its destination. `Client::start_goto` should be used if the old behavior is desired.
- The `BlockState::id` field is now private, use `.id()` instead.
- Update to [Bevy 0.16](https://bevyengine.org/news/bevy-0-16/).
- Rename `InstanceContainer::insert` to `get_or_insert`.
- Replace `BlockInteractEvent` with the more general-purpose `StartUseItemEvent`.
- Replace `wait_one_tick` and `wait_one_update` with `wait_ticks` and `wait_updates`.
- Functions that took `&Vec3` or `&BlockPos` as arguments now only take them as owned types.
- Rename `azalea_block::Block` to `BlockTrait` to disambiguate with `azalea_registry::Block`.
- `GotoEvent` is now non-enhaustive and should instead be constructed by calling its methods.

### Fixed

- Clients now validate incoming packets using the correct `MAXIMUM_UNCOMPRESSED_LENGTH` value.
- Several protocol fixes, including for `ClientboundSetPlayerTeam` and a few data components.
- No more chunk errors when the client joins another world with the same name but different height.
- Update the `InstanceName` component correctly when we receive a respawn or second login packet.
- azalea-chat now handles legacy color codes correctly when parsing from NBT.
- Send the correct UUID to servers in `ClientboundHello` when we're joining in offline-mode.
- Block shapes and some properties were using data from `1.20.3-pre4` due to using an old data generator (Pixlyzer), which has now been replaced with the data generator from [Pumpkin](https://github.com/Pumpkin-MC/Extractor).
- When patching the path, don't replace the move we're currently executing.
- The correct sequence number is now sent when interacting with blocks.
- Mining is now generally more reliable and doesn't flag Grim.
- Ghost blocks are now handled correctly due to implementing `ClientboundBlockChangedAck`.
- Player eye height was wrong due to being calculated from height instead of being a special case (was 1.53, should've been 1.62).
- The player inventory is now correctly updated when we close a container.
- Inventory interactions are now predicted on the client-side again, and the remaining click operations were implemented.
- `Client::open_container_at` now waits up to 10 ticks for the block to exist if you try to click air.
- Wrong physics collision code resulted in `HitResult` sometimes containing the wrong coordinates and `inside` value.
- Fix the client being unresponsive for a few seconds after joining due to not sending `ServerboundPlayerLoaded`.
- Fix panic when a client received `ClientboundAddEntity` and `ClientboundStartConfiguration` at the same time.
- Fix panic due to `ClientInformation` being inserted too late.
- `ClientboundTeleportEntity` did not handle relative teleports correctly.
- Pathfinder now gets stuck in water less by automatically trying to jump if it's in water.

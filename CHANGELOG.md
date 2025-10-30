# Changelog

All notable changes to Azalea will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
Due to the large scope of Azalea and the fact that almost every Minecraft version
is breaking anyways, semantic versioning is not followed.

## [Unreleased]

### Added

- Add `Client::query_entity` and `try_query_entity` to complement `query_self`.
- Add `Client::entity_interact` and `EntityInteractEvent` to interact with entities without checking that they're in the crosshair.
- Implement initial support for mob effects, including jump boost, haste, conduit power, and mining fatigue. (@ShayBox)

### Changed

- Update to Minecraft 1.21.10. (help from @eihqnh)
- Update to Bevy 0.17.
- `Client::query`, `map_component`, and `map_get_component` were replaced by `Client::query_self`.
- Rename `SendPacketEvent` to `SendGamePacketEvent` and `PingEvent` to `GamePingEvent`.
- Swap the order of the type parameters in entity filtering functions so query is first, then filter.
- Add optional `timeout_ticks` field to `Client::open_container_at`.

### Fixed

- The wrong path was temporarily executed if we received a `GotoEvent` while the path that's being executed was more than 50 nodes long.
- The pathfinder can now jump from dirt path and farmland blocks correctly.
- Don't panic when receiving an unexpected `PathFoundEvent`. (@Hiradpi)
- Don't panic when the `LocalPlayerEvents` component is missing. (@suprohub)
- The pathfinder sometimes got stuck when going up stairs that are facing the wrong direction.
- ReachBlockPosGoal had the wrong cost when the destination is surrounded in blocks.
- Some parkour movements had the wrong costs.
- The pathfinder no longer spins when descending more than one block.
- The pathfinder now avoids slipping off when the last block of the path is on ice.
- The 'with' field in formatted text didn't correctly support mixed types. (@Tert0)
- The WritableBookContent and ResolvableProfile data components had the wrong protocol implementations.

## [0.14.0+mc1.21.8] - 2025-09-28

### Added

- Sneaking/crouching.
- `HitResult` now contains the entity that's being looked at.
- A `QueuedServerBlockUpdates` component that keeps track of block updates per `Update`.
- Local clients now have a `TicksConnected` component. (@Kumpelinus)
- There is now a `azalea_inventory::default_components::get_default_component` function to get the default value of a component for a registry item.
- `ItemStack` now has a `get_component` function that supports default components.
- `Client::nearest_entity_by`.
- Blocks now have functions for getting property keys and values as strings. (@urisinger)
- `BitSet::len`, `BitSet::get`, `BitSet::iter_ones`.
- All packets are now `PartialEq`.
- The `fallback` field was implemented for chat messages. (@Tert0)
- Interactive auth now appends `?otc={code}` to the login URL to skip having to manually paste the auth code.

### Changed

- Update to Minecraft 1.21.8.
- Renamed `azalea_entity::EntityKind` to `EntityKindComponent` to disambiguate with `azalea_registry::EntityKind`.
- Moved functions and types related to hit results from `azalea::interact` to `azalea::interact::pick`.
- `Client::attack` now takes `Entity` instead of `MinecraftEntityId`.
- `ItemStackData::components` was renamed to `component_patch`.
- The fields in `LookDirection` have been replaced with getters.
- Renamed `Client::entity_by` to `any_entity_by`, and `Client::entities_by` to `nearest_entities_by`.
- `EyeHeight` was moved into `EntityDimensions`, and `EntityDimensions` is now its own component.
- Replaced `start_goto_without_mining` with `start_goto_with_opts`.
- Rename `send_chat_packet` / `send_command_packet` to `write_chat_packet` / `write_command_packet` (for consistency with `write_packet`).
- Split `ClientInformation` handling out of `BrandPlugin` to `ClientInformationPlugin`.
- `ClientBuilder::start` and `SwarmBuilder::start` now return a `Result<AppExit>` instead of `Result<!>`.
- `ClientsideCloseContainerEvent`, `MenuOpenedEvent`, and `CloseContainerEvent` are now triggers instead of events.
- `Client::chat` now takes anything with `impl Into<String>`.
- Some types related Azalea's bot plugin were moved to `azalea::bot::*`.
- `AABB` was renamed to `Aabb` to follow Rust naming guidelines.

### Fixed

- Fix packet order for loading (`PlayerLoaded`/`MovePlayerPos`), sprinting (`PlayerInput`/`PlayerCommand`), and `CarriedItem`.
- Clients no longer send invalid look directions if the server teleports us with one.
- Look directions are now rounded based on the default Minecraft sensitivity, which may help avoid flagging anticheats.
- Movement code was updated with the changes from 1.21.5, so it no longer flags Grim.
- Clients can no longer sprint if their food level is too low.
- `azalea-chat` now handles arrays of integers in the `with` field. (@qwqawawow)
- `azalea-chat` no longer incorrectly persists styles of components in the "extra" field.
- `dark_red` was way too dark red.
- Inventories now use the correct max stack sizes.
- Clients now send the correct data component checksums when interacting with items.
- Fix parsing some metadata fields of Display entities.
- Mining blocks in creative mode now works. (@eihqnh)
- Improved matchers on the `ChatPacket` functions to work on more servers. (@ShayBox)
- Bevy's `AppExit` Event is now handled by Azalea's ECS runner.
- Pathfinding now works over farmland blocks.
- There is no longer a panic when the account token is automatically refreshed.
- Fix `is_valid_id` on registries incorrectly returning true for values equal to the length.
- Fix outdated implementation for the `ClientboundMerchantOffers` packet.
- Fix compilation with new dependency versions. (@ShayBox)

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

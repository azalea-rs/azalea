Fuzzing for `azalea-protocol`.

Note that while Azalea doesn't try particularly hard to avoid crash exploits (because Azalea is generally used as a client), panics/crashes in the protocol library are still considered to be bugs that should be fixed.

Feel free to run this fuzzer for however long as you'd like to find crashes in `azalea-protocol`! It may take a very long time to find complex ones. Also, the input length is limited to 4 KiB by default.

Additionally, you should be aware that this fuzzer only targets `azalea-protocol`'s packet deserialization logic. There may be other crash bugs lurking outside of that.

## Usage

```sh
cargo install cargo-fuzz

cargo fuzz run clientbound_game -s none -- -rss_limit_mb=16384 -malloc_limit_mb=1024
# other valid targets: {clientbound,serverbound}_{config,game,handshake,login,status}
# note: the rss_limit_mb is increased (from the default of 2048) so libfuzzer
# doesn't oom due to the branchiness of the code :(

# also, the `-s none` is there for increased performance, but at the cost of catching less bugs. feel free to remove it.

# also see https://appsec.guide/docs/fuzzing/rust/cargo-fuzz/
```


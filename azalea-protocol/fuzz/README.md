Fuzzing for `azalea-protocol`.

## Usage

```sh
cargo fuzz run clientbound_game # {clientbound,serverbound}_{config,game,handshake,login,status}
# optionally, add `-s none` for a speedup at the cost of catching less memory safety issues
# see https://appsec.guide/docs/fuzzing/rust/cargo-fuzz/#addresssanitizer
```


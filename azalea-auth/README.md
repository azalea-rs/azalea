# Azalea Auth

A port of Mojang's Authlib and launcher authentication.

# Examples

```no_run
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    let cache_file = PathBuf::from("example_cache.json");

    let auth_result = azalea_auth::auth(
        "example@example.com",
        azalea_auth::AuthOpts {
            cache_file: Some(cache_file),
            ..Default::default()
        },
    )
    .await
    .unwrap();
    println!("{auth_result:?}");
}
```

Thanks to [wiki.vg contributors](https://wiki.vg/Microsoft_Authentication_Scheme), [Overhash](https://gist.github.com/OverHash/a71b32846612ba09d8f79c9d775bfadf), and [prismarine-auth contributors](https://github.com/PrismarineJS/prismarine-auth).

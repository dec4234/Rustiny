# Rustiny
A wrapper for the Destiny 2 / Bungie.net API written in rust.

Check out the [JavaDestinyAPI](https://github.com/dec4234/JavaDestinyAPI), if you need a Java implementation.

[![Discord Banner 2](https://discordapp.com/api/guilds/847480795232993280/widget.png?style=banner2)](https://discord.gg/dvZmP92d4h)

## Usage
(Rustiny usually uses the most recent version of Rust)

In addition to these examples, check out src/lib.rs for the tests that I use.

____

**Getting a User With A Name And Discriminator**
```rust
let client = ApiInterface::new("YOUR API KEY HERE", true).await;
println!("{}", BungieUser::get_user_by_name_and_discrim_with_platform(client, String::from("dec4234#9904"), DestinyPlatform::All)
    .await
    .unwrap()
    .primary.global_display_name);
```

## Development Outlook
1. ### Stats
   1. Users
   2. Clans **~~<--~~ Here**
   3. Characters
   4. Activity History / PGCRs
   5. Manifest
2. ### OAuth
   1. Implement OAuth API
   2. Support OAuth Flow
   3. Users
   4. Clans
   5. Social



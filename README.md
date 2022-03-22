# Rustiny
A wrapper for the Destiny 2 / Bungie.net API written in rust.

Check out the [JavaDestinyAPI](https://github.com/dec4234/JavaDestinyAPI), if you need a Java implementation.

[![Discord Banner 2](https://discordapp.com/api/guilds/847480795232993280/widget.png?style=banner2)](https://discord.gg/dvZmP92d4h)

Check it out on [crates.io](https://crates.io/crates/Rustiny)

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

**Get the name of the founder of a clan**
```rust
let clan = Clan::get_by_name(get_api().await.client, "Heavenly Mayhem").await.unwrap();

println!("{}", clan.founder.destinyUserInfo.global_display_name);
```

## Development Outlook
1. ### Stats
   1. Users
      1. User from Steam ID
   2. ~~Clans~~ 
   3. ~~Characters~~ 
   4. ~~Activity History / PGCRs~~
   5. Manifest 
   6. Destiny Item / Weapons / Armor
   7. Item Searching
   8. Character Equipment
   9. Collections
   10. Triumphs
   11. Metrics?
   12. Crafting Recipe Items?
   13. Leaderboards
       1. Clan - Aggregate/Leaderboards
       2. Character
       3. Destiny Aggregate Activity Stats
2. ### OAuth
   1. Implement OAuth API
   2. Support OAuth Flow
   3. Users
      1. Request to join clan
      2. Unequipped items
   4. Clans
      1. Pending/Banned Members
      2. Accepting / Inviting
      3. Banning / Kicking
      4. Updating description / other details
   5. Social
   6. Inventory Management
      1. Transfer from postmaster
      2. Transfer to vault
      3. Transfer to other character
      4. Insert Plugs
   7. Misc.
      1. Report PGCR player

## Version 0.1.12 - March 22, 2022
- Wrapped up PGCRs
- Added get_activity_history()
- Added Tester struct to prevent unnecessary repetition
- Clan Founder info
- Clan Weekly Rewards
- Clan Weekly Reward Manifest puller
- Added more stuff to Development To-Do List
- Add another example to the README



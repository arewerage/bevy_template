# My simple project template for [Bevy](https://github.com/bevyengine/bevy)

## Tested only on Windows 10!
Add Linux support (```Cargo.toml```):
```
[features]
default = [
    "bevy/x11",
    # or
    "bevy/wayland",
]
```
Mac OS support is there initially.

## Quick launch (Ctrl + Shift + B in VSCode):
```
cargo run --features dev
```

## Additional crates:
* [iyes_loopless](https://github.com/IyesGames/iyes_loopless)
* [bevy-inspector-egui](https://github.com/jakobhellermann/bevy-inspector-egui)
* [bevy_asset_loader](https://github.com/NiklasEi/bevy_asset_loader)

## Additional Information:
Mainly based on [bevy_game_template by NiklasEi](https://github.com/NiklasEi/bevy_game_template).

New functionality will be added based on my own needs.
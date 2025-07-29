Prerequisites: 
1. A way to launch ACNH mods... duh (either via Atmosphere or emulator)
2. Appropriate exeFS files needed to use Skyline with ACNH (https://cdn.discordapp.com/attachments/1291491226025201674/1312311037965373500/exefs.zip?ex=68891925&is=6887c7a5&hm=716e954951761cfd0843b24c4a5936c6ba88ed219d59b3b451e93cb8653a2fee&)
3. Rust and `cargo-skyline`
4. A modified `ItemUnitIcon.bcsv`, `ItemParam.bcsv`, and model files that include entries for your item

How to Use:
1. Modify `icon/mod.rs` to include your entries (make sure they are named as they are in your modified `ItemUnitIcon.bcsv`).
2. Open command prompt in the root folder and type "`cargo skyline build`".
3. Put the newly-created `.nro` file found under `/target/aarch64-skyline-switch/debug/` into your mod's romFS under `romFS/skyline/plugins/`.
4. Launch the game and (hopefully) enjoy your new UnitIcons!

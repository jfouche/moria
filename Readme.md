# Design

## Textures :  https://itch.io/
- https://kalponic-studio.itch.io/stylized-wall-textures

## 3D models : https://poly.pizza/
- swat : https://poly.pizza/m/Btfn3G5Xv4
- life potion : https://poly.pizza/m/5DvOJDIgDYu
- slime : https://poly.pizza/m/SW5h0gbCtq

## Audio : https://itch.io/
- Music : https://leohpaz.itch.io/minifantasy-dungeon-sfx-pack

# External lib
- Copy of bevy_gltf_collider (https://github.com/Defernus/bevy_gltf_collider) in order to manage **my** bevy_rapier3d version

# Debug
You can set RUST_LOG env var to what you want

ex : PowerShell
```powershell
$Env:RUST_LOG = "wgpu=error,naga=warn,moria=debug"
```

# Profiling
```shell
cargo run --release --features "bevy/trace_chrome bevy/trace"
```
Open the `trace-*.json` file with https://ui.perfetto.dev/

# dynamic linking
```shell
cargo run --features bevy/dynamic_linking
```

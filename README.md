# Fractals
### Rendered using [wgpu](https://wgpu.rs/) (Rust implementation of the [WebGPU](https://www.w3.org/TR/webgpu/) API)

<p align="middle">
  <img src=".github/assets/koch_snowflake.png" width="50%"/> 
  <br>
  5th iteration of the <a href="https://en.wikipedia.org/wiki/Koch_snowflake">Koch snowflake</a> 
</p>

## Run it yourself

Using [Nix Flakes](https://nixos.wiki/wiki/flakes) (probably only works on Wayland):
```sh
nix run github:julius-boettger/fractals
```

Using [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html):
```sh
git clone https://github.com/julius-boettger/fractals
cd fractals
cargo run
```

# Fractals
### Rendered using [wgpu](https://wgpu.rs/) (Rust implementation of the [WebGPU](https://www.w3.org/TR/webgpu/) API)

<table>
  <tr>
    <td><img src=".github/assets/canopy.png"/></th>
    <td><img src=".github/assets/sierpinski_triangle.png"/></th>
    <td><img src=".github/assets/koch_snowflake.png"/></th>
  </tr>
  <tr>
    <td><a href="https://en.wikipedia.org/wiki/Fractal_canopy">Canopy</a></td>
    <td><a href="https://en.wikipedia.org/wiki/Sierpi%C5%84ski_triangle">Sierpinski triangle</a></td>
    <td><a href="https://en.wikipedia.org/wiki/Koch_snowflake">Koch snowflake</a></td>
  </tr>
  <tr>
    <td>13th iteration, 0.2π left angle, 0.35π right angle</td>
    <td>7th iteration</td>
    <td>6th iteration</td>
  </tr>
</table>

### Controls

When the fractal window is focused, you can interact with it using the following controls:

- `←`/`→`: Cycle through different fractals
- `↑`/`↓`: Increase/decrease fractal iteration
  - ⚠️ Careful: Memory usage increases exponentially with every iteration increase. When you eventually run out of memory, your operating system will (hopefully) attempt to prevent itself from crashing by killing this process.
- Only when viewing [Canopy](https://en.wikipedia.org/wiki/Fractal_canopy):
  - `F`/`D`: Increase/decrease left angle
  - `J`/`K`: Increase/decrease right angle
- `F11`: Toggle fullscreen
- `SPACE`: Start/stop animation

### Command Line Arguments

Run `fractals --help` or `fractals [COMMAND] --help` to see available options. You can...

- Run a CPU/memory benchmark
  - Adjust the fractal type
  - Adjust the fractal iteration

```
> fractals --help
Rendering fractals with wgpu

Usage: fractals [COMMAND]

Commands:
  bench  Run CPU/memory benchmark by computing the triangles necessary to represent a given fractal iteration (without rendering it)
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

```
> fractals bench --help
Run CPU/memory benchmark by computing the triangles necessary to represent a given fractal iteration (without rendering it)

Usage: fractals bench [OPTIONS]

Options:
  -t, --type <TYPE>            Type of fractal to use [default: koch-snowflake] [possible values: canopy, koch-snowflake, sierpinski-triangle]
  -i, --iteration <ITERATION>  Iteration to compute, 1 meaning the initial state. Be careful when increasing this, you will eventually run out of memory [default: 10]
  -h, --help                   Print help
```

# Installation

## Download and run a prebuilt binary...

...from the [latest release](https://github.com/julius-boettger/fractals/releases/latest) (if available for your platform)

## Build and run from source

### Using [Nix Flakes](https://wiki.nixos.org/wiki/Flakes)
```sh
# option 1: fully automatic
nix run github:julius-boettger/fractals
# option 2: fetch source, build, run
git clone https://github.com/julius-boettger/fractals
cd fractals
nix build
./result/bin/fractals
```

### Using [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
```sh
# fetch the source
git clone https://github.com/julius-boettger/fractals
cd fractals
# build
cargo build --release
# run
./target/release/fractals[.exe]
```

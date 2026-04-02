# Cheerio

Mario-inspired endless runner built with Rust + Macroquad, compiled to WASM for the browser.

**[Play Now](https://prdpx7.github.io/Cheerio/)**

## Controls

**Desktop:** Space / Up Arrow to jump. X / Shift to throw fireball (Fire state). Escape to pause.

**Mobile:** Tap left half of screen to jump. Tap right half to fireball. Rotate to landscape.

## Gameplay

Auto-running platformer with four cycling zones: Grassland, Underground, Sky, Castle. Each zone has unique enemies, terrain, and visuals. Difficulty scales every full cycle.

Stomp enemies from above, collect coins, hit ? blocks for power-ups (Mushroom, Fire Flower), dodge gaps and hazards.

## Building

```bash
# native
cd cheerio && cargo run

# wasm
rustup target add wasm32-unknown-unknown
cargo build --manifest-path cheerio/Cargo.toml --target wasm32-unknown-unknown --release
```

## Tech Stack

Rust, Macroquad 0.4, WebAssembly, GitHub Pages

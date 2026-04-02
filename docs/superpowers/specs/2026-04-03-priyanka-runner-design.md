# Priyanka Runner -- Design Spec

Mario-inspired endless runner built with Rust + Macroquad, compiled to WASM for the browser. A gift -- polish and fun factor are the top priorities.

## Gameplay

The player character auto-runs rightward at a constant (but increasing) speed. The player's only input is jumping: tap for a short hop, hold for a full-height jump (variable jump height, matching Mario's physics). The game ends when the player takes damage while in "small" state, or falls into a pit/lava.

### Player States

Three power states, identical to Super Mario Bros:

| State | How to get | Effect | On damage |
|-|-|-|-|
| Small | Default start state | Single tile height | Death |
| Super | Collect Mushroom | Double height, break bricks | Shrink to Small |
| Fire | Collect Fire Flower (while Super) | Throw fireballs with a second input | Shrink to Small |

**Star Power:** Temporary invincibility (~10s). Rainbow palette cycle on the character sprite. Defeats enemies on contact. Chiptune star theme overrides zone BGM.

### Controls

- **Desktop:** Space / Up Arrow to jump. X / Shift to throw fireball (Fire state). Escape to pause.
- **Mobile/Touch:** Tap left half of screen to jump. Tap right half to fireball. Tap pause icon (top-right) to pause.

### Stomp Mechanics

Landing on an enemy from above defeats it and gives a small upward bounce. Chain stomps without touching the ground multiply points: 200, 400, 800, 1600, 8000 (1-UP equivalent bonus).

## Zones

Four zones cycle endlessly: Grassland > Underground > Sky > Castle. Each zone lasts ~30 seconds at base speed. After a full cycle, difficulty increases (faster scroll speed, denser obstacles, more aggressive enemy patterns). Zone transitions use a brief visual wipe (screen slides left) with a crossfade between BGM tracks.

### Zone 1: Grassland

**Visual:** Bright blue sky, green rolling hills (parallax background layers), brown ground with orange top stripe. White fluffy clouds. Green pipes. Brick blocks and ? blocks floating above.

**Terrain:** Flat ground with occasional gaps (1-3 tiles wide). Pipes of varying height. Block clusters at jump height with coins above them.

**Enemies:**
- **Goomba** -- walks forward at constant speed. Defeated by stomp. Dies to fireball.
- **Koopa Troopa** -- walks forward, retreats into shell when stomped. Shell slides forward and defeats other enemies.
- **Piranha Plant** -- emerges from pipes periodically. Cannot be stomped. Defeated by fireball.

**Collectibles:** Coins floating in arcs and lines. ? blocks contain coins, mushrooms, or fire flowers. Brick blocks (breakable when Super).

### Zone 2: Underground

**Visual:** Dark background (#1a1a2e), brown stone ground, stalactites hanging from ceiling. Dim lighting with occasional torch glow effects. No sky visible.

**Terrain:** Tighter vertical space. More platforms at different heights. Lava pits (instant death) instead of empty gaps. Narrow corridors.

**Enemies:**
- **Buzzy Beetle** -- walks forward like Goomba but immune to fireballs. Shell-kick on stomp.
- **Bullet Bill** -- fires horizontally from launchers placed in the terrain. Travels in a straight line. Defeated by stomp.

**Collectibles:** Coin clusters in hard-to-reach ceiling areas. More power-ups to offset difficulty.

### Zone 3: Sky

**Visual:** Light blue sky fading to warm peach at horizon. Fluffy cloud platforms. Floating mushroom platforms. Bright, airy feel.

**Terrain:** Disconnected floating platforms. Large gaps between them. Moving cloud platforms that drift horizontally. Some platforms are small (1-2 tiles).

**Enemies:**
- **Lakitu** -- flies above on a cloud, throws Spinies downward at intervals. Cannot be reached by normal jump (too high). Defeated by fireball.
- **Paratroopa** -- winged Koopa that bounces in an arc pattern. Stomp removes wings (becomes regular Koopa).

**Collectibles:** Coin heavens (dense coin clusters in the sky). Star power more common here as a reward for the difficulty.

### Zone 4: Castle

**Visual:** Dark stone walls (#2d1b00), red/orange lava below, stone brick textures. Bowser banners on walls. Ominous red glow.

**Terrain:** Stone platforms over lava. Moving platforms (horizontal and vertical). Narrow bridges.

**Enemies/Hazards:**
- **Fire Bar** -- rotating chain of fireballs anchored to a block. Various rotation speeds and lengths. Cannot be defeated.
- **Thwomp** -- stone block that slams down when the player approaches, then slowly rises. Must be timed to run under.
- **Bowser Flames** -- horizontal fire streams at varying heights. Periodic pattern. Must be jumped over.

**Collectibles:** Fewer coins, more strategically placed. Power-ups rare but critical.

## Difficulty Progression

Each full cycle (all 4 zones) increments the cycle counter. Difficulty scales:

| Parameter | Cycle 1 | Cycle 2 | Cycle 3 | Cycle 4+ |
|-|-|-|-|-|
| Scroll speed | 1.0x | 1.15x | 1.3x | 1.4x (cap) |
| Obstacle density | Base | +20% | +35% | +50% (cap) |
| Enemy speed | 1.0x | 1.1x | 1.2x | 1.3x (cap) |
| Gap width range | 1-2 tiles | 1-3 tiles | 2-3 tiles | 2-4 tiles |
| Power-up frequency | Normal | Normal | -20% | -30% |

Speed and density cap at cycle 4 to keep the game challenging but not impossible.

## Scoring

- **Distance:** 1 point per unit traveled, multiplied by cycle number
- **Coins:** 100 points each
- **Enemy stomp:** 200 base, chain multiplier (200, 400, 800, 1600, 8000)
- **Power-up collection:** 500 points
- **High score:** Persisted to browser `localStorage` under key `priyanka_runner_highscore`

## Screens

### Title Screen

- Game title "PRIYANKA RUNNER" in pixel-art style lettering, centered
- Subtitle "An Endless Adventure" below
- Mario-style animated background (scrolling grassland scene, clouds moving)
- "PRESS SPACE TO START" blinking text (or "TAP TO START" on mobile)
- High score displayed at top: "HI-SCORE: 8400"
- Small character sprite running in place at the bottom

### Gameplay HUD

Top of screen, Mario-style layout:
- Left: "SCORE" label + current score
- Center-left: Coin icon + "x 12" coin count
- Center-right: Current zone name
- Right: "HI-SCORE" label + high score

### Loading Screen

- Simple progress bar while assets load
- "LOADING..." text with animated dots
- Transitions to Title Screen when all assets are ready

### Pause Screen

- Semi-transparent dark overlay
- "PAUSED" text centered
- "PRESS ESC TO RESUME" (or "TAP TO RESUME")

### Game Over Screen

- "GAME OVER" in large text, drops in with a bounce animation
- Final score (large, centered)
- Stats: distance traveled, coins collected, zones reached, enemies stomped
- "NEW HIGH SCORE!" flashing if applicable
- "PRESS SPACE TO PLAY AGAIN" (or "TAP TO PLAY AGAIN")

## Audio

### Background Music

4 chiptune tracks, one per zone. Each loops seamlessly. Zone transitions crossfade over ~1 second.

| Zone | Music Style | Tempo |
|-|-|-|
| Grassland | Upbeat, bouncy, major key (SMB overworld vibe) | 140 BPM |
| Underground | Mysterious, minor key, echoing (SMB underground) | 120 BPM |
| Sky | Light, ethereal, high-pitched melody | 150 BPM |
| Castle | Tense, driving, ominous | 160 BPM |

Star power overrides with a fast, triumphant jingle for its duration.

### Sound Effects

| Event | Sound |
|-|-|
| Jump | Short rising tone |
| Coin | Classic two-tone "ding" |
| Stomp | Quick thud + pop |
| Power-up | Rising arpeggio |
| Fireball | Short "pew" |
| Pipe (zone transition) | Descending tone |
| Death | Descending melody, pause |
| 1-UP / chain bonus | Rising jingle |

Audio will use free chiptune SFX packs or be generated with a simple synth. Macroquad's `audio` module handles playback.

## Technical Architecture

### Project Structure

```
priyanka-runner/
  Cargo.toml
  src/
    main.rs           -- entry point, game loop, state machine
    player.rs         -- player struct, physics, animation states
    world.rs          -- world generator, chunk management
    zone.rs           -- zone definitions, terrain/enemy rules
    enemy.rs          -- enemy types, behaviors, collision
    collectible.rs    -- coins, power-ups, ? blocks
    renderer.rs       -- drawing functions, parallax, sprites, HUD
    audio.rs          -- BGM management, SFX playback
    score.rs          -- scoring, high score persistence
    screens.rs        -- title, game over screens
    constants.rs      -- game tuning values (speeds, sizes, timings)
  assets/
    sprites/          -- sprite sheets (PNG)
    audio/            -- chiptune tracks and SFX (OGG)
  web/
    index.html        -- minimal HTML loader
  build.sh            -- cargo build + wasm-bindgen + copy to dist/
  dist/               -- build output (HTML + WASM + JS + assets)
```

### Game State Machine

```
Title --> Playing --> GameOver --> Title
                 \--> Paused --> Playing
```

`main.rs` runs a match on the current state each frame, delegating to the appropriate update/render functions.

### World Generation

The world is divided into **chunks** (screen-width segments). The generator maintains a buffer of 3 chunks ahead of the player's position. As the player passes a chunk, it's recycled and a new one is generated at the front.

Each chunk is generated according to the current zone's rules:
- A **terrain template** defines ground height, gaps, and platform positions
- An **enemy spawner** places enemies based on density parameters
- A **collectible spawner** places coins and power-ups

Templates are hand-authored per zone (10-15 per zone) and selected randomly with constraints (no two identical templates in a row, difficulty-appropriate selection).

### Physics

- Gravity: constant downward acceleration (~980 units/s^2, tuned to feel like Mario)
- Jump: initial upward velocity set on press. If button released early, velocity is clamped to a lower max (variable jump height).
- Player horizontal position is fixed on screen (~20% from left edge). The world scrolls left.
- Collision detection: AABB (axis-aligned bounding box) for all entities. Player hitbox is slightly smaller than sprite for forgiveness.

### Rendering

- **Parallax backgrounds:** 3 layers per zone scrolling at different speeds (0.2x, 0.5x, 1.0x). Drawn as tiling textures.
- **Sprites:** Loaded from sprite sheets. Player has animation frames for: run cycle (3 frames), jump (1), fall (1), death (1), fire throw (1). Enemies have 2-frame walk cycles.
- **Resolution:** 480x270 internal resolution, scaled to fill browser window maintaining 16:9 aspect ratio. Pixel-perfect rendering (nearest-neighbor scaling) for crisp retro look with modern 2D art.
- **Draw order:** Background layers > terrain > collectibles > enemies > player > HUD

### Browser Integration

- **wasm-bindgen** for JS interop (localStorage access for high scores)
- **Build pipeline:** `cargo build --target wasm32-unknown-unknown --release` + `wasm-bindgen` to generate JS glue
- **HTML loader:** Minimal `index.html` that loads the WASM module and sets up the canvas
- **Deployment:** Static files, deployable to GitHub Pages, Netlify, or any static host

### Asset Pipeline

- Sprites: pixel art at 1x resolution, packed into sprite sheets
- Audio: OGG format for browser compatibility. Loaded asynchronously at startup with a loading screen.
- All assets loaded before gameplay begins (no streaming needed for this scale)

## Mobile Support

- Touch input: left half = jump, right half = fireball
- Responsive canvas scaling to fill viewport
- `<meta name="viewport">` for proper mobile sizing
- No other mobile-specific changes needed -- Macroquad handles touch events natively

# Kikuyu Cultural Game - Development Log

## Project Overview
Building a 2D game that combines Kikuyu culture with modern game mechanics using Rust and Bevy engine.

## Current Status: Basic Movement & Enemy System ✅

### What Works Now
- ✅ Player (blue sprite) moves left/right with arrow keys
- ✅ Enemies (red sprites) spawn every 2 seconds
- ✅ Enemies move in random directions
- ✅ Basic sprite rendering with custom colors and sizes

### Technical Stack
- **Engine**: Bevy 0.12
- **Language**: Rust
- **Graphics**: 2D sprites
- **Platform**: Cross-platform (Web via WASM, Desktop, Mobile)

## Code Structure

### Components
```rust
Player          // Marks player entity
Enemy           // Marks enemy entities  
Speed(f32)      // Movement speed in pixels/second
Direction(Vec2) // Movement direction (normalized vector)
```

### Systems (Game Logic)
```rust
setup()         // Initialize camera and player
player_movement() // Handle arrow key input
enemy_spawner()   // Spawn enemies every 2 seconds
move_enemies()    // Move enemies in random directions
```

### Key Concepts Learned
1. **ECS Architecture**: Entities have Components, Systems operate on them
2. **Transform**: Position, rotation, scale of entities
3. **Time.delta()**: Frame-independent movement
4. **Query**: Get entities with specific components
5. **Random Generation**: Using `rand` crate for random directions

## Assets Structure
```
assets/
└── sprites/
    └── game.png    // Placeholder sprite (currently using solid colors)
```

## Dependencies Added
```toml
[dependencies]
bevy = "0.12"
rand = "0.8"  # For random enemy movement
```

## Next Development Steps

### Phase 1: Core Mechanics
- [ ] Add collision detection between player and enemies
- [ ] Implement game over/restart system  
- [ ] Add score/health system
- [ ] Enemy cleanup (remove off-screen enemies)

### Phase 2: Cultural Elements
- [ ] Replace sprites with Kikuyu-themed artwork
- [ ] Add traditional Kikuyu music/sounds
- [ ] Implement age-set progression system
- [ ] Add wisdom points mechanic

### Phase 3: Game Polish
- [ ] Particle effects
- [ ] UI system (score, health display)
- [ ] Multiple enemy types
- [ ] Power-ups based on traditional items

### Phase 4: Cultural Depth
- [ ] Storytelling elements (proverbs, folktales)
- [ ] Traditional challenge mini-games
- [ ] Historical figure appearances
- [ ] Language learning integration

## Bevy Learning Notes

### Essential Patterns
```rust
// Spawning entities
commands.spawn((Component1, Component2, Transform::default()));

// Querying entities
Query<&Component, With<FilterComponent>>
Query<(&mut Component1, &Component2), Without<FilterComponent>>

// Resources (global data)
Res<Time>, Res<AssetServer>, Res<ButtonInput<KeyCode>>

// System ordering
.add_systems(Update, (system1, system2, system3))
```

### Transform Coordinates
- Origin (0,0) = center of screen
- X: positive = right, negative = left  
- Y: positive = up, negative = down
- Z: higher = closer to camera (layering)

### Common Gotchas
1. Always add `Transform` component to sprites
2. Use `time.delta().as_secs_f32()` for smooth movement
3. Normalize direction vectors for consistent speed
4. Remember Bevy uses center-based coordinates

## Cultural Integration Ideas

### Kikuyu Elements to Include
1. **Ngai (God)** - Power-ups from Mount Kenya direction
2. **Mugumo Tree** - Safe zones or wisdom dispensers
3. **Age-Sets (Riika)** - Level progression system
4. **Traditional Proverbs** - Loading screen wisdom
5. **Gikuyu & Mumbi Legend** - Origin story integration
6. **Traditional Dances** - Rhythm-based mini-games
7. **Farming Seasons** - Time-based mechanics
8. **Council of Elders** - Decision-making gameplay

### Authentic Representation Goals
- Consult with Kikuyu elders for cultural accuracy
- Use traditional color symbolism
- Incorporate real proverbs and their meanings
- Respect sacred vs. shareable cultural elements
- Make it educational without being preachy

## Performance Notes
- Current frame rate: Smooth 60fps
- Entity count: Low (1 player + enemies spawning)
- Memory usage: Minimal
- Target: Mobile-friendly performance

## Build Commands
```bash
# Development (fast compile)
cargo run

# Release build
cargo build --release

# Web build (future)
cargo build --target wasm32-unknown-unknown --release
```

## Vision Statement
Create an authentic, engaging game that introduces players to Kikuyu culture through interactive storytelling and traditional wisdom, while being genuinely fun to play.

---

**Current Focus**: Get basic game mechanics solid before adding cultural elements. Foundation first, culture second, polish third.
# Pac-Man

A **Pac-Man** mini game built with **Rust + Bevy**.  
Implements clear modular layering, AI behaviors, collision systems, and animation logic using the ECS architecture — serving as a learning project for Rust game development.

## Overview

- **Engine**: Bevy 0.17.2
- **Language**: Rust 2024 
- **Rendering**: 2D Sprites + Orthographic Camera  
- **Goal**: Recreate the classic Pac-Man gameplay using a modern ECS-based architecture  

## Quick Start

```bash
# Clone the project
git clone https://github.com/iAeternus/pac-man.git
cd pac-man

# Build and run
cargo run
```

## Development Roadmap

### Core Systems

* [ ] Game State Machine (Menu / Playing / Paused / Game Over)
* [ ] Map System (Static map loading and tile-based collision)
* [ ] Player System (Keyboard input, movement, pellet eating)
* [ ] Ghost System (Chasing, fleeing, patrolling behaviors)
* [ ] Item System (Normal and power pellets)
* [ ] Collision System (Player–Ghost / Player–Item detection)
* [ ] Scoring System (Real-time score tracking and display)
* [ ] Input System (↑↓←→ / Space / Enter controls)

### Animation & Visuals

* [ ] Animation System (Pac-Man mouth movement, ghost flashing)
* [ ] Particle Effects (Eating and death feedback)
* [ ] Multi-Level Support (JSON map loading and level switching)

### AI & Logic Extensions

* [ ] AI Pathfinding (A* algorithm and distinct ghost personalities)
* [ ] Difficulty Scaling and Speed Curves
* [ ] Ghost Respawn and Home Return Logic

### Audio & Debugging

* [ ] Audio Support (Eating and death sounds)
* [ ] Debug Tools (Bevy Inspector integration)

## License

This project is licensed under the [MIT License](LICENSE).

**Author**: [@iAeternus](https://github.com/iAeternus)


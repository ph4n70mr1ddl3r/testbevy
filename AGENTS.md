# Agent Guide for testbevy Project

## Essential Commands
```bash
# Build project
cargo build

# Run tests (no tests currently)
# cargo test

# Format code (using rustfmt)
cargo fmt

# Lint check (clippy)
cargo clippy

# Run application
cargo run
```

## Project Structure
- `src/main.rs`: Application entry point
- `src/game.rs`: Core game logic
- `src/poker_logic.rs`: Poker rules/hand evaluation
- `src/ai.rs`: AI decision system
- `src/ui.rs`: User interface components
- `src/animation.rs`: Animation systems
- `src/constants.rs`: Game constants

## Coding Patterns
- Uses Bevy ECS architecture:
  - Systems added via Plugins
  - Components store data
  - Resources for global state
- Poker logic in `poker_logic.rs` handles card evaluation
- AI uses game theory optimal strategies

## Style Guidelines
- Follow rustfmt config (100 line width)
- Clippy linter enforced (see clippy.toml)
- Module organization mirrors feature separation
- Tests currently missing (future work)

## Gotchas
- Game state managed through Bevy Resources
- UI updates via event system
- Animation system uses Bevy's state transitions
# Three Text Sphere

A Rust/WebAssembly application that renders 3D text orbiting around a sphere, using Yew and three.js. Inspired by the Godot implementation in [godot-text-sphere](../godot-text-sphere).

![Screenshot](images/screenshot.png?ts=1734032357295)

## Features

- 3D text characters orbiting a central sphere
- Smooth animation at 60fps
- Pure Rust implementation (minimal JavaScript)
- Static file deployment

## Quick Start

### Prerequisites

- Rust toolchain with wasm32 target
- Trunk (for building)
- basic-http-server (for testing)

```bash
# Install dependencies
rustup target add wasm32-unknown-unknown
cargo install trunk
cargo install basic-http-server
```

### Build

```bash
trunk build --release
```

### Run

```bash
basic-http-server -a 0.0.0.0:1417 dist/
```

Open http://localhost:1417 in your browser.

## Documentation

### Project Documentation

- [Architecture](docs/architecture.md) - System design and technology stack
- [PRD](docs/prd.md) - Product requirements document
- [Design](docs/design.md) - Technical design details
- [Plan](docs/plan.md) - Implementation plan and phases
- [Status](docs/status.md) - Current project status

### Development Guidelines

- [AI Agent Instructions](docs/ai_agent_instructions.md) - Guidelines for AI coding agents
- [Process](docs/process.md) - Development workflow and pre-commit process
- [Tools](docs/tools.md) - Recommended development tools

## Project Structure

```
three-text-sphere/
  Cargo.toml          # Rust dependencies
  Trunk.toml          # Build configuration
  index.html          # HTML shell with three.js
  LICENSE             # MIT License
  COPYRIGHT           # Copyright notice
  src/
    lib.rs            # WASM entry point and three.js bindings
  dist/               # Build output (generated)
  docs/               # Documentation
  images/             # Screenshots
```

## Configuration

Default parameters (compile-time):

| Parameter | Value | Description |
|-----------|-------|-------------|
| text | "DEMO" | Text to display |
| radius | 8.0 | Orbit distance |
| rotation_speed | 0.4 | rad/s |
| sphere_radius | 3.0 | Central sphere size |

## Reference

Based on the visual design of [godot-text-sphere](../godot-text-sphere), a Godot 4 implementation of the same concept.

## License

MIT - See [LICENSE](LICENSE) for details.

Copyright (c) 2025 Michael A Wright

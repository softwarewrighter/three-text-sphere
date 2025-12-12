# Product Requirements Document (PRD)

## Product Overview

Three Text Sphere is a web application that displays 3D text characters orbiting around a central sphere. It replicates the functionality of the Godot-based `godot-text-sphere` demo as a browser-based experience using Rust/WebAssembly.

## Goals

1. **Rust-First**: Pure Rust implementation with minimal JavaScript
2. **Static Deployment**: Single set of static files, no server-side logic
3. **Visual Parity**: Match the appearance of the Godot reference implementation
4. **Performance**: Smooth 60fps animation in modern browsers

## Target Users

- Developers exploring Rust/WASM for 3D web applications
- Users wanting a portable, browser-based version of the text sphere demo
- Educational reference for three.js integration with Rust

## Functional Requirements

### FR-1: 3D Scene Rendering

- Display a central sphere with configurable color and size
- Render 3D extruded text characters around the sphere
- Proper lighting and camera perspective

### FR-2: Text Positioning

- Evenly distribute characters around a circle
- Each character faces outward (toward the viewer from any angle)
- Configurable orbit radius
- Skip spaces in input text

### FR-3: Animation

- Continuous rotation around the vertical (Y) axis
- Configurable rotation speed
- Frame-rate independent animation using delta time

### FR-4: Configuration

| Parameter | Description | Default |
|-----------|-------------|---------|
| `text` | String to display | "DEMO" |
| `radius` | Orbit distance from center | 8.0 |
| `rotation_speed` | Animation speed (rad/s) | 0.4 |
| `letter_scale` | Size multiplier for letters | 3.0 |
| `sphere_radius` | Central sphere size | 3.0 |
| `sphere_color` | RGB color of sphere | (0.2, 0.4, 0.8) |

### FR-5: Static Serving

- All assets in a single `dist/` directory
- Compatible with `basic-http-server`
- No CORS requirements beyond standard WASM serving

## Non-Functional Requirements

### NFR-1: Performance

- Target 60fps on modern hardware
- Initial load under 5 seconds on broadband
- WASM binary under 1MB compressed

### NFR-2: Compatibility

- Chrome 80+, Firefox 75+, Safari 14+, Edge 80+
- WebGL 2.0 required

### NFR-3: Code Quality

- Pure Rust codebase (no TypeScript, no Python)
- Documented public APIs
- Idiomatic Rust patterns

## Out of Scope

- User interface controls (future enhancement)
- Mobile touch interactions
- Custom font loading
- Multiple text rings
- Persistent configuration

## Success Criteria

1. Text orbits sphere smoothly at 60fps
2. Visual appearance matches Godot reference
3. Builds and runs with standard Rust toolchain
4. Serves correctly from `basic-http-server`

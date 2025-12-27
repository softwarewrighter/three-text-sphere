# Architecture

## Overview

Three Text Sphere is a Rust/WebAssembly application that renders 3D text orbiting around a sphere, inspired by the Godot implementation in `godot-text-sphere`. The application uses Yew for the web framework and three.js for 3D rendering via JavaScript interop.

## Technology Stack

```
+------------------+
|   Browser        |
+------------------+
        |
+------------------+
|   three.js       |  (3D Rendering)
+------------------+
        |
+------------------+
|   JS Bindings    |  (wasm-bindgen, js-sys, web-sys)
+------------------+
        |
+------------------+
|   Yew Framework  |  (Component Model)
+------------------+
        |
+------------------+
|   Rust/WASM      |  (Core Logic)
+------------------+
```

## Key Components

### 1. Rust/WASM Core

- **Yew Application**: Single-page application framework
- **wasm-bindgen**: Rust-to-JavaScript FFI bindings
- **web-sys**: Web API bindings for DOM manipulation
- **js-sys**: JavaScript standard library bindings

### 2. three.js Integration

Three.js is loaded via CDN in the HTML shell. Rust communicates with three.js through:

- **Raw JS bindings**: Using `#[wasm_bindgen]` for three.js objects
- **Inline JavaScript**: Using `js!` macro for complex operations

### 3. Rendering Pipeline

```
Rust Logic          three.js              WebGL
+----------+       +----------+       +----------+
| Position |  -->  | Scene    |  -->  | Canvas   |
| Calc     |       | Objects  |       | Render   |
+----------+       +----------+       +----------+
```

## Module Structure

```
src/
  main.rs           # Entry point, mounts Yew app
  lib.rs            # WASM exports
  app.rs            # Main Yew component
  three/
    mod.rs          # three.js bindings
    scene.rs        # Scene setup and management
    text.rs         # 3D text mesh creation
    sphere.rs       # Central sphere mesh
    animation.rs    # Rotation animation loop
  utils/
    mod.rs          # Utility functions
    math.rs         # Trigonometric calculations
```

## Data Flow

1. **Initialization**: Yew mounts, creates three.js scene
2. **Text Generation**: Letters positioned using polar coordinates
3. **Animation Loop**: `requestAnimationFrame` rotates letter container
4. **Render**: three.js renders to canvas each frame

## Build Artifacts

```
dist/
  index.html        # HTML shell with three.js CDN
  three_text_sphere.js    # Generated JS glue code
  three_text_sphere_bg.wasm  # Compiled WASM binary
```

## Deployment

Static files served by any HTTP server. No backend required.

Testing: `basic-http-server -a 0.0.0.0:1417`

# Design Document

## Visual Design

### Reference: Godot Implementation

The design mirrors the `godot-text-sphere` project:

- **Central sphere**: Blue (RGB 0.2, 0.4, 0.8), radius 3.0 units
- **Letters**: 3D extruded text, HSV color cycling per letter
- **Orbit radius**: 8.0 units from center
- **Rotation**: Clockwise when viewed from above, 0.4 rad/s

### Scene Layout

```
        Camera (0, 4, 20)
            |
            v
      +-----------+
     /   Letters   \
    |   orbiting    |
    |  [  SPHERE  ] |
    |   around      |
     \   center    /
      +-----------+
           |
           v
    DirectionalLight
```

## Technical Design

### three.js Bindings Strategy

Since three.js is a JavaScript library, we use a hybrid approach:

1. **Minimal JS Glue**: Load three.js via CDN in HTML
2. **wasm-bindgen extern blocks**: Define Rust interfaces to three.js
3. **Unsafe JS calls**: Where bindings are impractical, use inline JS

```rust
#[wasm_bindgen]
extern "C" {
    type Scene;

    #[wasm_bindgen(constructor, js_namespace = THREE)]
    fn new() -> Scene;

    #[wasm_bindgen(method)]
    fn add(this: &Scene, object: &Object3D);
}
```

### Text Mesh Creation

Three.js TextGeometry requires a font loader. Design options:

**Option A: TextGeometry with loaded font** (Complex)
- Load font JSON asynchronously
- Create TextGeometry for each letter
- More authentic 3D text appearance

**Option B: Sprite-based text** (Simpler)
- Use canvas to render text to texture
- Apply texture to planes
- Always faces camera (billboarding)

**Option C: ExtrudeGeometry from paths** (Moderate)
- Define letter shapes as paths
- Extrude to 3D
- Full 3D but limited font support

**Selected: Option A** - Provides closest visual match to Godot implementation.

### Position Calculation

Letter positions use parametric circle equations:

```rust
fn calculate_position(index: usize, total: usize, radius: f32) -> (f32, f32, f32) {
    let angle = (index as f32 / total as f32) * std::f32::consts::TAU;
    let x = radius * angle.cos();
    let z = radius * angle.sin();
    (x, 0.0, z)  // Y stays at 0 for horizontal orbit
}
```

### Outward Facing Orientation

Each letter rotates to face away from center:

```rust
fn calculate_rotation(position: (f32, f32, f32)) -> f32 {
    // Angle from center, offset by PI to face outward
    position.2.atan2(position.0) + std::f32::consts::PI
}
```

### Animation System

Using `requestAnimationFrame` via web-sys:

```rust
fn animate(letters_group: &Group, last_time: f64) {
    let current_time = performance_now();
    let delta = (current_time - last_time) / 1000.0;

    letters_group.rotate_y(-ROTATION_SPEED * delta);

    renderer.render(&scene, &camera);

    request_animation_frame(/* callback */);
}
```

### Color Assignment

HSV color cycling for visual variety:

```rust
fn letter_color(index: usize, total: usize) -> (f32, f32, f32) {
    let hue = index as f32 / total as f32;
    hsv_to_rgb(hue, 0.8, 0.9)  // High saturation, high value
}
```

## File Structure

```
three-text-sphere/
  Cargo.toml
  src/
    lib.rs              # WASM entry point
    bindings/
      mod.rs            # three.js type bindings
      scene.rs          # Scene, Camera, Renderer
      geometry.rs       # Geometries and Materials
      mesh.rs           # Mesh and Group
      font.rs           # FontLoader, TextGeometry
    components/
      mod.rs
      sphere.rs         # Central sphere creation
      letters.rs        # Letter mesh creation
    animation.rs        # Animation loop
    config.rs           # Configuration constants
  static/
    index.html          # HTML shell
  dist/                 # Build output (gitignored)
```

## Build Configuration

`Cargo.toml` key dependencies:

```toml
[dependencies]
yew = { version = "0.21", features = ["csr"] }
wasm-bindgen = "0.2"
web-sys = { version = "0.3", features = [...] }
js-sys = "0.3"
gloo = "0.11"

[lib]
crate-type = ["cdylib"]
```

Build with `trunk` or `wasm-pack`:

```bash
trunk build --release
# or
wasm-pack build --target web
```

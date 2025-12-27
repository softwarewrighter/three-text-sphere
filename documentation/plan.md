# Implementation Plan

## Phase 1: Project Setup

### 1.1 Initialize Rust Project

- [x] Create `Cargo.toml` with required dependencies
- [ ] Set up `src/lib.rs` as WASM entry point
- [ ] Configure wasm-bindgen for web target

### 1.2 Static Assets

- [ ] Create `static/index.html` with:
  - three.js CDN link
  - Canvas element
  - WASM loader script

### 1.3 Build System

- [ ] Configure Trunk for building (or wasm-pack)
- [ ] Verify builds produce valid WASM
- [ ] Test with `basic-http-server -a 0.0.0.0:1417`

## Phase 2: three.js Bindings

### 2.1 Core Types

- [ ] Define `Scene` binding
- [ ] Define `PerspectiveCamera` binding
- [ ] Define `WebGLRenderer` binding
- [ ] Define `Mesh` and `Group` bindings

### 2.2 Geometry Types

- [ ] Define `SphereGeometry` binding
- [ ] Define `TextGeometry` binding (if using 3D text)
- [ ] Define `MeshStandardMaterial` binding

### 2.3 Lighting

- [ ] Define `DirectionalLight` binding
- [ ] Define `AmbientLight` binding

### 2.4 Font Loading

- [ ] Define `FontLoader` binding
- [ ] Handle async font loading
- [ ] Bundle a default font or load from CDN

## Phase 3: Scene Construction

### 3.1 Basic Scene

- [ ] Initialize scene, camera, renderer
- [ ] Set up canvas and viewport
- [ ] Add basic lighting

### 3.2 Central Sphere

- [ ] Create sphere geometry (radius 3.0)
- [ ] Apply blue material (0.2, 0.4, 0.8)
- [ ] Add to scene

### 3.3 Letter Generation

- [ ] Parse input text, skip spaces
- [ ] Calculate positions using polar coordinates
- [ ] Create TextGeometry for each letter
- [ ] Apply HSV-cycled colors
- [ ] Orient letters to face outward
- [ ] Add all letters to a Group

## Phase 4: Animation

### 4.1 Animation Loop

- [ ] Set up `requestAnimationFrame` callback
- [ ] Track delta time for frame-rate independence
- [ ] Rotate letters group around Y axis

### 4.2 Render Loop

- [ ] Call `renderer.render()` each frame
- [ ] Handle window resize events

## Phase 5: Configuration

### 5.1 Compile-Time Config

- [ ] Define constants in `config.rs`
- [ ] Document all parameters

### 5.2 Future: Runtime Config (Out of Scope)

- URL parameters
- Interactive UI controls

## Phase 6: Polish and Testing

### 6.1 Visual Polish

- [ ] Match Godot reference appearance
- [ ] Adjust lighting for good visibility
- [ ] Fine-tune camera position

### 6.2 Performance

- [ ] Profile WASM size
- [ ] Ensure 60fps on target browsers
- [ ] Optimize if needed

### 6.3 Documentation

- [x] Architecture document
- [x] PRD document
- [x] Design document
- [x] Plan document
- [x] Status document
- [x] README with links

## Phase 7: Deployment

### 7.1 Build Release

- [ ] Run release build
- [ ] Verify all assets in `dist/`

### 7.2 Test Deployment

- [ ] Serve with `basic-http-server -a 0.0.0.0:1417`
- [ ] Test in multiple browsers
- [ ] Verify no console errors

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| yew | 0.21 | Web framework |
| wasm-bindgen | 0.2 | JS interop |
| web-sys | 0.3 | Web APIs |
| js-sys | 0.3 | JS stdlib |
| gloo | 0.11 | Browser utilities |

## Risks and Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| TextGeometry complexity | High | Fall back to sprite text |
| Font loading async issues | Medium | Pre-bundle font or use simpler text |
| three.js version changes | Low | Pin CDN version |
| WASM size too large | Medium | Feature-flag unused three.js bindings |

## Success Criteria

1. `trunk build` produces valid WASM
2. `basic-http-server` serves without errors
3. Text orbits sphere smoothly
4. Visual match to Godot reference

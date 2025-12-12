use std::cell::RefCell;
use std::f64::consts::PI;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::window;

// Configuration constants (matching Godot reference)
const TEXT_TO_DISPLAY: &str = "DEMO";
const ORBIT_RADIUS: f64 = 8.0;
const ROTATION_SPEED: f64 = 0.4;
const LETTER_SIZE: f64 = 3.0;      // TextGeometry size parameter
const LETTER_DEPTH: f64 = 0.15;    // Extrusion depth (thin)
const SPHERE_RADIUS: f64 = 3.0;
const SPHERE_COLOR: u32 = 0x3366CC; // RGB(0.2, 0.4, 0.8)

// All three.js interop via inline JavaScript for cleaner bindings
#[wasm_bindgen(inline_js = r#"
export function isThreeReady() {
    return window.THREE !== undefined && window.THREE !== null;
}

export function createScene() {
    return new window.THREE.Scene();
}

export function createCamera(fov, aspect, near, far) {
    return new window.THREE.PerspectiveCamera(fov, aspect, near, far);
}

export function createRenderer() {
    return new window.THREE.WebGLRenderer({ antialias: true });
}

export function createGroup() {
    return new window.THREE.Group();
}

export function createSphereGeometry(radius, widthSegs, heightSegs) {
    return new window.THREE.SphereGeometry(radius, widthSegs, heightSegs);
}

export function createTextGeometry(text, font, size, height) {
    // In three.js 0.150+, the extrusion parameter is called 'height' not 'depth'
    return new window.TextGeometry(text, {
        font: font,
        size: size,
        height: height,
        curveSegments: 8,
        bevelEnabled: true,
        bevelThickness: 0.02,
        bevelSize: 0.01,
        bevelOffset: 0,
        bevelSegments: 3
    });
}

export function createMaterial(color) {
    return new window.THREE.MeshStandardMaterial({
        color: color,
        roughness: 0.5,
        metalness: 0.1
    });
}

export function createSphereMaterial(color) {
    return new window.THREE.MeshStandardMaterial({
        color: color,
        roughness: 0.6,
        metalness: 0.0
    });
}

export function createMesh(geometry, material) {
    return new window.THREE.Mesh(geometry, material);
}

export function createDirectionalLight(color, intensity) {
    return new window.THREE.DirectionalLight(color, intensity);
}

export function createAmbientLight(color, intensity) {
    return new window.THREE.AmbientLight(color, intensity);
}

export function setPosition(obj, x, y, z) {
    obj.position.set(x, y, z);
}

export function setRotation(obj, x, y, z) {
    obj.rotation.set(x, y, z);
}

export function setScale(obj, x, y, z) {
    obj.scale.set(x, y, z);
}

export function rotateY(obj, angle) {
    obj.rotateY(angle);
}

export function addToScene(scene, obj) {
    scene.add(obj);
}

export function addToGroup(group, obj) {
    group.add(obj);
}

export function lookAt(camera, x, y, z) {
    camera.lookAt(x, y, z);
}

export function setRendererSize(renderer, width, height) {
    renderer.setSize(width, height);
}

export function setPixelRatio(renderer, ratio) {
    renderer.setPixelRatio(ratio);
}

export function getRendererDomElement(renderer) {
    return renderer.domElement;
}

export function render(renderer, scene, camera) {
    renderer.render(scene, camera);
}

export function setCameraAspect(camera, aspect) {
    camera.aspect = aspect;
    camera.updateProjectionMatrix();
}

export function getLoadedFont() {
    return window.loadedFont || null;
}

export function isFontLoaded() {
    return window.loadedFont !== undefined && window.loadedFont !== null;
}

export function centerGeometry(geometry) {
    geometry.computeBoundingBox();
    const box = geometry.boundingBox;
    // Center on all axes including depth
    const centerX = (box.max.x + box.min.x) / 2;
    const centerY = (box.max.y + box.min.y) / 2;
    const centerZ = (box.max.z + box.min.z) / 2;
    geometry.translate(-centerX, -centerY, -centerZ);
}
"#)]
extern "C" {
    fn isThreeReady() -> bool;
    fn createScene() -> JsValue;
    fn createCamera(fov: f64, aspect: f64, near: f64, far: f64) -> JsValue;
    fn createRenderer() -> JsValue;
    fn createGroup() -> JsValue;
    fn createSphereGeometry(radius: f64, width_segs: u32, height_segs: u32) -> JsValue;
    fn createTextGeometry(text: &str, font: &JsValue, size: f64, depth: f64) -> JsValue;
    fn createMaterial(color: u32) -> JsValue;
    fn createSphereMaterial(color: u32) -> JsValue;
    fn createMesh(geometry: &JsValue, material: &JsValue) -> JsValue;
    fn createDirectionalLight(color: u32, intensity: f64) -> JsValue;
    fn createAmbientLight(color: u32, intensity: f64) -> JsValue;
    fn setPosition(obj: &JsValue, x: f64, y: f64, z: f64);
    fn setRotation(obj: &JsValue, x: f64, y: f64, z: f64);
    fn setScale(obj: &JsValue, x: f64, y: f64, z: f64);
    fn rotateY(obj: &JsValue, angle: f64);
    fn addToScene(scene: &JsValue, obj: &JsValue);
    fn addToGroup(group: &JsValue, obj: &JsValue);
    fn lookAt(camera: &JsValue, x: f64, y: f64, z: f64);
    fn setRendererSize(renderer: &JsValue, width: f64, height: f64);
    fn setPixelRatio(renderer: &JsValue, ratio: f64);
    fn getRendererDomElement(renderer: &JsValue) -> web_sys::HtmlCanvasElement;
    fn render(renderer: &JsValue, scene: &JsValue, camera: &JsValue);
    fn setCameraAspect(camera: &JsValue, aspect: f64);
    fn getLoadedFont() -> JsValue;
    fn isFontLoaded() -> bool;
    fn centerGeometry(geometry: &JsValue);
}

fn hsv_to_rgb(h: f64, s: f64, v: f64) -> u32 {
    let c = v * s;
    let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = match (h * 6.0) as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    let r = ((r + m) * 255.0) as u32;
    let g = ((g + m) * 255.0) as u32;
    let b = ((b + m) * 255.0) as u32;

    (r << 16) | (g << 8) | b
}

struct TextSphere {
    scene: JsValue,
    camera: JsValue,
    renderer: JsValue,
    letters_group: JsValue,
    letters_added: Rc<RefCell<bool>>,
}

impl TextSphere {
    fn new() -> Result<Self, JsValue> {
        let win = window().ok_or("no window")?;
        let document = win.document().ok_or("no document")?;
        let container = document
            .get_element_by_id("canvas-container")
            .ok_or("no canvas-container")?;

        let width = win.inner_width()?.as_f64().unwrap_or(800.0);
        let height = win.inner_height()?.as_f64().unwrap_or(600.0);

        // Create scene
        let scene = createScene();

        // Create camera (matching Godot: positioned at (0, 4, 20))
        let camera = createCamera(75.0, width / height, 0.1, 1000.0);
        setPosition(&camera, 0.0, 4.0, 20.0);
        lookAt(&camera, 0.0, 0.0, 0.0);

        // Create renderer
        let renderer = createRenderer();
        setRendererSize(&renderer, width, height);
        setPixelRatio(&renderer, win.device_pixel_ratio());

        let canvas = getRendererDomElement(&renderer);
        container.append_child(&canvas)?;

        // Add lighting
        let dir_light = createDirectionalLight(0xFFFFFF, 1.0);
        setPosition(&dir_light, 0.0, 6.0, 10.0);
        addToScene(&scene, &dir_light);

        let ambient = createAmbientLight(0x404040, 0.5);
        addToScene(&scene, &ambient);

        // Create central sphere
        let sphere_geo = createSphereGeometry(SPHERE_RADIUS, 64, 32);
        let sphere_mat = createSphereMaterial(SPHERE_COLOR);
        let sphere = createMesh(&sphere_geo, &sphere_mat);
        addToScene(&scene, &sphere);

        // Create letters group (will be populated when font loads)
        let letters_group = createGroup();
        addToScene(&scene, &letters_group);

        Ok(Self {
            scene,
            camera,
            renderer,
            letters_group,
            letters_added: Rc::new(RefCell::new(false)),
        })
    }

    fn try_add_letters(&self) {
        if *self.letters_added.borrow() {
            return;
        }

        if !isFontLoaded() {
            return;
        }

        let font = getLoadedFont();
        if font.is_null() || font.is_undefined() {
            return;
        }

        // Get non-space characters
        let chars: Vec<char> = TEXT_TO_DISPLAY.chars().filter(|c| *c != ' ').collect();
        let char_count = chars.len();

        if char_count > 0 {
            let angle_step = 2.0 * PI / char_count as f64;

            for (i, ch) in chars.iter().enumerate() {
                // Calculate position on circle (negative angle for clockwise when viewed from +Y)
                let angle = -(i as f64) * angle_step;
                let x = ORBIT_RADIUS * angle.cos();
                let z = ORBIT_RADIUS * angle.sin();

                // HSV color cycling
                let hue = i as f64 / char_count as f64;
                let color = hsv_to_rgb(hue, 0.8, 0.9);

                // Create TextGeometry for the letter
                let letter_str = ch.to_string();
                let geo = createTextGeometry(&letter_str, &font, LETTER_SIZE, LETTER_DEPTH);
                centerGeometry(&geo);

                let mat = createMaterial(color);
                let mesh = createMesh(&geo, &mat);

                // Position the letter
                setPosition(&mesh, x, 0.0, z);

                // Rotate to face outward
                let face_angle = x.atan2(z);
                setRotation(&mesh, 0.0, face_angle, 0.0);

                addToGroup(&self.letters_group, &mesh);
            }
        }

        *self.letters_added.borrow_mut() = true;
        log::info!("Letters added to scene");
    }

    fn animate(&self, delta: f64) {
        // Try to add letters if font just loaded
        self.try_add_letters();

        // Rotate letters group around Y axis (negative for clockwise from above)
        rotateY(&self.letters_group, -ROTATION_SPEED * delta);
        render(&self.renderer, &self.scene, &self.camera);
    }

    fn resize(&self, width: f64, height: f64) {
        setCameraAspect(&self.camera, width / height);
        setRendererSize(&self.renderer, width, height);
    }
}

// Application state
struct App {
    text_sphere: Option<TextSphere>,
    initialized: bool,
}

impl App {
    fn new() -> Self {
        Self {
            text_sphere: None,
            initialized: false,
        }
    }

    fn try_init(&mut self) -> Result<bool, JsValue> {
        if self.initialized {
            return Ok(true);
        }

        if !isThreeReady() {
            return Ok(false);
        }

        log::info!("THREE.js is ready, initializing scene");
        self.text_sphere = Some(TextSphere::new()?);
        self.initialized = true;
        Ok(true)
    }

    fn animate(&self, delta: f64) {
        if let Some(ref ts) = self.text_sphere {
            ts.animate(delta);
        }
    }

    fn resize(&self, width: f64, height: f64) {
        if let Some(ref ts) = self.text_sphere {
            ts.resize(width, height);
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("WASM application starting, waiting for THREE.js...");

    let app = Rc::new(RefCell::new(App::new()));

    // Set up animation loop that also handles initialization
    let app_clone = app.clone();
    let last_time = Rc::new(RefCell::new(0.0));

    let f: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();

    let win = window().ok_or("no window")?;

    *g.borrow_mut() = Some(Closure::new(move |time: f64| {
        let mut last = last_time.borrow_mut();
        let delta = if *last == 0.0 {
            0.016
        } else {
            (time - *last) / 1000.0
        };
        *last = time;

        // Try to initialize if not done yet
        {
            let mut app_ref = app_clone.borrow_mut();
            if let Err(e) = app_ref.try_init() {
                log::error!("Failed to initialize: {:?}", e);
            }
        }

        // Animate if initialized
        app_clone.borrow().animate(delta);

        if let Some(w) = window() {
            let _ = w.request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref());
        }
    }));

    // Start animation loop
    win.request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())?;

    // Handle window resize
    let app_resize = app.clone();
    let resize_closure = Closure::<dyn FnMut()>::new(move || {
        if let Some(w) = window() {
            let width = w.inner_width().ok().and_then(|v| v.as_f64()).unwrap_or(800.0);
            let height = w.inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(600.0);
            app_resize.borrow().resize(width, height);
        }
    });

    win.add_event_listener_with_callback("resize", resize_closure.as_ref().unchecked_ref())?;
    resize_closure.forget();

    Ok(())
}

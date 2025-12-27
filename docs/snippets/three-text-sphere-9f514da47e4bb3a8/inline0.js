
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

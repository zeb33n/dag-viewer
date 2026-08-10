mod drawing;
mod js;
use drawing::draw;
use scene::*;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    LazyLock, Mutex,
};
mod data_types;
use data_types::*;
mod colours;
mod model;
mod parser;
mod scene;

static GRAPHVIZ_LAYOUT: AtomicBool = AtomicBool::new(true);

pub fn is_graphviz_layout() -> bool {
    GRAPHVIZ_LAYOUT.load(Ordering::Relaxed)
}

static SCENE: LazyLock<Mutex<Scene>> = LazyLock::new(|| Mutex::new(Scene::new_default()));

#[unsafe(no_mangle)]
// force the compiler to use C ABI so WebAssemply module interface is stable
pub extern "C" fn dag_viewer_init(w: f32, h: f32, ptr: *const u8, len: usize) -> () {
    let bytes: &[u8] = unsafe { std::slice::from_raw_parts(ptr, len) };
    let dot = std::str::from_utf8(bytes).unwrap();
    let mut scene = SCENE.lock().unwrap();
    let mut s = Scene::new(w, h, dot);

    if !is_graphviz_layout() {
        s.layout();
    }

    *scene = s;
    draw(&*scene);
}

#[unsafe(no_mangle)]
pub extern "C" fn dag_viewer_drag(dx: f32, dy: f32) -> () {
    let mut scene = SCENE.lock().unwrap();
    scene.camera.pos.x += dx / scene.camera.zoom;
    scene.camera.pos.y += dy / scene.camera.zoom;
    draw(&*scene);
}

#[unsafe(no_mangle)]
pub extern "C" fn dag_viewer_zoom(x: f32, y: f32, direction: bool) -> () {
    let dz = if direction { 0.9 } else { 1.1 };
    let mut scene = SCENE.lock().unwrap();
    if scene.camera.zoom * dz <= 0.01 {
        scene.camera.zoom = 0.01;
        return;
    }
    let coord_before = scene.screen_to_world(&VecF2 { x: x, y: y });
    scene.camera.zoom *= dz;
    let coord_after = scene.screen_to_world(&VecF2 { x: x, y: y });
    scene.camera.pos.x += coord_before.x - coord_after.x;
    scene.camera.pos.y += coord_before.y - coord_after.y;
    draw(&*scene);
}

#[unsafe(no_mangle)]
pub extern "C" fn dag_viewer_click(x: f32, y: f32) -> () {
    let mut scene = SCENE.lock().unwrap();
    for (i, _) in scene.model.nodes.iter().enumerate() {
        if !scene.check_bound_circle(i, VecF2 { x: x, y: y }) {
            continue;
        }
        scene.highlight_bicone(i);
        match &scene.model.nodes[i].link {
            Some(l) => js::follow_link(l),
            None => (),
        }
        break;
    }
    draw(&*scene);
}

#[unsafe(no_mangle)]
pub extern "C" fn dag_viewer_hover(x: f32, y: f32) -> () {
    let mut scene = SCENE.lock().unwrap();
    let mut handle = None;
    for (i, _) in scene.model.nodes.iter().enumerate() {
        if !scene.check_bound_circle(i, VecF2 { x: x, y: y }) {
            continue;
        }
        handle = Some(i);
        break;
    }
    scene.highlight_node(handle);
    draw(&*scene);
}

#[unsafe(no_mangle)]
pub extern "C" fn dag_viewer_alloc(len: usize) -> *mut u8 {
    let mut buf = Vec::<u8>::with_capacity(len);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

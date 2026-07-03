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
mod model;
mod parser;
mod scene;

static GRAPHVIZ_LAYOUT: AtomicBool = AtomicBool::new(true);

pub fn is_graphviz_layout() -> bool {
    GRAPHVIZ_LAYOUT.load(Ordering::Relaxed)
}

static mut DOT_FILE: Option<&str> = None;

static SCENE: LazyLock<Mutex<Scene>> = LazyLock::new(|| Mutex::new(Scene::new_default()));

#[unsafe(no_mangle)]
// force the compiler to use C ABI so WebAssemply module interface is stable
pub extern "C" fn dag_viewer_init(w: i32, h: i32) -> () {
    let mut scene = SCENE.lock().unwrap();
    let dot = unsafe { DOT_FILE.expect("balh") };
    let mut s = Scene::new(w, h, dot);

    if !is_graphviz_layout() {
        s.layout(); // comment this out to use the layout_test instead
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
    let dz = if direction { -0.1 } else { 0.1 };
    let mut scene = SCENE.lock().unwrap();
    if scene.camera.zoom - dz <= 0.0 {
        return;
    }
    let coord_before = scene.screen_to_world(&VecF2 { x: x, y: y });
    scene.camera.zoom -= dz;
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
        scene.highlight_node(i);
        break;
    }
    draw(&*scene);
}

#[unsafe(no_mangle)]
pub extern "C" fn dag_viewer_load_dotfile(ptr: *const u8, len: usize) {
    let bytes: &[u8] = unsafe { std::slice::from_raw_parts(ptr, len) };
    unsafe { DOT_FILE = Some(std::str::from_utf8(bytes).unwrap()) };
}

#[unsafe(no_mangle)]
pub extern "C" fn dag_viewer_alloc(len: usize) -> *mut u8 {
    let mut buf = Vec::<u8>::with_capacity(len);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

mod drawing;

use std::sync::{LazyLock, Mutex};

use drawing::{draw, layout, Scene};

mod data_types;
use data_types::*;

static SCENE: LazyLock<Mutex<Scene>> = LazyLock::new(|| Mutex::new(Scene::new(vec![], (0.0, 0.0))));

#[unsafe(no_mangle)]
// force the compiler to use C ABI so WebAssemply module interface is stable
pub extern "C" fn dag_viewer_init(w: i32, h: i32) -> () {
    unsafe { js_log_str("banans".as_ptr()) };
    let mut scene = SCENE.lock().unwrap();
    *scene = Scene::new(layout(), (w as f32, h as f32));
    draw(&*scene);
}

#[unsafe(no_mangle)]
pub extern "C" fn dag_viewer_left() -> () {
    let mut scene = SCENE.lock().unwrap();
    scene.camera.pos.0 -= 10.0;
    draw(&*scene);
}

#[unsafe(no_mangle)]
pub extern "C" fn dag_viewer_right() -> () {
    let mut scene = SCENE.lock().unwrap();
    scene.camera.pos.0 += 10.0;
    draw(&*scene);
}

#[unsafe(no_mangle)]
pub extern "C" fn dag_viewer_up() -> () {
    let mut scene = SCENE.lock().unwrap();
    scene.camera.pos.1 -= 10.0;
    draw(&*scene);
}

#[unsafe(no_mangle)]
pub extern "C" fn dag_viewer_down() -> () {
    let mut scene = SCENE.lock().unwrap();
    scene.camera.pos.1 += 10.0;
    draw(&*scene);
}

#[unsafe(no_mangle)]
pub extern "C" fn dag_viewer_z() -> () {
    let mut scene = SCENE.lock().unwrap();
    scene.camera.zoom += 0.05;
    draw(&*scene);
}

#[unsafe(no_mangle)]
pub extern "C" fn dag_viewer_x() -> () {
    let mut scene = SCENE.lock().unwrap();
    scene.camera.zoom -= 0.05;
    draw(&*scene);
}

#[link(wasm_import_module = "dag_viewer_js")]
unsafe extern "C" {
    fn js_fill_circ(x: i32, y: i32, radius: i32, colour: Colour);
    fn js_fill_rect(x: i32, y: i32, w: i32, h: i32, colour: Colour);
    fn js_fill_line(x1: i32, y1: i32, x2: i32, y2: i32, colour: Colour, width: i32);
    fn js_log(msg: i32);
    fn js_log_str(msg: *const u8);
}

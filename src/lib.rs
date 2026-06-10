mod data_types;
mod drawing;
mod js;

use std::sync::{LazyLock, Mutex};

use drawing::{draw, layout, Scene};

static SCENE: LazyLock<Mutex<Scene>> = LazyLock::new(|| Mutex::new(Scene::new(vec![], (0.0, 0.0))));

#[unsafe(no_mangle)]
// force the compiler to use C ABI so WebAssemply module interface is stable
pub extern "C" fn dag_viewer_init(w: i32, h: i32) -> () {
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

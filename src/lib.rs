mod drawing;
mod js;

use std::sync::{LazyLock, Mutex};

use drawing::draw;
use scene::*;
mod data_types;
mod model;
mod scene;
use dot_parser::ast::Graph;
const DOT_FILE: &str = include_str!("../graph.dot");

static SCENE: LazyLock<Mutex<Scene>> = LazyLock::new(|| Mutex::new(Scene::new_default()));

#[unsafe(no_mangle)]
// force the compiler to use C ABI so WebAssemply module interface is stable
pub extern "C" fn dag_viewer_init(w: i32, h: i32) -> () {
    let mut scene = SCENE.lock().unwrap();
    let graph: Graph<(dot_parser::ast::ID<'_>, dot_parser::ast::ID<'_>)> =
        Graph::try_from(DOT_FILE).unwrap();
    *scene = Scene::new(w, h, &graph);
    draw(&*scene);
}

#[unsafe(no_mangle)]
pub extern "C" fn dag_viewer_left() -> () {
    let mut scene = SCENE.lock().unwrap();
    scene.camera.pos.x -= 10.0;
    draw(&*scene);
}

#[unsafe(no_mangle)]
pub extern "C" fn dag_viewer_right() -> () {
    let mut scene = SCENE.lock().unwrap();
    scene.camera.pos.x += 10.0;
    draw(&*scene);
}

#[unsafe(no_mangle)]
pub extern "C" fn dag_viewer_up() -> () {
    let mut scene = SCENE.lock().unwrap();
    scene.camera.pos.y -= 10.0;
    draw(&*scene);
}

#[unsafe(no_mangle)]
pub extern "C" fn dag_viewer_down() -> () {
    let mut scene = SCENE.lock().unwrap();
    scene.camera.pos.y += 10.0;
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
    if scene.camera.zoom <= 0.05 {
        return;
    }
    scene.camera.zoom -= 0.05;
    draw(&*scene);
}

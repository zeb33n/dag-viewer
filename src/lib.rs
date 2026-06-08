mod drawing;
use drawing::layout;

mod data_types;
use data_types::*;

#[unsafe(no_mangle)]
// force the compiler to use C ABI so WebAssemply module interface is stable
pub extern "C" fn dag_viewer_rs_main() -> () {
    layout();
    unsafe { js_log(1) };
    unsafe { js_fill_rect(50, 50, 100, 100, 0xFF0000FF) };
    unsafe { js_log_str("banana".as_ptr()) };
    // layout();
}

#[link(wasm_import_module = "dag_viewer_js")]
unsafe extern "C" {
    fn js_fill_rect(x: i32, y: i32, h: i32, w: i32, colour: Colour);
    fn js_log(msg: i32);
    fn js_log_str(msg: *const u8);
}

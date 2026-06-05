#[unsafe(no_mangle)]
// force the compiler to use C ABI so WebAssemply module interface is stable
pub extern "C" fn dag_viewer_rs_main() -> () {
    unsafe { js_log(1) };
    // endianness bug (colour hex is backwards (bgr not rgb))
    unsafe { js_fill_rect(50, 50, 100, 100, 0xFF00FF00) };
}

#[link(wasm_import_module = "dag_viewer_js")]
unsafe extern "C" {
    fn js_fill_rect(x: i32, y: i32, h: i32, w: i32, colour: u32);
    fn js_log(msg: i32);
}

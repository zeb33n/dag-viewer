use crate::data_types::*;

mod js_internal {
    use crate::data_types::*;

    #[link(wasm_import_module = "dag_viewer_js")]
    unsafe extern "C" {
        pub fn js_fill_circ(x: i32, y: i32, radius: i32, colour: Colour);
        pub fn js_fill_rect(x: i32, y: i32, w: i32, h: i32, colour: Colour);
        pub fn js_fill_line(x1: i32, y1: i32, x2: i32, y2: i32, colour: Colour, width: i32);
        pub fn js_fill_string(x: i32, y: i32, s: *const u8, colour: Colour, size: i32);

        // debugging functions
        #[allow(dead_code)]
        pub fn js_log(msg: i32);
        #[allow(dead_code)]
        pub fn js_log_str(msg: *const u8);
    }
}

pub fn fill_circ(x: i32, y: i32, radius: i32, colour: Colour) -> () {
    unsafe { js_internal::js_fill_circ(x, y, radius, colour) }
}

pub fn fill_rect(x: i32, y: i32, w: i32, h: i32, colour: Colour) -> () {
    unsafe { js_internal::js_fill_rect(x, y, w, h, colour) }
}

pub fn fill_line(x1: i32, y1: i32, x2: i32, y2: i32, colour: Colour, width: i32) -> () {
    unsafe { js_internal::js_fill_line(x1, y1, x2, y2, colour, width) }
}

pub fn fill_string(x: i32, y: i32, s: *const u8, colour: Colour, size: i32) -> () {
    unsafe { js_internal::js_fill_string(x, y, s, colour, size) }
}

#[allow(dead_code)]
pub fn log(msg: i32) -> () {
    unsafe { js_internal::js_log(msg) }
}

#[allow(dead_code)]
pub fn log_str(msg: *const u8) -> () {
    unsafe { js_internal::js_log_str(msg) }
}

use crate::data_types::*;

pub mod js_internal {
    use crate::data_types::*;

    #[link(wasm_import_module = "dag_viewer_js")]
    unsafe extern "C" {
        pub fn js_log(ptr: *const u8, len: usize);
        pub fn js_fill_circ(x: f32, y: f32, radius: f32, colour: Colour);
        pub fn js_fill_rect(x: f32, y: f32, w: f32, h: f32, colour: Colour);
        pub fn js_fill_line(x1: f32, y1: f32, x2: f32, y2: f32, colour: Colour, width: f32);
        pub fn js_fill_string(x: f32, y: f32, s: *const u8, len: usize, colour: Colour, size: f32);
    }
}

pub fn fill_circ(x: f32, y: f32, radius: f32, colour: Colour) -> () {
    unsafe { js_internal::js_fill_circ(x, y, radius, colour) }
}

pub fn fill_rect(x: f32, y: f32, w: f32, h: f32, colour: Colour) -> () {
    unsafe { js_internal::js_fill_rect(x, y, w, h, colour) }
}

pub fn fill_line(x1: f32, y1: f32, x2: f32, y2: f32, colour: Colour, width: f32) -> () {
    unsafe { js_internal::js_fill_line(x1, y1, x2, y2, colour, width) }
}

pub fn log(msg: &str) -> () {
    unsafe { js_internal::js_log(msg.as_ptr(), msg.len()) }
}

#[macro_export] 
macro_rules! web_print {
    ($($t:tt)*) => {{
        let s = format!($($t)*);
        unsafe { $crate::js::js_internal::js_log(s.as_ptr(), s.len()) }
    }};
}

pub fn fill_string(x: f32, y: f32, s: &str, colour: Colour, size: f32) -> () {
    unsafe { js_internal::js_fill_string(x, y, s.as_ptr(), s.len(), colour, size) }
}

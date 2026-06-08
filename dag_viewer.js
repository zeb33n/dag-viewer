'use strict';

let app = document.getElementById("dag_viewer");
let ctx = app.getContext("2d");
let w = null;
let text = "";

function color_hex(colour) {
    const r = ((colour>>(3*8))&0xFF).toString(16).padStart(2, '0');
    const g = ((colour>>(2*8))&0xFF).toString(16).padStart(2, '0');
    const b = ((colour>>(1*8))&0xFF).toString(16).padStart(2, '0');
    const a = ((colour>>(0*8))&0xFF).toString(16).padStart(2, '0');
    console.log(a);
    return "#"+r+g+b+a;
}

function js_fill_rect(x, y, w, h, colour) {
    ctx.fillStyle = color_hex(colour); 
    ctx.fillRect(x, y, w, h);
}

function js_log_str(msg) {
    console.log(get_text(msg));
}

function js_log(msg) {
    console.log(msg);
}

function cstrlen(mem, ptr) {
    let len = 0;
    while (mem[ptr] != 0) {
        len++;
        ptr++;
    }
    return len;
}

function cstr_by_ptr(mem_buffer, ptr) {
    const mem = new Uint8Array(mem_buffer);
    const len = cstrlen(mem, ptr);
    const bytes = new Uint8Array(mem_buffer, ptr, len);
    return new TextDecoder().decode(bytes);
}


function get_text(msg) {
    const buffer = w.instance.exports.memory.buffer;
    const message = cstr_by_ptr(buffer, msg);
    return message;
}

const wasm_path = new URL('target/wasm32-unknown-unknown/release/dag_viewer.wasm', import.meta.url);

w = await WebAssembly.instantiateStreaming(await fetch(wasm_path), {
    dag_viewer_js: {
        js_fill_rect,
        js_log_str,
        js_log,
    }

})

export function dag_viewer_init() {
    w.instance.exports.dag_viewer_rs_main();
}

'use strict';

let app = document.getElementById("dag_viewer");
let ctx = app.getContext("2d");
let w = null;
let text = "";
let mouse_is_down = false;
let mouse_click_pos = { x: 0, y: 0};

function color_hex(colour) {
    const r = ((colour>>(3*8))&0xFF).toString(16).padStart(2, '0');
    const g = ((colour>>(2*8))&0xFF).toString(16).padStart(2, '0');
    const b = ((colour>>(1*8))&0xFF).toString(16).padStart(2, '0');
    const a = ((colour>>(0*8))&0xFF).toString(16).padStart(2, '0');
    return "#"+r+g+b+a;
}

function js_fill_rect(x, y, w, h, colour) {
    ctx.fillStyle = color_hex(colour); 
    ctx.fillRect(x, y, w, h);
}

function js_fill_line(x1, y1, x2, y2, colour, width) {
    ctx.beginPath();
    ctx.moveTo(x1, y1);
    ctx.lineTo(x2, y2);
    ctx.strokeStyle = color_hex(colour);
    ctx.lineWidth = width;
    ctx.stroke();
}

function js_fill_circ(x, y, radius, colour) {
    ctx.beginPath();
    ctx.arc(x, y, radius, 0, 2 * Math.PI);
    ctx.fillStyle = color_hex(colour);
    ctx.fill();
}

function js_fill_string(x, y, pString, stringLen, colour, size) {
    ctx.fillStyle = color_hex(colour);
    ctx.font = size + "px monospace";
    const bytes = new Uint8Array(w.instance.exports.memory.buffer, pString, stringLen);
    const str = new TextDecoder().decode(bytes);
    ctx.fillText(str, x, y); 
}

function canvas_coords(e) {
    const bounding_box = app.getBoundingClientRect();

    return {
        x: (e.clientX - bounding_box.left) * app.width / app.clientWidth,
        y: (e.clientY - bounding_box.top) * app.height / app.clientHeight,
    };
}

const wasm_path = new URL('target/wasm32-unknown-unknown/release/dag_viewer.wasm', import.meta.url);

w = await WebAssembly.instantiateStreaming(await fetch(wasm_path), {
    dag_viewer_js: {
        js_fill_rect,
        js_fill_line,
        js_fill_circ, 
        js_log: (ptr, len) => {
            const bytes = new Uint8Array(w.instance.exports.memory.buffer, ptr, len);
            const str = new TextDecoder().decode(bytes);
            console.log(str);
        },

        js_fill_string,
    }
})

export function dag_viewer_init() {
    w.instance.exports.dag_viewer_init(app.width, app.height);

    app.addEventListener("mousedown", (e) => {
        const coords = canvas_coords(e);
        mouse_click_pos = {x: coords.x, y: coords.y};
        mouse_is_down = true;
    });

    app.addEventListener("mouseup", (_) => {
        mouse_is_down = false;
    });

    app.addEventListener("mouseleave", (_) => {
        mouse_is_down = false;
    });

    app.addEventListener("mousemove", (e) => {
        if (!mouse_is_down) return;

        const coords = canvas_coords(e);
        
        const dx = mouse_click_pos.x - coords.x;
        const dy = mouse_click_pos.y - coords.y;

        mouse_click_pos = {x: coords.x, y: coords.y};

        w.instance.exports.dag_viewer_drag(dx, dy);
    });

    app.addEventListener("wheel", (e) => {
        const coords = canvas_coords(e);
        const direction = e.deltaY < 0;
        w.instance.exports.dag_viewer_zoom(coords.x, coords.y, direction);
    });

    app.addEventListener("click", (e) => {
        const coords = canvas_coords(e);
        w.instance.exports.dag_viewer_click(coords.x, coords.y);
    })
}

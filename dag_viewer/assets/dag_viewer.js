'use strict';

// TODO need to load everything per init for multi graph support
function color_hex(colour) {
    const r = ((colour>>(3*8))&0xFF).toString(16).padStart(2, '0');
    const g = ((colour>>(2*8))&0xFF).toString(16).padStart(2, '0');
    const b = ((colour>>(1*8))&0xFF).toString(16).padStart(2, '0');
    const a = ((colour>>(0*8))&0xFF).toString(16).padStart(2, '0');
    return "#"+r+g+b+a;
}

export async function dag_viewer_init(dotfile, id) {
    let app = document.getElementById(id);
    let ctx = app.getContext("2d");
    let w = null;
    let text = "";
    let mouse_is_down = false;
    let mouse_click_pos = { x: 0, y: 0};

    function resize_canvas() {
        const dpr = window.devicePixelRatio || 1;
        const rect = app.getBoundingClientRect();
        app.width = rect.width * dpr;
        app.height = rect.height * dpr;
        ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
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

    function js_follow_link(pString, stringLen) {
        const bytes = new Uint8Array(w.instance.exports.memory.buffer, pString, stringLen);
        const str = new TextDecoder().decode(bytes);
        window.location.href = str;
    }

    function js_log(ptr, len) {
        const bytes = new Uint8Array(w.instance.exports.memory.buffer, ptr, len);
        const str = new TextDecoder().decode(bytes);
        console.log(str);
    }

    function canvas_coords(e) {
        const bounding_box = app.getBoundingClientRect();

        return {
            x: (e.clientX - bounding_box.left),
            y: (e.clientY - bounding_box.top),
        };
    }

    // resize the canvas
    resize_canvas();
    
    // load wasm
    const wasm_path = new URL('dag_viewer.wasm', import.meta.url);
    w = await WebAssembly.instantiateStreaming(await fetch(wasm_path), {
        dag_viewer_js: {
            js_fill_rect,
            js_fill_line,
            js_fill_circ,
            js_log,
            js_fill_string,
            js_follow_link,
        }
    })

    // load the dot file into wasm memory
    const dotPath = new URL(dotfile, import.meta.url);
    const res = await fetch(dotPath);
    const buf = await res.arrayBuffer();
    const d = new Uint8Array(buf);
    const ptr = w.instance.exports.dag_viewer_alloc(d.length);
    new Uint8Array(w.instance.exports.memory.buffer).set(d, ptr);

    // start the app
    const rectangle = app.getBoundingClientRect();
    w.instance.exports.dag_viewer_init(rectangle.width, rectangle.height, ptr, d.length);

    // event listeners
    window.addEventListener("resize", resize_canvas);

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
        const coords = canvas_coords(e);
        if (!mouse_is_down) {
            w.instance.exports.dag_viewer_hover(coords.x, coords.y);
            return
        };
        
        const dx = mouse_click_pos.x - coords.x;
        const dy = mouse_click_pos.y - coords.y;

        mouse_click_pos = {x: coords.x, y: coords.y};

        w.instance.exports.dag_viewer_drag(dx, dy);
    } );

    app.addEventListener("wheel", (e) => {
        e.preventDefault()
        const coords = canvas_coords(e);
        const direction = e.deltaY < 0;
        w.instance.exports.dag_viewer_zoom(coords.x, coords.y, direction);
    }, { passive: false });

    app.addEventListener("click", (e) => {
        const coords = canvas_coords(e);
        w.instance.exports.dag_viewer_click(coords.x, coords.y);
    })
}


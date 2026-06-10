use crate::{data_types::*, model::*, scene::*, js_fill_circ, js_fill_line, js_fill_rect, js_log};



struct Line {
    a: Vec2,
    b: Vec2,
    colour: Colour,
}

struct Path {
    from: LogicalNodeHandle,
    to: LogicalNodeHandle,
    line_segments: Vec<Line>,
}

pub struct DrawableNode {
    is_fake_node: bool,
    position: Vec2,
    logical_node_handle: LogicalNodeHandle,
    edges: Vec<Path>,
}

pub fn layout_test() -> Vec<DrawableNode> {
    let nodes = vec![
        Node {
            label: String::from("node_1"),
            dependents: vec![]
        },
        Node {
            label: String::from("node_2"),
            dependents: vec![]
        },
    ];

    let line = Line {
        a: (50, 50),
        b: (100, 100),
        colour: 0xFFFFFFFF,
    };

    let path = Path {
        from: 0,
        to: 1,
        line_segments: vec![line],
    };

    let node_1 = DrawableNode {
        is_fake_node: false,
        position: (50, 50),
        logical_node_handle: 0,
        edges: vec![path],
    };

    let node_2 = DrawableNode {
        is_fake_node: false,
        position: (100, 100),
        logical_node_handle: 1,
        edges: vec![],
    };

    vec![node_1, node_2]
}

pub fn draw(scene: &Scene) -> () {
    unsafe {
        js_fill_rect(0, 0, scene.screen_width, scene.screen_height, 0xFFFFFFFF);
    }
    for node in scene.nodes.iter() {
        for path in node.edges.iter() {
            for line in path.line_segments.iter() {
                let a = scene.to_screen(line.a);
                let b = scene.to_screen(line.b);
                unsafe { js_fill_line(a.0, a.1, b.0, b.1, 0x00FFFFFF, 5) };
            }
        }
        let position = scene.to_screen(node.position);
        unsafe { js_fill_circ(position.0, position.1, 10, 0x00FF00FF) };
    }
}

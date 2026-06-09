use crate::{data_types::*, js_fill_circ, js_fill_line, js_fill_rect, js_log};
use std::sync::OnceLock;

#[derive(Debug)]
struct Node {
    label: String,
}

static LOGICAL_NODES: OnceLock<Vec<Node>> = OnceLock::new();

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

fn init_nodes(nodes: Vec<Node>) -> () {
    LOGICAL_NODES.set(nodes).unwrap();
}

fn get_node(handle: LogicalNodeHandle) -> &'static Node {
    &LOGICAL_NODES.get().unwrap()[handle]
}

pub fn layout() -> Vec<DrawableNode> {
    let nodes = vec![
        Node {
            label: String::from("node_1"),
        },
        Node {
            label: String::from("node_2"),
        },
    ];

    init_nodes(nodes);

    let line = Line {
        a: (50.0, 50.0),
        b: (100.0, 100.0),
        colour: 0xFFFFFFFF,
    };

    let path = Path {
        from: 0,
        to: 1,
        line_segments: vec![line],
    };

    let node_1 = DrawableNode {
        is_fake_node: false,
        position: (50.0, 50.0),
        logical_node_handle: 0,
        edges: vec![path],
    };

    let node_2 = DrawableNode {
        is_fake_node: false,
        position: (100.0, 100.0),
        logical_node_handle: 1,
        edges: vec![],
    };

    vec![node_1, node_2]
}

pub struct Camera {
    pub pos: Vec2,
    pub zoom: f32,
}

pub struct Scene {
    pub camera: Camera,
    nodes: Vec<DrawableNode>,
    screen_width: f32,
    screen_height: f32,
}

impl Scene {
    pub fn new(nodes: Vec<DrawableNode>, screen_dim: Vec2) -> Self {
        Self {
            camera: Camera {
                pos: (screen_dim.0 / 2.0, screen_dim.1 / 2.0),
                zoom: 1.0,
            },
            nodes: nodes,
            screen_width: screen_dim.0,
            screen_height: screen_dim.1,
        }
    }

    fn to_screen(&self, coord: Vec2) -> Vec2 {
        (
            (coord.0 - self.camera.pos.0) * self.camera.zoom + self.screen_width,
            (coord.1 - self.camera.pos.1) * self.camera.zoom + self.screen_height,
        )
    }
}

pub fn draw(scene: &Scene) -> () {
    unsafe {
        js_fill_rect(
            0,
            0,
            scene.screen_width as i32,
            scene.screen_height as i32,
            0xFFFFFFFF,
        )
    };
    for node in scene.nodes.iter() {
        for path in node.edges.iter() {
            for line in path.line_segments.iter() {
                let a = scene.to_screen(line.a);
                let b = scene.to_screen(line.b);
                unsafe {
                    js_fill_line(
                        a.0 as i32, a.1 as i32, b.0 as i32, b.1 as i32, 0x000000FF, 5,
                    )
                };
            }
        }
        let position = scene.to_screen(node.position);
        unsafe { js_fill_circ(position.0 as i32, position.1 as i32, 10, 0x00FF00FF) };
        unsafe { js_log(position.0 as i32) };
        unsafe { js_log(position.1 as i32) };
    }
}

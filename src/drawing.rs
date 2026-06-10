use crate::{data_types::*, js};
use std::sync::OnceLock;

#[derive(Debug)]
struct Node {
    label: String,
}

static LOGICAL_NODES: OnceLock<Vec<Node>> = OnceLock::new();

struct Line {
    a: VecF2,
    b: VecF2,
    colour: Colour,
}

struct Path {
    from: LogicalNodeHandle,
    to: LogicalNodeHandle,
    line_segments: Vec<Line>,
}

pub struct DrawableNode {
    is_fake_node: bool,
    position: VecF2,
    logical_node_handle: LogicalNodeHandle,
    edges: Vec<Path>,
    colour: Colour,
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

    let line_1 = Line {
        a: VecF2 { x: 50.0, y: 50.0 },
        b: VecF2 { x: 100.0, y: 100.0 },
        colour: 0x000000FF,
    };

    let line_2 = Line {
        a: VecF2 { x: 150.0, y: 50.0 },
        b: VecF2 { x: 100.0, y: 100.0 },
        colour: 0x000000FF,
    };

    let path_1 = Path {
        from: 0,
        to: 1,
        line_segments: vec![line_1],
    };

    let path_2 = Path {
        from: 3,
        to: 1,
        line_segments: vec![line_2],
    };

    let node_1 = DrawableNode {
        is_fake_node: false,
        position: VecF2 { x: 50.0, y: 50.0 },
        logical_node_handle: 0,
        edges: vec![path_1],
        colour: 0xFF0000FF,
    };

    let node_2 = DrawableNode {
        is_fake_node: false,
        position: VecF2 { x: 100.0, y: 100.0 },
        logical_node_handle: 1,
        edges: vec![],
        colour: 0x00FF00FF,
    };

    let node_3 = DrawableNode {
        is_fake_node: false,
        position: VecF2 { x: 150.0, y: 50.0 },
        logical_node_handle: 1,
        edges: vec![path_2],
        colour: 0x0000FFFF,
    };

    vec![node_1, node_2, node_3]
}

pub struct Camera {
    pub pos: VecF2,
    pub zoom: f32,
}

pub struct Scene {
    pub camera: Camera,
    nodes: Vec<DrawableNode>,
    screen_width: f32,
    screen_height: f32,
}

impl Scene {
    pub fn new(nodes: Vec<DrawableNode>, screen_w: f32, screen_h: f32) -> Self {
        Self {
            camera: Camera {
                pos: VecF2 {
                    x: screen_w / 2.0,
                    y: screen_h / 2.0,
                },
                zoom: 1.0,
            },
            nodes: nodes,
            screen_width: screen_w,
            screen_height: screen_h,
        }
    }

    fn to_screen(&self, coord: &VecF2) -> VecI2 {
        VecI2::from_vecf2(VecF2 {
            x: (coord.x - self.camera.pos.x) * self.camera.zoom + self.screen_width / 2.0,
            y: (coord.y - self.camera.pos.y) * self.camera.zoom + self.screen_height / 2.0,
        })
    }
}

pub fn draw(scene: &Scene) -> () {
    js::fill_rect(
        0,
        0,
        scene.screen_width as i32,
        scene.screen_height as i32,
        0xFFFFFFFF,
    );
    for node in scene.nodes.iter() {
        for path in node.edges.iter() {
            for line in path.line_segments.iter() {
                let a = scene.to_screen(&line.a);
                let b = scene.to_screen(&line.b);
                js::fill_line(a.x, a.y, b.x, b.y, line.colour, 5)
            }
        }
    }
    for node in scene.nodes.iter() {
        let position = scene.to_screen(&node.position);
        js::fill_circ(position.x, position.y, 10, node.colour);
        js::log(position.x);
        js::log(position.y);
    }
}

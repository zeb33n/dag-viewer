use crate::data_types::*;
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

fn init_nodes(nodes: Vec<Node>) {
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

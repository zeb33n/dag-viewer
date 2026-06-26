use crate::scene::Scene;
use crate::web_print;
use crate::{data_types::*, js};

#[derive(Clone)]
pub struct Line {
    pub a: VecF2,
    pub b: VecF2,
    pub colour: Colour,
}

#[derive(Clone)]

pub struct Path {
    pub from: LogicalNodeHandle,
    pub to: LogicalNodeHandle,
    pub line_segments: Vec<Line>,
}

impl Path {
    pub fn new(to: LogicalNodeHandle, from: LogicalNodeHandle) -> Self {
        Self {
            from: from,
            to: to,
            line_segments: vec![],
        }
    }
}

#[derive(Clone)]
pub struct DrawableNode {
    pub is_fake_node: bool,
    pub position: VecF2,
    pub logical_node_handle: LogicalNodeHandle,
    pub edges: Vec<Path>,
    pub colour: Colour,
}

impl DrawableNode {
    pub fn new(handle: LogicalNodeHandle) -> Self {
        Self {
            is_fake_node: false,
            position: VecF2 { x: 0.0, y: 0.0 },
            logical_node_handle: handle,
            edges: vec![],
            colour: 0xFF000055,
        }
    }
}

pub fn draw(scene: &Scene) -> () {
    js::fill_rect(0.0, 0.0, scene.screen_w, scene.screen_h, 0xFFFFFFFF);
    for node in scene.nodes.iter() {
        for path in node.edges.iter() {
            for line in path.line_segments.iter() {
                let a = scene.world_to_screen(&line.a);
                let b = scene.world_to_screen(&line.b);
                let width = 5.0 * scene.camera.zoom;
                js::fill_line(a.x, a.y, b.x, b.y, line.colour, width);
            }
        }
    }
    for node in scene.nodes.iter() {
        if node.is_fake_node {
            continue;
        }
        let p = scene.world_to_screen(&node.position);
        let radius = 30.0 * scene.camera.zoom;
        js::fill_circ(p.x, p.y, radius, node.colour);
        let text: &str = &scene.model.get_node(node.logical_node_handle).label;
        js::fill_string(p.x + radius, p.y, text, 0x000000FF, radius * 2.0);
    }
}

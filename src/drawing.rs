use crate::model::Path;
use crate::scene::Scene;
use crate::{data_types::*, js};

pub struct DrawableNode {
    pub is_fake_node: bool,
    pub position: VecF2,
    pub logical_node_handle: LogicalNodeHandle,
    pub edges: Vec<Path>,
    pub colour: Colour,
}

pub fn draw(scene: &Scene) -> () {
    js::fill_rect(
        0,
        0,
        scene.screen_w as i32,
        scene.screen_h as i32,
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

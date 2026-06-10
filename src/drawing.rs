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
                let width = (5.0 * scene.camera.zoom) as i32;
                js::fill_line(a.x, a.y, b.x, b.y, line.colour, width);
            }
        }
    }
    for node in scene.nodes.iter() {
        let p = scene.to_screen(&node.position);
        let radius = (10.0 * scene.camera.zoom) as i32;
        js::fill_circ(p.x, p.y, radius, node.colour);
        let text: *const u8 = scene
            .model
            .get_node(node.logical_node_handle)
            .label
            .as_ptr();
        js::fill_string(p.x + radius, p.y, text, 0x000000FF, radius * 2);
    }
}

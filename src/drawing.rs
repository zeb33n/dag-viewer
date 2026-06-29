use crate::js;
use crate::scene::Scene;

pub fn draw(scene: &Scene) -> () {
    js::fill_rect(0.0, 0.0, scene.screen_w, scene.screen_h, 0xFFFFFFFF);
    for path in scene.edges.iter() {
        for line in path.line_segments.iter() {
            let a = scene.world_to_screen(&line.a);
            let b = scene.world_to_screen(&line.b);
            let width = 5.0 * scene.camera.zoom;
            js::fill_line(a.x, a.y, b.x, b.y, line.colour, width);
        }
    }
    for handle in scene.nodes.iter() {
        let node = scene.model.get_node(*handle);
        if node.is_fake_node {
            continue;
        }
        let p = scene.world_to_screen(&node.position);
        let radius = 30.0 * scene.camera.zoom;
        js::fill_circ(p.x, p.y, radius, node.colour);
        let text: &str = &node.label;
        js::fill_string(p.x + radius, p.y, text, 0x000000FF, radius * 2.0);
    }
}

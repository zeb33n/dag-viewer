use crate::colours::COLOURS_HIGHLIGHT;
use crate::js;
use crate::scene::Scene;

pub fn draw(scene: &Scene) -> () {
    js::fill_rect(0.0, 0.0, scene.screen_w, scene.screen_h, 0xFFFFFFFF);
    for path in scene.model.edges.iter() {
        for line in path.line_segments.iter() {
            let a = scene.world_to_screen(&line.a);
            let b = scene.world_to_screen(&line.b);
            let width = 5.0 * scene.camera.zoom;
            js::fill_line(a.x, a.y, b.x, b.y, line.colour, width);
        }
    }
    let mut labels = Vec::new();
    for node in scene.model.nodes.iter() {
        if node.is_fake_node {
            continue;
        }
        let p = scene.world_to_screen(&node.position);
        let radius = node.radius * scene.camera.zoom;
        js::fill_circ(p.x, p.y, radius, node.colour);
        if scene.camera.zoom <= 0.1 && node.label_colour != COLOURS_HIGHLIGHT.text {
            continue;
        }
        labels.push((
            p.x + radius,
            p.y,
            &node.label,
            node.label_colour,
            node.label_size,
        ));
    }
    for (x, y, label, colour, size) in labels {
        js::fill_string(x, y, label, colour, size);
    }
}

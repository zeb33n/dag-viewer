use std::collections::HashMap;

use crate::drawing::DrawableNode;
use crate::js;
use crate::{data_types::*, model::*};
use dot_parser::ast::{Graph, ID};

pub struct Camera {
    pub pos: VecF2,
    pub zoom: f32,
}

pub struct Scene {
    pub camera: Camera,
    pub nodes: Vec<DrawableNode>,
    pub screen_w: f32,
    pub screen_h: f32,
    pub model: Model,
    pub root: usize,
}

impl Scene {
    pub fn new(screen_w: i32, screen_h: i32, graph: &Graph<(ID<'_>, ID<'_>)>) -> Self {
        let mut model = Model::new(graph);
        Self {
            camera: Camera {
                pos: VecF2 {
                    x: screen_w as f32 / 2.0,
                    y: screen_h as f32 / 2.0,
                },
                zoom: 1.0,
            },
            nodes: Self::layout_test(&mut model),
            screen_w: screen_w as f32,
            screen_h: screen_h as f32,
            model: model,
            // TODO calculate this properly
            root: 3,
        }
    }

    pub fn new_default() -> Self {
        Self {
            camera: Camera {
                pos: VecF2 { x: 0.0, y: 0.0 },
                zoom: 1.0,
            },
            nodes: Self::layout_test(&mut Model::new_default()),
            screen_w: 0.0,
            screen_h: 0.0,
            model: Model::new_default(),
            root: 0,
        }
    }

    pub fn layout() -> () {}

    pub fn layout_test(model: &mut Model) -> Vec<DrawableNode> {
        model.logical_nodes = vec![
            Node {
                label: String::from("node_1"),
                dependents: vec![1],
            },
            Node {
                label: String::from("node_2"),
                dependents: vec![],
            },
            Node {
                label: String::from("node_3"),
                dependents: vec![1],
            },
            Node {
                label: String::from("node_4"),
                dependents: vec![0, 2, 4],
            },
            Node {
                label: String::from("node_5"),
                dependents: vec![],
            },
        ];

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

        let line_3 = Line {
            a: VecF2 { x: 150.0, y: 0.0 },
            b: VecF2 { x: 50.0, y: 50.0 },
            colour: 0x000000FF,
        };

        let line_4 = Line {
            a: VecF2 { x: 150.0, y: 0.0 },
            b: VecF2 { x: 150.0, y: 50.0 },
            colour: 0x000000FF,
        };

        let line_5 = Line {
            a: VecF2 { x: 150.0, y: 0.0 },
            b: VecF2 { x: 200.0, y: 50.0 },
            colour: 0x000000FF,
        };

        let path_1 = Path {
            from: 0,
            to: 1,
            line_segments: vec![line_1],
        };

        let path_2 = Path {
            from: 2,
            to: 1,
            line_segments: vec![line_2],
        };

        let path_3 = Path {
            from: 3,
            to: 0,
            line_segments: vec![line_3],
        };

        let path_4 = Path {
            from: 3,
            to: 2,
            line_segments: vec![line_4],
        };

        let path_5 = Path {
            from: 3,
            to: 4,
            line_segments: vec![line_5],
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
            logical_node_handle: 2,
            edges: vec![path_2],
            colour: 0x0000FFFF,
        };

        let node_4 = DrawableNode {
            is_fake_node: false,
            position: VecF2 { x: 150.0, y: 0.0 },
            logical_node_handle: 3,
            edges: vec![path_3, path_4, path_5],
            colour: 0xFF00FFFF,
        };

        let node_5 = DrawableNode {
            is_fake_node: false,
            position: VecF2 { x: 200.0, y: 50.0 },
            logical_node_handle: 4,
            edges: vec![],
            colour: 0xFFFF00FF,
        };

        vec![node_1, node_2, node_3, node_4, node_5]
    }

    pub fn world_to_screen(&self, coord: &VecF2) -> VecF2 {
        VecF2 {
            x: (coord.x - self.camera.pos.x) * self.camera.zoom + self.screen_w / 2.0,
            y: (coord.y - self.camera.pos.y) * self.camera.zoom + self.screen_h / 2.0,
        }
    }

    pub fn screen_to_world(&self, coord: &VecF2) -> VecF2 {
        VecF2 {
            x: (coord.x - self.screen_w / 2.0) / self.camera.zoom + self.camera.pos.x,
            y: (coord.y - self.screen_h / 2.0) / self.camera.zoom + self.camera.pos.y,
        }
    }

    pub fn get_bound_circle(&self, handle: usize) -> Circle {
        let node = &self.nodes[handle];
        Circle {
            center: self.world_to_screen(&VecF2 {
                x: node.position.x,
                y: node.position.y,
            }),
            radius: 10.0 * self.camera.zoom,
        }
    }

    pub fn check_bound_circle(&self, handle: usize, coord: VecF2) -> bool {
        let circ = self.get_bound_circle(handle);
        let dx = coord.x - circ.center.x;
        let dy = coord.y - circ.center.y;

        dx * dx + dy * dy <= circ.radius * circ.radius
    }

    pub fn highlight_node(&self, handle: usize) {
        let mut nodes = self.get_reverse_dependencies(handle);
        nodes.extend(self.get_dependencies(handle).iter());
        for h in nodes.into_iter() {
            js::log_str(self.model.get_node(h).label.as_ptr());
        }
    }

    pub fn get_dependencies(&self, handle: usize) -> Vec<usize> {
        let mut out: Vec<usize> = Vec::new();
        self.recursive_dependencies(handle, &mut out);
        return out;
    }

    fn recursive_dependencies(&self, handle: usize, out: &mut Vec<usize>) {
        let node = &self.nodes[handle];
        for e in node.edges.iter() {
            if out.contains(&e.to) {
                return;
            };
            out.push(e.to);
            self.recursive_dependencies(e.to, out);
        }
    }

    // TODO may need optimising in theory the filter
    // needs only to be called once on the root node
    pub fn get_reverse_dependencies(&self, handle: usize) -> Vec<usize> {
        let mut seen: HashMap<usize, bool> = HashMap::new();
        self.nodes
            .iter()
            .enumerate()
            .map(|(i, _)| i)
            .collect::<Vec<usize>>()
            .into_iter()
            .filter(|h| self.recursive_reverse_dependencies_filter(*h, handle, &mut seen))
            .collect()
    }

    fn recursive_reverse_dependencies_filter(
        &self,
        handle: usize,
        find: usize,
        seen: &mut HashMap<usize, bool>,
    ) -> bool {
        if seen.contains_key(&handle) {
            return *seen.get(&handle).unwrap();
        }

        if handle == find {
            seen.insert(handle, true);
            return true;
        }

        let node = &self.nodes[handle];

        let out = node
            .edges
            .iter()
            .any(|edge| self.recursive_reverse_dependencies_filter(edge.to, find, seen));
        seen.insert(handle, out);
        out
    }
}

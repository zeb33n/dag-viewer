use crate::drawing::DrawableNode;
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
            logical_node_handle: 2,
            edges: vec![path_2],
            colour: 0x0000FFFF,
        };

        vec![node_1, node_2, node_3]
    }

    pub fn world_to_screen(&self, coord: &VecF2) -> VecF2 {
        VecF2 {
            x: (coord.x - self.camera.pos.x) * self.camera.zoom + self.screen_w / 2.0,
            y: (coord.y - self.camera.pos.y) * self.camera.zoom + self.screen_h / 2.0,
        }
    }
}

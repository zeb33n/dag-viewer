use dot_parser::ast::{Graph, ID};

use crate::js_log;
use crate::{data_types::Vec2, drawing::DrawableNode, model::*};
use crate::drawing::{draw, layout_test};

pub struct Scene {
    pub camera: Vec2,
    pub nodes: Vec<DrawableNode>,
    pub screen_width: i32,
    pub screen_height: i32,
    pub model: Model
}

impl Scene {
    pub fn new(screen_dim: Vec2, graph: &Graph<(ID<'_>, ID<'_>)>) -> Self {
        Self {
            camera: (screen_dim.0 / 2, screen_dim.1 / 2),
            nodes: layout_test(),
            screen_width: screen_dim.0,
            screen_height: screen_dim.1,
            model: Model::new(graph)
        }
    }

    pub fn new_default() -> Self {
        Self {
            camera: (0, 0),
            nodes: layout_test(),
            screen_width: 0,
            screen_height: 0,
            model: Model::new_default()
        }
    }

    pub fn layout(&mut self) -> () {
        // populate nodes 
    }

    pub fn to_screen(&self, coord: Vec2) -> Vec2 {
        (coord.0 + self.camera.0, coord.1 + self.camera.1)
    }
}
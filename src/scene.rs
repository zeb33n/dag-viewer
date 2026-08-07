use std::collections::{HashMap, HashSet};
use std::ops::Mul;

use crate::colours::{COLOURS, COLOURS_HIGHLIGHT};
use crate::{data_types::*, model::*};

use crate::web_print;

pub struct Camera {
    pub pos: VecF2,
    pub zoom: f32,
}

pub struct Scene {
    pub camera: Camera,
    pub screen_w: f32,
    pub screen_h: f32,
    pub model: Model,
}

/*
    Wraps a DrawableNode, adds variables needed for the layout algorithm.
    The very last part of the layout function extracts the inner DrawableNode.
*/
#[derive(Clone)]
struct LayoutNodeWrapper {
    handle: usize,

    /*
        which layers is the node in?
        - the root node is layer 0, its dependencies are layer 1, then theirs are layer 2, etc
    */
    layers: Vec<usize>,

    /*
        what's layer is it actually in
    */
    layer: usize,

    /*
        dependent nodes (layer, node).
        These are notional connections that are then made concrete by adding
        paths to the Drawable node
    */
    dependents: Vec<(usize, usize)>,
}

impl Scene {
    pub fn new(screen_w: f32, screen_h: f32, src: &str) -> Self {
        let mut out = Self {
            camera: Camera {
                pos: VecF2 {
                    x: screen_w / 2.0,
                    y: screen_h / 2.0,
                },
                zoom: 1.0,
            },
            screen_w: screen_w,
            screen_h: screen_h,
            model: Model::from_source(src),
        };
        out.center_camera();
        out
    }

    pub fn new_default() -> Self {
        Self {
            camera: Camera {
                pos: VecF2 { x: 0.0, y: 0.0 },
                zoom: 1.0,
            },
            screen_w: 0.0,
            screen_h: 0.0,
            model: Model::new_default(),
        }
    }

    fn gather_nodes(
        &self,
        node: &Node,
        out: &mut HashMap<String, LayoutNodeWrapper>,
        on_layer: usize,
    ) -> () {
        if on_layer > 5 {
            return;
        }
        for h_node in &node.dependents {
            let n: &Node = self.model.get_node(*h_node);
            if let Some(val) = out.get_mut(&n.label) {
                val.layers.push(on_layer);
            } else {
                out.insert(
                    n.label.clone(),
                    LayoutNodeWrapper {
                        layers: vec![on_layer],
                        handle: *h_node,
                        layer: 0,
                        dependents: vec![],
                    },
                );
            }
            self.gather_nodes(n, out, on_layer + 1);
        }
    }

    fn find_dependent(
        dependent: NodeHandle,
        by_layer: &Vec<Vec<LayoutNodeWrapper>>,
        start_layer: usize,
    ) -> Option<(usize, usize)> {
        for i in start_layer..by_layer.len() {
            for j in 0..by_layer[i].len() {
                //wrapper in &by_layer[i] {
                let wrapper = &by_layer[i][j];
                if wrapper.handle == dependent {
                    //web_print!("some");
                    return Some((i, j));
                }
            }
        }
        //web_print!("none");
        None
    }

    fn connect_nodes(&mut self, by_layer: &mut Vec<Vec<LayoutNodeWrapper>>) -> () {
        let mut connections: Vec<(usize, usize, usize, usize)> = Vec::new(); // (from_layer, from_node, to_layer, to_node)
        for i in 0..by_layer.len() {
            let layer = &by_layer[i];
            for j in 0..layer.len() {
                let wrapper = &layer[j];
                //web_print!("{} {:?}", model.get_node(wrapper.node.logical_node_handle).label, model.get_node(wrapper.node.logical_node_handle).dependents);
                for d in &self.model.get_node(wrapper.handle).dependents {
                    web_print!("finding dependent {}", self.model.get_node(*d).label);
                    match Self::find_dependent(*d, by_layer, i + 1) {
                        Some((layer, node)) => {
                            connections.push((i, j, layer, node));
                            web_print!("found dependent in layer {}", layer);
                        }
                        None => {
                            web_print!("couldn't find dependent");
                        }
                    }
                }
            }
        }
        web_print!("CONNECTIONS {:?}", connections);
        for (from_layer, from_node, to_layer, to_node) in connections {
            if (to_layer - from_layer) == 1 {
                /*
                    The simple case
                */
                by_layer[from_layer][from_node]
                    .dependents
                    .push((to_layer, to_node));
                continue;
            }

            // add fake nodes
            let mut itr_layer = from_layer;
            let mut itr_node = from_node;
            for layer in (from_layer + 1)..to_layer {
                let end = by_layer[layer].len();
                self.model.nodes.push(Node::new_fake_node());
                by_layer[layer].push(LayoutNodeWrapper {
                    handle: self.model.nodes.len() - 1,
                    layers: vec![],
                    layer: layer,
                    dependents: vec![],
                });
                by_layer[itr_layer][itr_node].dependents.push((layer, end));
                itr_layer = layer;
                itr_node = end;
            }
            by_layer[itr_layer][itr_node]
                .dependents
                .push((to_layer, to_node));
        }
    }

    fn create_path_to(
        &self,
        mut start: VecF2,
        by_layer: &mut Vec<Vec<LayoutNodeWrapper>>,
        first_link: (usize, usize),
        to: NodeHandle,
        from: NodeHandle,
    ) -> Path {
        let mut path: Path = Path::new(to, from);
        let mut layer = first_link.0;
        let mut node = first_link.1;

        loop {
            let n: &LayoutNodeWrapper = &by_layer[layer][node];
            path.line_segments.push(Line {
                a: VecF2 {
                    x: start.x,
                    y: start.y,
                },
                b: VecF2 {
                    x: self.model.nodes[n.handle].position.x,
                    y: self.model.nodes[n.handle].position.y,
                },
                colour: 0x00000055,
            });
            start = VecF2 {
                x: self.model.nodes[n.handle].position.x,
                y: self.model.nodes[n.handle].position.y,
            };
            if self.model.nodes[n.handle].is_fake_node {
                if n.dependents.len() == 0 {
                    web_print!("LOLOLOL")
                }
                assert!(n.dependents.len() == 1);
                layer = n.dependents[0].0;
                node = n.dependents[0].1;
            } else {
                break;
            }
        }
        path
    }

    fn add_drawable_connections(&mut self, by_layer: &mut Vec<Vec<LayoutNodeWrapper>>) {
        let mut paths: Vec<(usize, usize, Path)> = vec![];
        // stupid 2 pass process because of borrow checker
        for i in 0..by_layer.len() {
            for j in 0..by_layer[i].len() {
                //let wrapper = &by_layer[i][j];
                if self.model.get_node(by_layer[i][j].handle).is_fake_node {
                    continue;
                }
                //assert!(by_layer[i][j].dependents.len() == model.get_node(by_layer[i][j].node.logical_node_handle).dependents.len());
                for k in 0..by_layer[i][j].dependents.len() {
                    let wrapper = &by_layer[i][j];
                    let logical = self.model.get_node(wrapper.handle);
                    let path = self.create_path_to(
                        self.model.nodes[wrapper.handle].position.clone(),
                        by_layer,
                        wrapper.dependents[k],
                        logical.dependents[k],
                        wrapper.handle,
                    );
                    paths.push((i, j, path));
                }
            }
        }
        for (i, j, path) in paths {
            let wrapper = &mut by_layer[i][j];
            self.model.edges.push(path);
            self.model.nodes[wrapper.handle]
                .edges
                .push(self.model.edges.len() - 1);
        }
    }

    fn recursively_add_layers(layers: &mut Vec<Vec<LayoutNodeWrapper>>, model: &Model) -> () {
        loop {
            let mut top_layer_copy: Vec<LayoutNodeWrapper> = vec![];
            let mut new_layer: Vec<LayoutNodeWrapper> = vec![];
            for wrapper in &layers[layers.len() - 1] {
                let mut deps_in_top = false;
                for d in &model.get_node(wrapper.handle).dependents {
                    let dep = *d;
                    /* any dependencies in the final layer? (they shouldn't be anywhere else) */
                    //Self::print_node(model, wrapper);
                    match Self::find_dependent(dep, layers, layers.len() - 1) {
                        None => {}
                        Some(_) => {
                            deps_in_top = true;
                            break;
                        }
                    };
                }
                if deps_in_top {
                    let mut cpy = wrapper.clone();
                    cpy.layer = layers.len();
                    new_layer.push(cpy);
                } else {
                    top_layer_copy.push(wrapper.clone());
                }
            }
            if new_layer.len() == 0 {
                return;
            }
            layers.pop();
            layers.push(top_layer_copy);
            layers.push(new_layer);
        }
    }

    fn find_highest_layer_and_sort(nodes: &mut HashMap<String, LayoutNodeWrapper>) -> usize {
        let mut highest_layer: usize = 0;
        for (_k, v) in nodes {
            //web_print!("key: {} val: {:?}", k, v.layers);
            v.layers.sort();
            v.layer = *v.layers.last().unwrap();
            if v.layer > highest_layer {
                highest_layer = v.layer;
            }
        }
        highest_layer
    }

    pub fn layout(&mut self) -> () {
        let root: NodeHandle = self.model.get_root_node();
        let node = self.model.get_node(root);
        let _label = node.label.as_str();
        //web_print!("root: {} num dependents: {}", label, node.dependents.len());
        // for d in &node.dependents {
        //     let n = self.model.get_node(*d);
        //     web_print!("{}", n.label);
        // }
        let mut nodes: HashMap<String, LayoutNodeWrapper> = HashMap::new();
        nodes.insert(
            node.label.clone(),
            LayoutNodeWrapper {
                layers: vec![0],
                handle: root,
                layer: 0,
                dependents: vec![],
            },
        );
        self.gather_nodes(node, &mut nodes, 1);
        let highest_layer = Self::find_highest_layer_and_sort(&mut nodes);
        let mut by_layer: Vec<Vec<LayoutNodeWrapper>> = vec![vec![]; highest_layer + 1];

        for (_k, v) in &mut nodes {
            by_layer[v.layer].push(v.clone());
        }

        Self::recursively_add_layers(&mut by_layer, &self.model);

        self.connect_nodes(&mut by_layer);

        // for i in 0..by_layer.len() {
        //     web_print!("layer {} size: {}", i, by_layer[i].len());
        //     for n in &by_layer[i] {
        //         let logical_node = self.model.get_node(n.node.logical_node_handle);
        //         web_print!("layer: {} node: {}", i, logical_node.label);
        //     }
        // }

        let mut column_start: VecF2 = VecF2 { x: 0.0, y: 0.0 };
        let column_width = 1000.0;
        let column_height = 100.0;
        for layer in &mut by_layer {
            let mut cursor = column_start.clone();
            for node in layer {
                self.model.nodes[node.handle].position = cursor.clone();
                cursor += VecF2 {
                    x: 0.0,
                    y: column_height,
                };
            }
            column_start += VecF2 {
                x: column_width,
                y: 0.0,
            }
        }

        // the layout is now finalized, add drawable node connections

        self.add_drawable_connections(&mut by_layer);
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
        let node = &self.model.nodes[handle];
        Circle {
            center: self.world_to_screen(&VecF2 {
                x: node.position.x,
                y: node.position.y,
            }),
            radius: node.radius * self.camera.zoom,
        }
    }

    pub fn check_bound_circle(&self, handle: usize, coord: VecF2) -> bool {
        let circ = self.get_bound_circle(handle);
        let dx = coord.x - circ.center.x;
        let dy = coord.y - circ.center.y;

        dx * dx + dy * dy <= circ.radius * circ.radius
    }

    // TODO smooth zooming
    fn center_camera(&mut self) {
        let first = &self.model.nodes.get(0).unwrap().position;
        let (x_min, x_max, y_min, y_max) = self.model.nodes.iter().skip(1).fold(
            (first.x, first.x, first.y, first.y),
            |(min_x, max_x, min_y, max_y), n| {
                let p = &n.position;
                (
                    min_x.min(p.x),
                    max_x.max(p.x),
                    min_y.min(p.y),
                    max_y.max(p.y),
                )
            },
        );

        let x_range = x_max - x_min;
        let y_range = y_max - y_min;

        let margin_scale = 0.6;

        if self.screen_w * margin_scale < x_range * self.camera.zoom {
            self.camera.zoom = (self.screen_w * margin_scale) / x_range;
        }
        if self.screen_h * margin_scale < y_range * self.camera.zoom {
            self.camera.zoom = (self.screen_h * margin_scale) / y_range;
        }
        self.camera.pos = VecF2 {
            x: (x_min + x_max) / 2.0,
            y: (y_min + y_max) / 2.0,
        };
    }

    pub fn highlight_node(&mut self, handle: Option<usize>) {
        for i in 0..self.model.nodes.len() {
            let (colour, radius, label_size, label_colour) = match handle {
                Some(h) if h == i => (
                    COLOURS_HIGHLIGHT.node_hover,
                    40.0,
                    20.0,
                    COLOURS_HIGHLIGHT.text,
                ),
                _ if matches!(self.model.nodes[i].bicone, Bicone::Upstream) => {
                    (COLOURS_HIGHLIGHT.node_upstream, 30.0, 10.0, COLOURS.text)
                }
                _ if matches!(self.model.nodes[i].bicone, Bicone::Downstream) => {
                    (COLOURS_HIGHLIGHT.node_downstream, 30.0, 10.0, COLOURS.text)
                }
                _ if matches!(self.model.nodes[i].bicone, Bicone::Center) => {
                    (COLOURS_HIGHLIGHT.node_hover, 30.0, 10.0, COLOURS.text)
                }
                _ => (COLOURS.node, 30.0, 10.0, COLOURS.text),
            };
            self.model.nodes[i].colour = colour;
            self.model.nodes[i].label_colour = label_colour;
            self.model.nodes[i].radius = radius;
            self.model.nodes[i].label_size = label_size;
        }
    }

    pub fn highlight_bicone(&mut self, handle: usize) {
        let (nodes_dowstream, edges_downstream) = self.get_dependencies_nodes_edges(handle);
        let (nodes_upstream, edges_upstream) = self.get_dependants_nodes_edges(handle);

        for i in 0..self.model.nodes.len() {
            let colour = if nodes_dowstream.contains(&i) {
                self.model.nodes[i].bicone = Bicone::Downstream;
                COLOURS_HIGHLIGHT.node_downstream
            } else if nodes_upstream.contains(&i) {
                self.model.nodes[i].bicone = Bicone::Upstream;
                COLOURS_HIGHLIGHT.node_upstream
            } else {
                self.model.nodes[i].bicone = Bicone::None;
                COLOURS.node
            };

            self.model.nodes[i].colour = colour;
        }

        for i in 0..self.model.edges.len() {
            let colour = if edges_downstream.contains(&i) {
                COLOURS_HIGHLIGHT.edge_downstream
            } else if edges_upstream.contains(&i) {
                COLOURS_HIGHLIGHT.edge_upstream
            } else {
                COLOURS.edge
            };

            self.set_edge_colour(i, colour);
        }

        self.model.nodes[handle].colour = COLOURS_HIGHLIGHT.node_hover;
        self.model.nodes[handle].bicone = Bicone::Center;
    }

    fn set_edge_colour(&mut self, handle: usize, colour: Colour) {
        for line in self.model.edges[handle].line_segments.iter_mut() {
            line.colour = colour;
        }
    }

    fn get_dependencies_nodes_edges(&mut self, handle: usize) -> (Vec<usize>, Vec<usize>) {
        let mut visited: HashSet<usize> = HashSet::new();
        let mut queue = vec![handle];
        let mut edges = vec![];
        let mut nodes = vec![];

        while let Some(h) = queue.pop() {
            if !visited.insert(h) {
                continue;
            }
            nodes.push(h);
            for edge_handle in self.model.nodes[h].edges.iter() {
                edges.push(*edge_handle);
                queue.push(self.model.edges[*edge_handle].to);
            }
        }

        (nodes, edges)
    }

    fn get_incoming_edges(&self, handle: usize) -> Vec<usize> {
        let mut out: Vec<usize> = Vec::new();
        for (i, edge) in self.model.edges.iter().enumerate() {
            if edge.to == handle {
                out.push(i);
            }
        }
        out
    }

    fn get_dependants_nodes_edges(&mut self, handle: usize) -> (Vec<usize>, Vec<usize>) {
        let mut visited: HashSet<usize> = HashSet::new();
        let mut queue = vec![handle];
        let mut edges = vec![];
        let mut nodes = vec![];

        while let Some(h) = queue.pop() {
            if !visited.insert(h) {
                continue;
            }
            nodes.push(h);
            let edge_handles = self.get_incoming_edges(h);
            for edge_handle in edge_handles.iter() {
                edges.push(*edge_handle);
                queue.push(self.model.edges[*edge_handle].from);
            }
        }

        (nodes, edges)
    }
}

use std::collections::HashMap;

use crate::drawing::DrawableNode;

use crate::js;
use crate::{data_types::*, model::*};

use dot_parser::ast::{Graph, ID};
use crate::drawing::*;
use crate::web_print;

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

/* 
    Wraps a DrawableNode, adds variables needed for the layout algorithm.
    The very last part of the layout function extracts the inner DrawableNode.
*/
#[derive(Clone)]
struct LayoutNodeWrapper {
    node: DrawableNode,

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
    dependents: Vec<(usize, usize)>
}

impl Scene {
    pub fn new(screen_w: i32, screen_h: i32, graph: &Graph<(ID<'_>, ID<'_>)>) -> Self {
        
        Self {
            camera: Camera {
                pos: VecF2 {
                    x: screen_w as f32 / 2.0,
                    y: screen_h as f32 / 2.0,
                },
                zoom: 1.0,
            },
            nodes: vec![],
            screen_w: screen_w as f32,
            screen_h: screen_h as f32,
            model: Model::new(graph),
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

    fn gather_nodes(&self, node: &Node, out: &mut HashMap<String, LayoutNodeWrapper>, on_layer: usize) -> () {
        if on_layer > 5 {
            return;
        }
        for h_node in &node.dependents {
            let n: &Node = self.model.get_node(*h_node);
            if let Some(val) = out.get_mut(&n.label) {
                val.layers.push(on_layer);
            }
            else {
                out.insert(n.label.clone(), LayoutNodeWrapper{
                    layers: vec![on_layer],
                    node: DrawableNode::new(*h_node),
                    layer: 0,
                    dependents: vec![]
                });
            }
            self.gather_nodes(n, out, on_layer + 1);
        }
    }

    fn find_dependent(dependent: LogicalNodeHandle, by_layer: &Vec<Vec<LayoutNodeWrapper>>, start_layer: usize) -> Option<(usize, usize)> {
        for i in start_layer..by_layer.len() {
            for j in 0..by_layer[i].len() {//wrapper in &by_layer[i] {
                let wrapper = &by_layer[i][j];
                if wrapper.node.logical_node_handle == dependent {
                    //web_print!("some");
                    return Some((i, j));
                }
            }
        }
        //web_print!("none");
        None
    }

    fn print_node(model: &Model, wrapper: &LayoutNodeWrapper) -> () {
        let inner = model.get_node(wrapper.node.logical_node_handle);
        let mut string = format!("{}", inner.label);
        string += " [ ";
        for node_h in &inner.dependents {
            let h = *node_h;
            string += format!("{}, ", model.get_node(h).label).as_str();
        }
        string += " ]";
        web_print!("{}", string);
    }

    fn connect_nodes(by_layer: &mut Vec<Vec<LayoutNodeWrapper>>, model: &Model) -> () {
        let mut connections: Vec<(usize, usize, usize, usize)> = Vec::new(); // (from_layer, from_node, to_layer, to_node)
        for i in 0..by_layer.len() {
            let layer = &by_layer[i];
            for j in 0..layer.len() {
                let wrapper = &layer[j];
                //web_print!("{} {:?}", model.get_node(wrapper.node.logical_node_handle).label, model.get_node(wrapper.node.logical_node_handle).dependents);
                Self::print_node(model, wrapper);
                for d in &model.get_node(wrapper.node.logical_node_handle).dependents {
                    web_print!("finding dependent {}", model.get_node(*d).label);               
                    match Self::find_dependent(*d, by_layer, i+1) {
                        Some((layer, node)) => {
                            connections.push(
                                (i, j, layer, node)
                            );
                            web_print!("found dependent in layer {}", layer);
                        },
                        None => { web_print!("couldn't find dependent"); }
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
                by_layer[from_layer][from_node].dependents.push((to_layer, to_node));
                continue;
            }
            
            // add fake nodes
            let mut itr_layer = from_layer;
            let mut itr_node = from_node;
            for layer in (from_layer + 1)..to_layer {
                let mut end = by_layer[layer].len();
                by_layer[layer].push(LayoutNodeWrapper { 
                    node: DrawableNode { 
                        is_fake_node: true, 
                        position: VecF2 { x: 0.0, y: 0.0 },
                        logical_node_handle: 0, 
                        edges: vec![],
                        colour: 0
                    },
                    layers: vec![],
                    layer: layer,
                    dependents: vec![]
                });
                by_layer[itr_layer][itr_node].dependents.push((
                    layer, end
                ));
                itr_layer = layer;
                itr_node = end;
            }
            by_layer[itr_layer][itr_node].dependents.push((
                to_layer, to_node
            ));
        }
    }

    fn create_path_to(mut start: VecF2, by_layer: &mut Vec<Vec<LayoutNodeWrapper>>, first_link: (usize, usize), to: LogicalNodeHandle, from : LogicalNodeHandle) -> Path {
        let mut path: Path = Path::new(to, from);
        let mut layer = first_link.0;
        let mut node = first_link.1;
        
        loop {
            let mut n: &LayoutNodeWrapper = &by_layer[layer][node];
            path.line_segments.push(Line { 
                a: VecF2 { x: start.x, y: start.y },
                b: VecF2 { x: n.node.position.x, y: n.node.position.y },
                colour: 0x000000FF 
            });
            start = VecF2 { x: n.node.position.x, y: n.node.position.y };
            if n.node.is_fake_node {
                if n.dependents.len() == 0 {
                    web_print!("LOLOLOL")
                }
                assert!(n.dependents.len() == 1);
                layer = n.dependents[0].0;
                node = n.dependents[0].1;
            }
            else {
                break;
            }
        }
        path
    }

    fn add_drawable_connections(model: &Model, by_layer: &mut Vec<Vec<LayoutNodeWrapper>>) {
        let mut paths: Vec<(usize, usize, Path)> = vec![];
        // stupid 2 pass process because of borrow checker
        for i in 0..by_layer.len() {
            for j in 0..by_layer[i].len() {
                //let wrapper = &by_layer[i][j];
                if by_layer[i][j].node.is_fake_node {
                    continue;
                }
                //assert!(by_layer[i][j].dependents.len() == model.get_node(by_layer[i][j].node.logical_node_handle).dependents.len());
                for k in 0..by_layer[i][j].dependents.len() {
                    let wrapper = &by_layer[i][j];
                    let logical = model.get_node(wrapper.node.logical_node_handle);
                    let path = Self::create_path_to(
                        wrapper.node.position.clone(), 
                        by_layer, 
                        wrapper.dependents[k], 
                        logical.dependents[k], 
                        wrapper.node.logical_node_handle);
                    paths.push((
                        i, j , path
                    ));
                }
            }
        }
        for (i, j, path) in paths {
            //web_print!("PATH");
            let mut wrapper = &mut by_layer[i][j];
            wrapper.node.edges.push(path);
        }
    }

    fn recursively_add_layers(layers: &mut Vec<Vec<LayoutNodeWrapper>>, model: &Model) -> () {
        loop {
            let mut top_layer_copy: Vec<LayoutNodeWrapper> = vec![];
            let mut new_layer: Vec<LayoutNodeWrapper> = vec![];
            for wrapper in &layers[layers.len() - 1] {
                let mut deps_in_top = false;
                for d in &model.get_node(wrapper.node.logical_node_handle).dependents {
                    let dep = *d;
                    /* any dependencies in the final layer? (they shouldn't be anywhere else) */
                    //Self::print_node(model, wrapper);
                    match Self::find_dependent(dep, layers, layers.len() - 1) {
                        None => {
                        }
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
                }
                else {
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
        for (k, v) in nodes {
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
        let b_test_mode = false;
        if b_test_mode {
            self.nodes = Self::layout_test(&mut self.model);
        }
        let root: LogicalNodeHandle = self.model.get_root_node();
        self.root;
        let node = self.model.get_node(root);
        let label = node.label.as_str();
        //web_print!("root: {} num dependents: {}", label, node.dependents.len());
        // for d in &node.dependents {
        //     let n = self.model.get_node(*d);
        //     web_print!("{}", n.label);
        // }
        let mut nodes: HashMap<String, LayoutNodeWrapper> = HashMap::new();
        nodes.insert(node.label.clone(), LayoutNodeWrapper{
            layers: vec![0],
            node: DrawableNode::new(root),
            layer: 0,
            dependents: vec![]
        });
        self.gather_nodes(node, &mut nodes, 1);
        let highest_layer = Self::find_highest_layer_and_sort(&mut nodes);
        let mut by_layer: Vec<Vec<LayoutNodeWrapper>> = vec![vec![]; highest_layer + 1];

        for (k, v) in &mut nodes {
            by_layer[v.layer].push(v.clone());
        }

        Self::recursively_add_layers(&mut by_layer, &self.model);

        Self::connect_nodes(&mut by_layer, &self.model);

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
                node.node.position = cursor.clone();
                cursor += VecF2 { x: 0.0, y: column_height };
            }
            column_start += VecF2 { x: column_width, y: 0.0 }
        }

        // the layout is now finalized, add drawable node connections 

        Self::add_drawable_connections(&self.model, &mut by_layer);

        self.nodes = by_layer
            .into_iter()
            .flatten()
            .map(|x| x.node)
            //.filter(|x| !x.is_fake_node)
            .collect();
    }

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
            //js::log_str(self.model.get_node(h).label.as_ptr());
            //web_print!("{}", self.model.get_node(h).label);
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
